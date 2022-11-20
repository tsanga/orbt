use async_graphql::{Error, Context};

use crate::model::user::User;

use super::{action::{Action}, actor::Actor};

pub trait Authority {
    fn require_act<M>(&self, action: impl Action<M>, model: &M) -> Result<Actor, Error>;
    fn actor_user(&self) -> Result<User, Error>;
    //fn actor_internal(&self) -> Result<(), Error>;
    //fn actor(&self) -> Actor;
}

impl Authority for Context<'_> {
    fn require_act<M>(&self, action: impl Action<M>, model: &M) -> Result<Actor, Error> {
        let actor = self.data::<Actor>()?;
        if actor.can_act::<M>(action.clone(), model) {
            Ok(actor.clone())
        } else {
            Err(Error::new(format!("Unauthorized to perform action: {:?}", &action)))
        }
    }

    fn actor_user(&self) -> Result<User, Error> {
        let actor = self.data::<Actor>()?;
        let Actor::User(user) = actor else { return Err("Requires 'user' actor type.".into()) };
        Ok(user.clone())
    }
}