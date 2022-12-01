use async_graphql::*;
use futures::{Stream, StreamExt};

use crate::{model::{room::{Room, RoomMember, RoomChatMsg, RoomMemberUpdate, RoomRemoteUpdate}, user::User}, store::DataStore, types::{color::{Color, ColorType}, token::Token, id::Id}, auth::{action::Action, authority::Authority, actor::Actor}, stream::StreamController};

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

    async fn room_member<'ctx>(&self, ctx: &Context<'ctx>, room: Id<Room>, user: Id<User>) -> Result<Option<RoomMember>> {
        let room_store = ctx.data::<DataStore<Room>>()?;
        let room = room_store.get(&room)?.ok_or::<async_graphql::Error>("Room not found".into())?;
        let member = room.get_member(&user).cloned();
        if let Some(member) = &member {
            ctx.require_act(RoomAction::GetMember(&member.user), &room)?;
        }
        Ok(member)
    }

    async fn room_colors<'ctx>(&self, ctx: &Context<'ctx>, room: Id<Room>) -> Result<Vec<Color>> {
        let room_store = ctx.data::<DataStore<Room>>()?;
        let room = room_store.get(&room)?.ok_or::<async_graphql::Error>("Room not found".into())?;
        let colors = room.get_available_colors();
        Ok(colors.iter().map(|c| Color::from(*c)).collect())
    }

    async fn room_color_is_available<'ctx>(&self, ctx: &Context<'ctx>, room: Id<Room>, color: ColorType) -> Result<bool> {
        let room_store = ctx.data::<DataStore<Room>>()?;
        let room = room_store.get(&room)?.ok_or::<async_graphql::Error>("Room not found".into())?;
        let avail = room.is_color_available(color);
        Ok(avail)
    }
}

#[Object]
impl RoomMutation {
    async fn create_room<'ctx>(&self, ctx: &Context<'ctx>, name: Option<String>, owner_color: Option<ColorType>) -> Result<Room> {
        let user = ctx.actor_user()?;
        let room_store = ctx.data::<DataStore<Room>>()?;

        let mut room = Room::new();

        let name = name.unwrap_or_else(|| {
            if user.name.len() > 0 {
                format!("{}'s room", user.name)
            } else {
                "My Room".to_string()
            }
        });

        room.name = Some(name);
        room.init_owner(&user, owner_color)?;

        room_store.insert(room.clone());
        Ok(room)
    }

    async fn send_chat_message<'ctx>(&self, ctx: &Context<'ctx>, room: Id<Room>, msg: String) -> Result<RoomChatMsg> {
        let room_store = ctx.data::<DataStore<Room>>()?;
        let Some(mut room) = room_store.get(&room)? else { return Err("Room not found".into()) };

        let actor = ctx.require_act(RoomAction::SendChat, &room)?;
        let Actor::User(user) = actor else { return Err("Illegal actor".into()) };

        let room_chat_msg = room.create_chat_msg(&user, msg)?;

        room.add_chat_msg(room_chat_msg.clone());

        let stream_ctl = ctx.data::<StreamController>()?;
        stream_ctl.publish(room_chat_msg.clone());

        Ok(room_chat_msg)
    }

    async fn pass_room_remote<'ctx>(&self, ctx: &Context<'ctx>, room: Id<Room>, to_user: Id<User>) -> Result<Room> {
        let room_store = ctx.data::<DataStore<Room>>()?;
        let Some(mut room) = room_store.get(&room)? else { return Err("Room not found".into()) };

        let actor = ctx.require_act(RoomAction::PassRemote(&to_user), &room)?;
        let Actor::User(user) = actor else { return Err("Illegal actor".into()) };
        if !room.is_member(&user.id) { return Err("You are not a member of that room".into()) }
        if !room.is_member(&to_user) { return Err("That user is not a member of the room".into()) }
    
        let from = room.remote.clone();

        room.pass_remote(&user.id, to_user.clone())?;

        let stream_ctl = ctx.data::<StreamController>()?;
        let room_remote_update = RoomRemoteUpdate::new(room.id.clone(), from, to_user);
        stream_ctl.publish(room_remote_update);

        Ok(room.clone())
    }

    async fn join_room<'ctx>(&self, ctx: &Context<'ctx>, invite_token: String, color: Option<ColorType>) -> Result<Room> {
        let room_store = ctx.data::<DataStore<Room>>()?;
        let Some(room_id) = room_store.data.lock().unwrap().values().find(|r| r.check_invite(&invite_token)).map(|r| r.id.clone()) else { return Err("Room not found".into()) };
        let Some(mut room) = room_store.get(&room_id)? else { return Err("Room not found".into()) };

        if !room.check_invite(&invite_token) { return Err("Invalid invite".into()) }
        if room.members.len() >= crate::model::room::MAX_ROOM_SIZE { return Err("Room is full".into()) }
        let actor = ctx.require_act(RoomAction::Join(&invite_token), &room)?;
        let Actor::User(user) = actor else { return Err("Illegal actor".into()) };
        let room_member = room.join(&user, color)?;

        let stream_ctl = ctx.data::<StreamController>()?;
        let room_member_update = RoomMemberUpdate::new_join(room.id.clone(), room_member);
        stream_ctl.publish(room_member_update);

        Ok(room.clone())
    }

    async fn leave_room<'ctx>(&self, ctx: &Context<'ctx>, room: Id<Room>) -> Result<User> {
        let room_store = ctx.data::<DataStore<Room>>()?;
        let Some(mut room) = room_store.get(&room)? else { return Err("Room not found".into()) };

        if room.members.len() >= crate::model::room::MAX_ROOM_SIZE { return Err("Room is full".into()) }
        let actor = ctx.require_act(RoomAction::Leave, &room)?;
        let Actor::User(user) = actor else { return Err("Illegal actor".into()) };
        let room_member = room.leave(&user)?;

        let stream_ctl = ctx.data::<StreamController>()?;
        let room_member_update = RoomMemberUpdate::new_leave(room.id.clone(), room_member);
        stream_ctl.publish(room_member_update);

        Ok(user)
    }

    async fn create_room_invite<'ctx>(&self, ctx: &Context<'ctx>, room: Id<Room>) -> Result<Token> {
        let room_store = ctx.data::<DataStore<Room>>()?;
        let Some(mut room) = room_store.get(&room)? else { return Err("Room not found".into()) };

        let actor = ctx.require_act(RoomAction::CreateInvite, &room)?;
        let Actor::User(user) = actor else { return Err("Illegal actor".into()) };
        let room_invite = room.create_invite(user.id.clone())?;

        Ok(room_invite.token.clone())
    }

    async fn revoke_room_invite<'ctx>(&self, ctx: &Context<'ctx>, room: Id<Room>, invite: String) -> Result<Room> {
        let room_store = ctx.data::<DataStore<Room>>()?;
        let Some(mut room) = room_store.get(&room)? else { return Err("Room not found".into()) };

        ctx.require_act(RoomAction::RevokeInvite(&invite), &room)?;
        room.revoke_invite(&invite)?;

        Ok(room.clone())
    }
}

