use async_graphql::*;
use futures::{Stream, StreamExt};

use crate::{model::room::{Room, RoomMember, RoomChatMsg}, store::DataStore, types::{color::Color, time::Time}, auth::{action::Action, authority::Authority, actor::Actor}, stream::SimpleBroker};

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

    async fn get_member<'ctx>(&self, ctx: &Context<'ctx>, room: u32, user: u32) -> Result<Option<RoomMember>> {
        let store = ctx.data::<DataStore>()?.room_store();
        let room_store = store.read().unwrap();
        let room = room_store.get_room_by_id(room).ok_or::<async_graphql::Error>("Room not found".into())?;
        let member = room.get_member(user).cloned();
        if let Some(member) = &member {
            ctx.require_act(RoomAction::GetMember(member.user), &room)?;
        }
        Ok(member)
    }
}

#[Object]
impl RoomMutation {
    async fn create<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Room> {
        let store = ctx.data::<DataStore>()?.room_store();
        let mut room_store = store.write().unwrap();
        let room = room_store.new_room()?;
        ctx.require_act(RoomAction::Create, &room)?;
        Ok(room)
    }

    async fn init<'ctx>(&self, ctx: &Context<'ctx>, room: u32, owner: u32, token: String, color: Option<Color>) -> Result<Room> {
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

        ctx.require_act(RoomAction::SendChat, &room)?;

        let room_chat_msg = RoomChatMsg::new(room_id, 0, msg, Time::now());

        room.add_chat_msg(room_chat_msg.clone());
        room_store.save(room);

        SimpleBroker::publish(room_chat_msg.clone());
        Ok(room_chat_msg)
    }
}

#[Subscription]
impl RoomSubscription {
    async fn chat<'ctx>(&self, ctx: &Context<'ctx>, room_id: u32) -> Result<impl Stream<Item = RoomChatMsg>> {
        let store = ctx.data::<DataStore>()?;
        let room_store_lock = store.room_store();
        let room_store = room_store_lock.read().unwrap();
        let Some(room) = room_store.get_room_by_id(room_id) else { return Err("Room not found".into()) };
        ctx.require_act(RoomAction::SubscribeChat, &room)?;
        Ok(SimpleBroker::<RoomChatMsg>::subscribe().filter(move |event| {
            let res = event.id == room_id;
            async move { res }
        }))
    }
}

#[derive(Debug, Clone)]
pub enum RoomAction {
    Get, // room id
    GetMember(u32), // member user id
    Create,
    SendChat,
    SubscribeChat,
}

impl Action<Room> for RoomAction {
    fn can_act(&self, actor: &crate::auth::actor::Actor, room: &Room) -> bool {
        match actor {
            Actor::None => false,
            Actor::Internal => true,
            Actor::User(user) => {
                match self {
                    Self::Create => {
                        // todo probably add some checks for if they already made a room
                        true
                    },
                    Self::Get | Self::SendChat | Self::SubscribeChat => {
                        room.is_member(user.id)
                    },
                    Self::GetMember(id) => {
                        room.is_member(user.id) && room.is_member(*id)
                    },
                }
            }
        }
    }
}

