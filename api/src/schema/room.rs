use async_graphql::*;
use futures::Stream;

use crate::{
    auth::{action::Action, actor::Actor, authority::Authority},
    model::{room::Room, user::User},
    store::DataStore,
    stream::{StreamControl, Subscriber},
    types::{color::ColorType, id::Id, token::Token},
};

#[derive(Default)]
pub struct RoomQuery;

#[derive(Default)]
pub struct RoomMutation;

#[derive(Default)]
pub struct RoomSubscription;

#[Object]
impl RoomQuery {
    async fn room<'ctx>(&self, ctx: &Context<'ctx>, room: Id<Room>) -> Result<Option<Room>> {
        let room_store = ctx.data::<DataStore<Room>>()?;
        let room = room_store.get(&room)?;
        if let Some(room) = &room {
            ctx.require_act(RoomAction::Get, &*room)?;
        }
        Ok(room.as_deref().cloned())
    }
}

#[Object]
impl RoomMutation {
    async fn create_room<'ctx>(&self, ctx: &Context<'ctx>, name: Option<String>) -> Result<Room> {
        let user = ctx.user()?;
        let room_store = ctx.data::<DataStore<Room>>()?;

        if Room::any_room(room_store, |r| r.is_owner(&user.id)) {
            return Err("You have already created a room".into());
        }

        let room_name = name.unwrap_or_else(|| {
            if user.name.len() > 0 {
                format!("{}'s Room", user.name)
            } else {
                "My Room".to_string()
            }
        });

        let mut room = Room::new(room_name);

        room.init_owner(&user);

        room_store.insert(room.clone());
        Ok(room)
    }

    async fn send_chat_message<'ctx>(&self, ctx: &Context<'ctx>, msg: String) -> Result<Room> {
        let user = ctx.user()?;
        let mut room = ctx.room()?;

        ctx.require_act(RoomAction::SendChat, &room)?;

        let room_chat_msg = room.create_chat_msg(&user, msg)?;
        room.add_chat_msg(room_chat_msg);

        let stream_ctl = ctx.data::<StreamControl<User, Room>>()?;
        stream_ctl.publish(room.clone());

        Ok(room.clone())
    }

    async fn pass_room_remote<'ctx>(&self, ctx: &Context<'ctx>, to_user: Id<User>) -> Result<Room> {
        let user = ctx.user()?;
        let mut room = ctx.room()?;

        ctx.require_act(RoomAction::PassRemote(&to_user), &room)?;

        if !room.is_member(&user.id) {
            return Err("You are not a member of that room".into());
        }
        if !room.is_member(&to_user) {
            return Err("That user is not a member of the room".into());
        }

        room.pass_remote(&user.id, to_user.clone())?;

        let stream_ctl = ctx.data::<StreamControl<User, Room>>()?;
        stream_ctl.publish(room.clone());

        Ok(room.clone())
    }

    async fn join_room<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        invite_token: Option<String>,
        color: Option<ColorType>,
    ) -> Result<Room> {
        let room_store = ctx.data::<DataStore<Room>>()?;

        let user = ctx.user()?;

        if user.name.len() == 0 {
            return Err("You must set a name before joining a room".into());
        }

        if Room::any_room(room_store, |r| r.is_member(&user.id)) {
            return Err("You are already a member of another room".into());
        }

        let room_id = if let Some(invite_token) = invite_token {
            Room::find_room(room_store, |r| r.check_invite(&invite_token))
                .map(|r| r.id.clone())
                .ok_or::<async_graphql::Error>("Room not found".into())?
        } else {
            Room::find_room(room_store, |r| r.is_owner(&user.id))
                .map(|r| r.id.clone())
                .ok_or::<async_graphql::Error>("You are not the owner of any room".into())?
        };

        let Some(mut room) = room_store.get(&room_id)? else { return Err("Room not found".into()) };

        if room.members.len() >= crate::model::room::MAX_ROOM_SIZE {
            return Err("Room is full".into());
        };
        room.join(&user, color)?;

        let stream_ctl = ctx.data::<StreamControl<User, Room>>()?;
        stream_ctl.publish(room.clone());

        Ok(room.clone())
    }

    async fn leave_room<'ctx>(&self, ctx: &Context<'ctx>) -> Result<User> {
        let user = ctx.user()?;
        let mut room = ctx.room()?;

        if room.is_owner(&user.id) {
            return Err("You cannot leave a room you own".into());
        }

        ctx.require_act(RoomAction::Leave, &room)?;
        room.leave(&user.id)?;

        let stream_ctl = ctx.data::<StreamControl<User, Room>>()?;
        stream_ctl.disconnect(&user.id);
        stream_ctl.publish(room.clone());

        Ok(user.clone())
    }

    async fn create_room_invite<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Token> {
        let user = ctx.user()?;
        let mut room = ctx.room()?;
        ctx.require_act(RoomAction::CreateInvite, &room)?;

        let room_invite = room.create_invite(user.id.clone())?;

        let stream_ctl = ctx.data::<StreamControl<User, Room>>()?;
        stream_ctl.publish(room.clone());

        Ok(room_invite.token.clone())
    }

    async fn revoke_room_invite<'ctx>(&self, ctx: &Context<'ctx>, invite: String) -> Result<Room> {
        let mut room = ctx.room()?;
        ctx.require_act(RoomAction::RevokeInvite(&invite), &room)?;

        room.revoke_invite(&invite)?;

        let stream_ctl = ctx.data::<StreamControl<User, Room>>()?;
        stream_ctl.publish(room.clone());

        Ok(room.clone())
    }

    async fn kick_member<'ctx>(&self, ctx: &Context<'ctx>, member: Id<User>) -> Result<Room> {
        let mut room = ctx.room()?;

        ctx.require_act(RoomAction::KickMember(&member), &room)?;
        if room.is_owner(&member) {
            return Err("You cannot kick the owner of the room".into());
        }

        let room_member = room.leave(&member)?;

        let stream_ctl = ctx.data::<StreamControl<User, Room>>()?;

        stream_ctl.disconnect(&room_member.user);
        stream_ctl.publish(room.clone());

        Ok(room.clone())
    }

    async fn set_typing_status<'ctx>(&self, ctx: &Context<'ctx>, typing: bool) -> Result<Room> {
        let user = ctx.user()?;
        let mut room = ctx.room()?;
        ctx.require_act(RoomAction::SetTypingStatus, &room)?;

        let Some(member) = room.get_member_mut(&user.id) else { return Err("You are not in this room".into()) };

        member.typing = typing;

        let stream_ctl = ctx.data::<StreamControl<User, Room>>()?;
        stream_ctl.publish(room.clone());

        Ok(room.clone())
    }
}

