use async_graphql::*;

use crate::{model::{room::{Room, RoomMember}}, store::DataStore, types::color::Color, auth::action::Action};

#[derive(Default)]
pub struct RoomQuery;

#[derive(Default)]
pub struct RoomMutation;

#[Object]
impl RoomQuery {
    async fn get<'ctx>(&self, ctx: &Context<'ctx>, id: u32) -> Result<Option<Room>> {
        let store = ctx.data::<DataStore>()?.room_store();
        let room_store = store.read().unwrap();
        let room = room_store.get_room_by_id(id);
        Ok(room)
    }

    async fn get_member<'ctx>(&self, ctx: &Context<'ctx>, room: u32, user: u32) -> Result<Option<RoomMember>> {
        let store = ctx.data::<DataStore>()?.room_store();
        let room_store = store.read().unwrap();
        let room = room_store.get_room_by_id(room).ok_or::<async_graphql::Error>("Room not found".into())?;
        let member = room.get_member(user).cloned();
        Ok(member)
    }
}

#[Object]
impl RoomMutation {
    async fn create<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Room> {
        let store = ctx.data::<DataStore>()?.room_store();
        let mut room_store = store.write().unwrap();
        let room = room_store.new_room()?;
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
}

pub enum RoomAction {
    Get(u32),
    Create,
}

impl Action for RoomAction {
    fn can_act(&self, actor: &crate::auth::actor::Actor) -> bool {
        match self {
            RoomAction::Get(_) => actor.is,
            RoomAction::Create => actor.is_internal(),
        }
    }
}