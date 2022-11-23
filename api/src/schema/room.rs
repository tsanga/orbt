use async_graphql::*;
use futures::{Stream, StreamExt};

use crate::{model::{room::{Room, RoomMember, RoomChatMsg, RoomMemberUpdate, RoomRemoteUpdate}, user::User}, store::DataStore, types::{color::{Color, ColorType}, time::Time, token::Token}, auth::{action::Action, authority::Authority, actor::Actor}, stream::OrbtStreamBroker};

#[derive(Default)]
pub struct RoomQuery;

#[derive(Default)]
pub struct RoomMutation;

#[derive(Default)]
pub struct RoomSubscription;

#[Object]
impl RoomQuery {
    async fn get<'ctx>(&self, ctx: &Context<'ctx>, id: u32) -> Result<Option<Room>> {
        let store = ctx.data::<DataStore>()?.room_store();
        let room_store = store.read().unwrap();
        let room = room_store.get_room_by_id(id);
        if let Some(room) = &room {
            ctx.require_act(RoomAction::Get, &room)?;
        }
        Ok(room)
    }

    async fn get_member<'ctx>(&self, ctx: &Context<'ctx>, room_id: u32, user: u32) -> Result<Option<RoomMember>> {
        let store = ctx.data::<DataStore>()?.room_store();
        let room_store = store.read().unwrap();
        let room = room_store.get_room_by_id(room_id).ok_or::<async_graphql::Error>("Room not found".into())?;
        let member = room.get_member(user).cloned();
        if let Some(member) = &member {
            ctx.require_act(RoomAction::GetMember(member.user), &room)?;
        }
        Ok(member)
    }

    async fn get_available_colors<'ctx>(&self, ctx: &Context<'ctx>, room_id: u32) -> Result<Vec<Color>> {
        let store = ctx.data::<DataStore>()?.room_store();
        let room_store = store.read().unwrap();
        let room = room_store.get_room_by_id(room_id).ok_or::<async_graphql::Error>("Room not found".into())?;
        let colors = room.get_available_colors();
        Ok(colors.iter().map(|c| Color::from(*c)).collect())
    }

    async fn is_color_available<'ctx>(&self, ctx: &Context<'ctx>, room_id: u32, color: ColorType) -> Result<bool> {
        let store = ctx.data::<DataStore>()?.room_store();
        let room_store = store.read().unwrap();
        let room = room_store.get_room_by_id(room_id).ok_or::<async_graphql::Error>("Room not found".into())?;
        let avail = room.is_color_available(color);
        Ok(avail)
    }
}

#[Object]
impl RoomMutation {
    async fn create<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Room> {
        let store = ctx.data::<DataStore>()?.room_store();
        let mut room_store = store.write().unwrap();
        let room = room_store.new_room()?;
        //ctx.require_act(RoomAction::Create, &room)?;
        Ok(room)
    }

