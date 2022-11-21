use async_graphql::*;
use serde::{Serialize, Deserialize};

use crate::{types::{token::Token}, store::DataStore};

use super::room::{RoomMember, Room};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub token: Token,
}

impl User {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            token: Token::new(),
        }
    }
}

#[ComplexObject]
impl User {
    async fn room<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<Room>> {
        let store = ctx.data::<DataStore>()?;
        let room_store_lock = store.room_store();
        let room_store = room_store_lock.read().unwrap();
        let room_opt = room_store.rooms.read().unwrap().values().into_iter().find(|r| r.is_member(self.id)).cloned();
        Ok(room_opt)
    }

    async fn room_member<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<RoomMember>> {
        let store = ctx.data::<DataStore>()?;
        let room_store_lock = store.room_store();
        let room_store = room_store_lock.read().unwrap();
        let room_opt = room_store.rooms.read().unwrap().values().into_iter().find(|r| r.is_member(self.id)).cloned();
        let Some(room) = room_opt else { return Ok(None) };
        let room_member = room.get_member(self.id).cloned();
        Ok(room_member)
    }
}