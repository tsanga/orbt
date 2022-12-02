use async_graphql::*;
use serde::{Deserialize, Serialize};

use crate::{
    store::DataStore,
    types::{
        id::{Id, UuidId},
        token::Token,
    },
};

use super::{
    room::{Room, RoomMember},
    Model,
};

#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct User {
    pub id: Id<Self>,
    pub name: String,
    //#[graphql(skip)]
    pub token: Token,
}

impl Model for User {
    type Id = UuidId;

    fn id(&self) -> &Self::Id {
        &self.id.0
    }
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            id: Id::new(),
            name,
            token: Token::new_with_length(16),
        }
    }
}

#[ComplexObject]
impl User {
    async fn room<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<Room>> {
        let room_store = ctx.data::<DataStore<Room>>()?;
        let room_opt = room_store
            .data
            .lock()
            .unwrap()
            .values()
            .into_iter()
            .find(|r| r.is_member(&self.id))
            .cloned();
        Ok(room_opt)
    }

    async fn room_member<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<RoomMember>> {
        let room_store = ctx.data::<DataStore<Room>>()?;
        let room_opt = room_store
            .data
            .lock()
            .unwrap()
            .values()
            .into_iter()
            .find(|r| r.is_member(&self.id))
            .cloned();
        let Some(room) = room_opt else { return Ok(None) };
        let room_member = room.get_member(&self.id).cloned();
        Ok(room_member)
    }
}