    async fn init<'ctx>(&self, ctx: &Context<'ctx>, room: u32, owner: u32, token: String, color: Option<ColorType>) -> Result<Room> {
        let store = ctx.data::<DataStore>()?;

        let room_store_lock = store.room_store();
        let room_store = room_store_lock.write().unwrap();

        let user_store_lock = store.user_store();
        let user_store = user_store_lock.read().unwrap();

        let mut room = room_store.get_room_by_id(room).ok_or::<async_graphql::Error>("Room not found".into())?;
        if room.is_init() {
            return Err("Room already initialized".into());
        }

        if !room.create_token.check(token) {
            return Err("Invalid create token".into());
        }
        
        let user = user_store.get_user_by_id(owner).ok_or::<async_graphql::Error>("User not found".into())?;
        
        room.init_owner(&user, color)?;
        
        room_store.save(room.clone());

        Ok(room)
    }

    async fn send_chat_msg<'ctx>(&self, ctx: &Context<'ctx>, room_id: u32, msg: String) -> Result<RoomChatMsg> {
        let store = ctx.data::<DataStore>()?;
        let room_store_lock = store.room_store();
        let room_store = room_store_lock.write().unwrap();
        let Some(mut room) = room_store.get_room_by_id(room_id) else { return Err("Room not found".into()) };

        let actor = ctx.require_act(RoomAction::SendChat, &room)?;
        let Actor::User(user) = actor else { return Err("Illegal actor".into()) };

        let room_chat_msg = room.create_chat_msg(&user, msg)?;

        room.add_chat_msg(room_chat_msg.clone());
        room_store.save(room);

        OrbtStreamBroker::publish(room_chat_msg.clone());
        Ok(room_chat_msg)
    }

    async fn pass_remote<'ctx>(&self, ctx: &Context<'ctx>, room_id: u32, to_user: u32) -> Result<Room> {
        let store = ctx.data::<DataStore>()?;
        let room_store_lock = store.room_store();
        let room_store = room_store_lock.write().unwrap();
        let Some(mut room) = room_store.get_room_by_id(room_id) else { return Err("Room not found".into()) };

        let actor = ctx.require_act(RoomAction::PassRemote(to_user), &room)?;
        let Actor::User(user) = actor else { return Err("Illegal actor".into()) };
        if !room.is_member(user.id) { return Err("You are not a member of that room".into()) }
        if !room.is_member(to_user) { return Err("That user is not a member of the room".into()) }
        let from = room.remote;
        room.pass_remote(user.id, to_user)?;
        room_store.save(room.clone());

        let room_remote_update = RoomRemoteUpdate::new(room.id, from, to_user);
        OrbtStreamBroker::publish(room_remote_update);

        Ok(room)
    }

    async fn join<'ctx>(&self, ctx: &Context<'ctx>, room_id: u32, invite_token: String, color: Option<ColorType>) -> Result<Room> {
        let store = ctx.data::<DataStore>()?;
        let room_store_lock = store.room_store();
        let room_store = room_store_lock.write().unwrap();
        let Some(mut room) = room_store.get_room_by_id(room_id) else { return Err("Room not found".into()) };

        if !room.check_invite(&invite_token) { return Err("Invalid invite".into()) }
        if room.members.len() >= crate::model::room::MAX_ROOM_SIZE { return Err("Room is full".into()) }
        let actor = ctx.require_act(RoomAction::Join(invite_token.clone()), &room)?;
        let Actor::User(user) = actor else { return Err("Illegal actor".into()) };
        let room_member = room.join(&user, color)?;
        room_store.save(room.clone());

        let room_member_update = RoomMemberUpdate::new_join(room.id, room_member, user);
        OrbtStreamBroker::publish(room_member_update);

        Ok(room)
    }

    async fn leave<'ctx>(&self, ctx: &Context<'ctx>, room_id: u32) -> Result<User> {
        let store = ctx.data::<DataStore>()?;
        let room_store_lock = store.room_store();
        let room_store = room_store_lock.write().unwrap();
        let Some(mut room) = room_store.get_room_by_id(room_id) else { return Err("Room not found".into()) };

        if room.members.len() >= crate::model::room::MAX_ROOM_SIZE { return Err("Room is full".into()) }
        let actor = ctx.require_act(RoomAction::Leave, &room)?;
        let Actor::User(user) = actor else { return Err("Illegal actor".into()) };
        let room_member = room.leave(&user)?;
        room_store.save(room.clone());

        let room_member_update = RoomMemberUpdate::new_leave(room.id, room_member, user.clone());
        OrbtStreamBroker::publish(room_member_update);

        Ok(user)
    }

    async fn create_invite<'ctx>(&self, ctx: &Context<'ctx>, room_id: u32) -> Result<Token> {
        let store = ctx.data::<DataStore>()?;
        let room_store_lock = store.room_store();
        let room_store = room_store_lock.write().unwrap();
        let Some(mut room) = room_store.get_room_by_id(room_id) else { return Err("Room not found".into()) };

        let actor = ctx.require_act(RoomAction::CreateInvite, &room)?;
        let Actor::User(user) = actor else { return Err("Illegal actor".into()) };
        let room_invite = room.create_invite(user.id)?;
        room_store.save(room);
        Ok(room_invite.token.clone())
    }

    async fn revoke_invite<'ctx>(&self, ctx: &Context<'ctx>, room_id: u32, invite: String) -> Result<Room> {
        let store = ctx.data::<DataStore>()?;
        let room_store_lock = store.room_store();
        let room_store = room_store_lock.write().unwrap();
        let Some(mut room) = room_store.get_room_by_id(room_id) else { return Err("Room not found".into()) };

        ctx.require_act(RoomAction::RevokeInvite(invite.clone()), &room)?;
        room.revoke_invite(&invite)?;
        room_store.save(room.clone());

        Ok(room)
    }
}

