use async_graphql::*;

use crate::{
    auth::authority::Authority,
    types::{token::Token}, Database,
};

use super::room::{Room, RoomMember};
use mongodb::bson::{doc, Bson};
use musty::{prelude::{model, Id}, Model};

#[derive(Clone, SimpleObject)]
#[graphql(complex)]
#[model(mongo())]
pub struct User {
    #[graphql(skip)]
    pub id: String,
    pub name: String,
    //#[graphql(skip)]
    pub token: Token,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            id: name.to_string().into(),
            name,
            token: Token::new_with_length(16),
        }
    }

    pub async fn get_owned_room(&self, db: &Database) -> Option<Room> {
        let id: &Bson = &self.id.clone().try_into().unwrap();

        let room = Room::find_one(&db, doc! {"owner": &id }).await.unwrap();
        room
    }
}

#[ComplexObject]
impl User {
    async fn id<'ctx>(&self) -> Id<User> {
        self.id.clone().into()
    }

    async fn room<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Room> {
        let room = ctx.room().await?;
        Ok(room.clone())
    }

    async fn room_member<'ctx>(&self, ctx: &Context<'ctx>) -> Result<RoomMember> {
        let room = ctx.room().await?;
        let member = room
            .get_member(&self.id)
            .ok_or::<async_graphql::Error>("You are not in a room".into())?;
        Ok(member.clone())
    }
}
