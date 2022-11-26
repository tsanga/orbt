use async_graphql::*;
use serde::{Deserialize, Serialize};

use crate::{store::DataStore, types::token::Token};

use super::room::{Room, RoomMember};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct User {
    #[graphql(skip)]
    pub id: u32,
    pub name: String,
    //#[graphql(skip)]
    pub token: Token,
}

impl User {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            token: Token::new_with_length(16),
        }
    }
}

#[ComplexObject]
impl User {
    async fn room<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<Room>> {
        let store = ctx.data::<DataStore>()?;
        let room_store_lock = store.room_store();
        let room_store = room_store_lock.read().unwrap();
        let room_opt = room_store
            .rooms
            .read()
            .unwrap()
            .values()
            .into_iter()
            .find(|r| r.is_member(self.id))
            .cloned();
        Ok(room_opt)
    }

    async fn room_member<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<RoomMember>> {
        let store = ctx.data::<DataStore>()?;
        let room_store_lock = store.room_store();
        let room_store = room_store_lock.read().unwrap();
        let room_opt = room_store
            .rooms
            .read()
            .unwrap()
            .values()
            .into_iter()
            .find(|r| r.is_member(self.id))
            .cloned();
        let Some(room) = room_opt else { return Ok(None) };
        let room_member = room.get_member(self.id).cloned();
        Ok(room_member)
    }

    async fn id(&self) -> String {
        self.id.to_string()
    }
}