#[Subscription]
impl RoomSubscription {
    async fn room_chat<'ctx>(&self, ctx: &Context<'ctx>, room_id: u32) -> Result<impl Stream<Item = RoomChatMsg>> {
        let store = ctx.data::<DataStore>()?;
        let room_store_lock = store.room_store();
        let room_store = room_store_lock.read().unwrap();
        let Some(room) = room_store.get_room_by_id(room_id) else { return Err("Room not found".into()) };
        ctx.require_act(RoomAction::SubscribeChat, &room)?;
        Ok(OrbtStreamBroker::<RoomChatMsg>::subscribe().filter(move |event| {
            let res = event.id == room_id;
            async move { res }
        }))
    }

    async fn room_members<'ctx>(&self, ctx: &Context<'ctx>, room_id: u32) -> Result<impl Stream<Item = RoomMemberUpdate>> {
        let store = ctx.data::<DataStore>()?;
        let room_store_lock = store.room_store();
        let room_store = room_store_lock.read().unwrap();
        let Some(room) = room_store.get_room_by_id(room_id) else { return Err("Room not found".into()) };
        ctx.require_act(RoomAction::SubscribeMembers, &room)?;
        Ok(OrbtStreamBroker::<RoomMemberUpdate>::subscribe().filter(move |event| {
            let res = event.room == room_id;
            async move { res }
        }))
    }

    async fn room_remote<'ctx>(&self, ctx: &Context<'ctx>, room_id: u32) -> Result<impl Stream<Item = RoomRemoteUpdate>> {
        let store = ctx.data::<DataStore>()?;
        let room_store_lock = store.room_store();
        let room_store = room_store_lock.read().unwrap();
        let Some(room) = room_store.get_room_by_id(room_id) else { return Err("Room not found".into()) };
        ctx.require_act(RoomAction::SubscribeRemote, &room)?;
        Ok(OrbtStreamBroker::<RoomRemoteUpdate>::subscribe().filter(move |event| {
            let res = event.room == room_id;
            async move { res }
        }))
    }
}

#[derive(Debug, Clone)]
pub enum RoomAction {
    Get,
    GetMember(u32), // u32: member user id
    //Create,
    SendChat,
    PassRemote(u32), // u32: to user
    Join(String), // string: invite token
    Leave,
    CreateInvite,
    RevokeInvite(String), // string: invite token
    SubscribeChat,
    SubscribeMembers,
    SubscribeRemote,
}

impl Action<Room> for RoomAction {
    fn can_act(&self, actor: &crate::auth::actor::Actor, room: &Room) -> bool {
        match actor {
            Actor::None => false,
            Actor::Internal => true,
            Actor::User(user) => {
                match self {
                    Self::Get | Self::SendChat | Self::SubscribeChat | Self::SubscribeMembers | Self::SubscribeRemote | Self::Leave => {
                        room.is_member(user.id)
                    },
                    Self::GetMember(id) => {
                        room.is_member(user.id) && room.is_member(*id)
                    },
                    Self::PassRemote(to_user_id) => {
                        room.can_pass_remote(user.id, *to_user_id)
                    }, 
                    Self::Join(token) => {
                        room.check_invite(token) && room.members.len() < crate::model::room::MAX_ROOM_SIZE
                    },
                    Self::CreateInvite => {
                        room.is_member(user.id) && !room.invites.iter().any(|i| i.inviter == user.id && i.token.is_valid())
                    },
                    Self::RevokeInvite(token) => {
                        let invite_exists = room.invites.iter().any(|i| i.token.check(&token));
                        let is_member_or_owner = room.is_member(user.id) || room.is_owner(user.id);
                        let is_owner_or_created_invite = room.is_owner(user.id) || room.invites.iter().any(|i| i.inviter == user.id);
                        
                        invite_exists && is_member_or_owner && is_owner_or_created_invite
                    }
                }
            }
        }
    }
}

