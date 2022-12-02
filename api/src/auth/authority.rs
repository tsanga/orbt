use async_graphql::{Context, Error};

use crate::{
    model::{user::User, Model, room::Room},
    store::{DataStore, DataStoreEntry},
};

use super::{action::Action, actor::Actor};

pub trait Authority {
    fn require_act<M>(&self, action: impl Action<M>, model: &M) -> Result<Actor, Error>;
    fn user(&self) -> Result<DataStoreEntry<User>, Error>;
    fn room(&self) -> Result<DataStoreEntry<Room>, Error>;
}

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

    fn user(&self) -> Result<DataStoreEntry<User>, Error> {
        User::from_context(&self)
    }

    fn room(&self) -> Result<DataStoreEntry<Room>, Error> {
        Room::from_context(&self)
    }
}

pub trait FromContext where Self: Model {
    fn from_context<'a>(ctx: &Context<'a>) -> async_graphql::Result<DataStoreEntry<'a, Self>>;
}

impl FromContext for User {
    fn from_context<'a>(ctx: &Context<'a>) -> async_graphql::Result<DataStoreEntry<'a, Self>> {
        let actor = ctx.data::<Actor>()?;
        let Actor::User(user_id) = actor else { return Err("Requires 'user' actor type.".into()) };
        let user_store = ctx.data::<DataStore<User>>()?;
        let user = user_store
            .get(user_id)?
            .ok_or::<async_graphql::Error>("User not found.".into())?;
        Ok(user)
    }
}

impl FromContext for Room {
    fn from_context<'a>(ctx: &Context<'a>) -> async_graphql::Result<DataStoreEntry<'a, Self>> {
        let actor = ctx.data::<Actor>()?;
        let Actor::User(user_id) = actor else { return Err("Requires 'user' actor type.".into()) };
        let room_store = ctx.data::<DataStore<Room>>()?;
        let room = Room::get_by_member(room_store, user_id).ok_or::<async_graphql::Error>("User is not in a room".into())?;
        Ok(room)
    }
}