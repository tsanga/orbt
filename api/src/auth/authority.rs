use async_graphql::{Error, Context};

use crate::{model::user::User, store::{DataStore, DataStoreEntry}};

use super::{action::{Action}, actor::Actor};

pub trait Authority {
    fn require_act<M>(&self, action: impl Action<M>, model: &M) -> Result<Actor, Error>;
    fn actor_user(&self) -> Result<DataStoreEntry<User>, Error>;
    //fn actor_internal(&self) -> Result<(), Error>;
    //fn actor(&self) -> Actor;
}

impl Authority for Context<'_> {
    fn require_act<M>(&self, action: impl Action<M>, model: &M) -> Result<Actor, Error> {
        let Ok(actor) = self.data::<Actor>() else { return Err("Not authenticated".into()) };
        if actor.can_act::<M>(action.clone(), model) {
            Ok(actor.clone())
        } else {
            Err(Error::new(format!("Unauthorized to perform action: {:?}", &action)))
        }
    }

    fn actor_user(&self) -> Result<DataStoreEntry<User>, Error> {
        let actor = self.data::<Actor>()?;
        let Actor::User(user_id) = actor else { return Err("Requires 'user' actor type.".into()) };
        let user_store = self.data::<DataStore<User>>()?;
        let user = user_store.get(user_id)?.ok_or::<async_graphql::Error>("User not found.".into())?;
        Ok(user)
    }
}