#[Subscription]
impl RoomSubscription {
    async fn room<'ctx>(
        &'ctx self,
        ctx: &Context<'ctx>,
    ) -> Result<impl Stream<Item = Room> + 'ctx> {
        let room_store = ctx.data::<DataStore<Room>>()?;
        let mut room = ctx.room()?;

        let user = ctx
            .require_act(RoomAction::SubscribeChat, &room)?
            .user(ctx)?;

        let room_id = room.id.clone(); // room id
        let stream_ctl = ctx.data::<StreamControl<User, Room>>()?;

        let Some(room_member) = room.get_member_mut(&user.id) else { return Err("You are not a member of this room".into()) };
        if room_member.connected {
            return Err("You are already subscribed to this room".into());
        }

        room_member.connected = true;

        Ok(stream_ctl.subscribe(UserRoomSubscriber::new(
            user.id.clone(),
            room_id,
            room_store.clone(),
            stream_ctl.clone(),
        )))
    }
}

pub struct UserRoomSubscriber {
    user: Id<User>,
    room: Id<Room>,
    room_store: DataStore<Room>,
    stream_ctl: StreamControl<User, Room>,
}

impl UserRoomSubscriber {
    pub fn new(
        user: Id<User>,
        room: Id<Room>,
        room_store: DataStore<Room>,
        stream_ctl: StreamControl<User, Room>,
    ) -> Self {
        Self {
            user,
            room,
            room_store,
            stream_ctl,
        }
    }
}

impl Subscriber<User, Room> for UserRoomSubscriber {
    fn subscriber_id(&self) -> &Id<User> {
        &self.user
    }

    fn topic_id(&self) -> &Id<Room> {
        &self.room
    }

    fn on_disconnect(&mut self) {
        let Ok(Some(mut room)) = self.room_store.get(&self.room) else { return };
        let Some(room_member) = room.get_member_mut(&self.user) else { return };
        room_member.connected = false;
        room_member.typing = false;
        if let Some(remote) = room.remote.as_ref() {
            if remote == &self.user {
                room.remote = room.owner.clone();
            }
        }
        self.stream_ctl.publish(room.clone())
    }

    fn map_msg(&self, msg: Room) -> Option<Room> {
        Some(msg)
    }
}

#[derive(Debug, Clone)]
pub enum RoomAction<'a> {
    Get,
    GetMember(&'a Id<User>),
    SendChat,
    PassRemote(&'a Id<User>),
    Join(&'a str), // string: invite token
    Leave,
    CreateInvite,
    RevokeInvite(&'a str), // string: invite token
    KickMember(&'a Id<User>),
    SetTypingStatus,
    SubscribeChat,
    SubscribeMembers,
    SubscribeRemote,
}

impl<'a> Action<Room> for RoomAction<'a> {
    fn can_act(&self, actor: &crate::auth::actor::Actor, room: &Room) -> bool {
        match actor {
            Actor::None => false,
            Actor::Internal => true,
            Actor::User(user_id) => match self {
                Self::Get
                | Self::SendChat
                | Self::SubscribeChat
                | Self::SubscribeMembers
                | Self::SubscribeRemote
                | Self::Leave => room.is_member(user_id),
                Self::GetMember(id) => room.is_member(user_id) && room.is_member(id),
                Self::PassRemote(to_user_id) => room.can_pass_remote(user_id, to_user_id),
                Self::Join(token) => {
                    room.check_invite(token)
                        && room.members.len() < crate::model::room::MAX_ROOM_SIZE
                }
                Self::CreateInvite => {
                    room.is_member(user_id)
                    /*&& !room
                    .invites
                    .iter()
                    .any(|i| &i.inviter == user_id && i.token.is_valid())*/
                }
                Self::RevokeInvite(token) => {
                    let invite_exists = room.invites.iter().any(|i| i.token.check(&token));
                    let is_member_or_owner = room.is_member(user_id) || room.is_owner(user_id);
                    let is_owner_or_created_invite = room.is_owner(user_id)
                        || room
                            .invites
                            .iter()
                            .any(|i| &i.inviter == user_id && i.token.check(&token));

                    invite_exists && is_member_or_owner && is_owner_or_created_invite
                }
                Self::KickMember(id) => room.is_owner(user_id) && room.is_member(id),
                Self::SetTypingStatus => {
                    room.is_member(user_id)
                        && room
                            .get_member(user_id)
                            .map(|m| m.connected)
                            .unwrap_or(false)
                }
            },
        }
    }
}
