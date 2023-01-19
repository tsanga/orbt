use async_graphql::{Context, Error, async_trait::async_trait};
use musty::Model;

use crate::{
    model::{room::Room, user::User}, Database,
};

use super::{action::Action, actor::Actor};

#[async_trait]
pub trait Authority {
    fn require_act<M>(&self, action: impl Action<M>, model: &M) -> Result<Actor, Error>;
    async fn user(&self) -> Result<User, Error>;
    async fn room(&self) -> Result<Room, Error>;
}

#[async_trait]
impl Authority for Context<'_> {
    fn require_act<M>(&self, action: impl Action<M>, model: &M) -> Result<Actor, Error> {
        let Ok(actor) = self.data::<Actor>() else { return Err("Not authenticated".into()) };
        if actor.can_act::<M>(action.clone(), model) {
            Ok(actor.clone())
        } else {
            Err(Error::new(format!(
                "Unauthorized to perform action: {:?}",
                &action
            )))
        }
    }


    async fn user(&self) -> Result<User, Error> {
        User::from_context(&self).await
    }

    async fn room(&self) -> Result<Room, Error> {
        Room::from_context(&self).await
    }
}

#[async_trait]
pub trait FromContext where Self: Sized {
    async fn from_context<'a>(ctx: &Context<'a>) -> async_graphql::Result<Self>;
}

#[async_trait]
impl FromContext for User {
    async fn from_context<'a>(ctx: &Context<'a>) -> async_graphql::Result<Self> {
        let actor = ctx.data::<Actor>()?;
        let Actor::User(user) = actor else { return Err("Requires 'user' actor type.".into()) };
        Ok(user.clone())
    }
}

#[async_trait]
impl FromContext for Room {
    async fn from_context<'a>(ctx: &Context<'a>) -> async_graphql::Result<Self> {
        let actor = ctx.data::<Actor>()?;
        let Actor::User(user) = actor else { return Err("Requires 'user' actor type.".into()) };
        let db = ctx.data::<Database>()?;
        let room = Room::get_by_member_user_id(&db, &user.id).await
            .ok_or::<async_graphql::Error>("User is not in a room".into())?;
        Ok(room)
    }
}
