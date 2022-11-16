use async_graphql::{Error, Context};

use crate::model::user::User;

use super::{action::Action, actor::Actor};

pub trait Authority {
    fn require_act(&self, action: &impl Action) -> Result<Actor, Error>;
    //fn actor_user(&self) -> Result<User, Error>;
    //fn actor_internal(&self) -> Result<(), Error>;
    //fn actor(&self) -> Actor;
}

impl Authority for Context<'_> {
    fn require_act(&self, action: &impl Action) -> Result<Actor, Error> {
        let actor = self.data::<Actor>()?;
        if actor.can_act(action) {
            Ok(actor.clone())
        } else {
            Err(Error::new(format!("Unauthorized to perform action: {}", &action.to_string())))
        }
    }
}