use async_graphql::*;
use serde::{Deserialize, Serialize};

use crate::{
    types::{
        id::{Id, Identifiable},
        token::Token,
    }, auth::authority::Authority,
};

use super::{
    room::{Room, RoomMember},
    Model,
};

#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct User {
    #[graphql(skip)]
    pub id: Id<Self>,
    pub name: String,
    //#[graphql(skip)]
    pub token: Token,
}

impl Model for User {
    fn model_id(&self) -> &Id<Self> {
        &self.id
    }
}

impl Identifiable for User {
    const MODEL_IDENT: &'static str = "u";
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
    pub async fn id(&self) -> Id<Self> {
        self.id.clone()
    }

    async fn room<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Room> {
        let room = ctx.room()?;
        Ok(room.clone())
    }

    async fn room_member<'ctx>(&self, ctx: &Context<'ctx>) -> Result<RoomMember> {
        let room = ctx.room()?;
        let member = room.get_member_by_user_id(&self.id).ok_or::<async_graphql::Error>("You are not in a room".into())?;
        Ok(member.clone())
    }
}