#[Subscription]
impl RoomSubscription {
    async fn room_chat<'ctx>(&'ctx self, ctx: &Context<'ctx>, id: Id<Room>) -> Result<impl Stream<Item = RoomChatMsg> + 'ctx>{
        let room_store = ctx.data::<DataStore<Room>>()?;
        let user_store = ctx.data::<DataStore<User>>()?;
        let Some(mut room) = room_store.get(&id)? else { return Err("Room not found".into()) };

        let actor = ctx.require_act(RoomAction::SubscribeChat, &room)?;
        let Actor::User(user) = actor else { return Err("Illegal actor".into()) };

        let id = room.id.clone(); // room id
        let stream_ctl = ctx.data::<StreamController>()?;

        let Some(room_member) = room.get_member_mut(&user.id) else { return Err("You are not a member of that room".into()) };
        if room_member.connection.connected_chat {
            return Err("You are already subscribed to room_chat for this room".into())
        }

        room_member.connection.connected_chat = true;

        Ok(
            stream_ctl.subscribe::<RoomChatMsg, _>(&user.id, &room.id, user_store.clone(), room_store.clone(), |data| {
                // on disconnect
                let Ok(room) = &mut data.room_store.get(&data.room) else { return /* suppress this shit ig? */ };
                let Some(room) = room else { return };
                let Some(room_member) = room.get_member_mut(&data.user) else { return };
                room_member.connection.connected_chat = false;
                if !room_member.is_connected() {
                    room_member.handle_disconnect(&data.room, stream_ctl);
                }
            }).filter(move |event| {
                let res = event.room == id;
                async move { res }
            })
        )
    }

    async fn room_members<'ctx>(&'ctx self, ctx: &Context<'ctx>, id: Id<Room>) -> Result<impl Stream<Item = RoomMemberUpdate> + 'ctx> {
        let room_store = ctx.data::<DataStore<Room>>()?;
        let user_store = ctx.data::<DataStore<User>>()?;

        let Some(mut room) = room_store.get(&id)? else { return Err("Room not found".into()) };

        let actor = ctx.require_act(RoomAction::SubscribeMembers, &room)?;
        let Actor::User(user) = actor else { return Err("Illegal actor".into()) };

        let stream_ctl = ctx.data::<StreamController>()?;

        let Some(room_member) = room.get_member_mut(&user.id) else { return Err("You are not a member of that room".into()) };
        if room_member.connection.connected_members {
            return Err("You are already subscribed to room_members for this room".into())
        }
        room_member.connection.connected_members = true;

        let id = room.id.clone();
        Ok(
            stream_ctl.subscribe::<RoomMemberUpdate, _>(&user.id, &room.id, user_store.clone(), room_store.clone(), |data| {
                // on disconnect
                let Ok(room) = &mut data.room_store.get(&data.room) else { return /* suppress this shit ig? */ };
                let Some(room) = room else { return };
                let Some(room_member) = room.get_member_mut(&data.user) else { return };
                room_member.connection.connected_members = false;
                if !room_member.is_connected() {
                    room_member.handle_disconnect(&data.room, stream_ctl);
                }
            }).filter(move |event| {
                let res = event.room == id;
                async move { res }
            })
        )
    }

    async fn room_remote<'ctx>(&'ctx self, ctx: &Context<'ctx>, id: Id<Room>) -> Result<impl Stream<Item = RoomRemoteUpdate> + 'ctx> {
        let room_store = ctx.data::<DataStore<Room>>()?;
        let user_store = ctx.data::<DataStore<User>>()?;

        let Some(mut room) = room_store.get(&id)? else { return Err("Room not found".into()) };

        let actor = ctx.require_act(RoomAction::SubscribeRemote, &room)?;
        let Actor::User(user) = actor else { return Err("Illegal actor".into()) };

        let stream_ctl = ctx.data::<StreamController>()?;

        let Some(room_member) = room.get_member_mut(&user.id) else { return Err("You are not a member of that room".into()) };
        if room_member.connection.connected_remote {
            return Err("You are already subscribed to room_members for this room".into())
        }
        room_member.connection.connected_chat = true;

        let id = room.id.clone();
        Ok(
            stream_ctl.subscribe::<RoomRemoteUpdate, _>(&user.id, &room.id, user_store.clone(), room_store.clone(), |data| {
                // on disconnect
                let Ok(room) = &mut data.room_store.get(&data.room) else { return /* suppress this shit ig? */ };
                let Some(room) = room else { return };
                let Some(room_member) = room.get_member_mut(&data.user) else { return };
                room_member.connection.connected_remote = false;
                if !room_member.is_connected() {
                    room_member.handle_disconnect(&data.room, stream_ctl);
                }
            }).filter(move |event| {
                let res = event.room == id;
                async move { res }
            })
        )
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
    SubscribeChat,
    SubscribeMembers,
    SubscribeRemote,
}

impl<'a> Action<Room> for RoomAction<'a> {
    fn can_act(&self, actor: &crate::auth::actor::Actor, room: &Room) -> bool {
        match actor {
            Actor::None => false,
            Actor::Internal => true,
            Actor::User(user) => {
                match self {
                    Self::Get | Self::SendChat | Self::SubscribeChat | Self::SubscribeMembers | Self::SubscribeRemote | Self::Leave => {
                        room.is_member(&user.id)
                    },
                    Self::GetMember(id) => {
                        room.is_member(&user.id) && room.is_member(id)
                    },
                    Self::PassRemote(to_user_id) => {
                        room.can_pass_remote(&user.id, to_user_id)
                    }, 
                    Self::Join(token) => {
                        room.check_invite(token) && room.members.len() < crate::model::room::MAX_ROOM_SIZE
                    },
                    Self::CreateInvite => {
                        room.is_member(&user.id) && !room.invites.iter().any(|i| i.inviter == user.id && i.token.is_valid())
                    },
                    Self::RevokeInvite(token) => {
                        let invite_exists = room.invites.iter().any(|i| i.token.check(&token));
                        let is_member_or_owner = room.is_member(&user.id) || room.is_owner(&user.id);
                        let is_owner_or_created_invite = room.is_owner(&user.id) || room.invites.iter().any(|i| i.inviter == user.id);
                        
                        invite_exists && is_member_or_owner && is_owner_or_created_invite
                    }
                }
            }
        }
    }
}

