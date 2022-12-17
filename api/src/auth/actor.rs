use actix_web::{web, HttpRequest};
use async_graphql::Context;
use serde::{Deserialize, Serialize};

use crate::{
    model::user::User,
    store::{DataStore, DataStoreEntry},
    types::id::Id,
};

use super::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Actor {
    None,
    User(Id<User>),
    Internal,
}

impl Actor {
    pub fn identify(user_store: web::Data<DataStore<User>>, request: HttpRequest) -> Self {
        if let Some(identifier) = request.headers().get("Authorization") {
            let identifier = identifier.to_str().unwrap();
            return Self::identify_with_token(user_store, identifier);
        }
        Self::None
    }

    pub fn identify_with_token(
        user_store: web::Data<DataStore<User>>,
        identifier: impl ToString,
    ) -> Self {
        let identifier = identifier.to_string();
        if identifier == option_env!("API_TOKEN").unwrap_or("ORBT_INTERNAL") {
            return Self::Internal;
        } else {
            if let Some(user) = user_store
                .data
                .lock()
                .unwrap()
                .values()
                .find(|u| u.token.check(&identifier))
            {
                return Self::User(user.id.clone());
            }
        }
        Self::None
    }

    pub fn can_act<M>(&self, action: impl Action<M>, model: &M) -> bool {
        action.can_act(&self, model)
    }

    pub fn is_user_or_internal(&self) -> bool {
        match self {
            Self::User(_) | Self::Internal => true,
            _ => false,
        }
    }

    pub fn is_user(&self) -> bool {
        match self {
            Self::User(_) => true,
            _ => false,
        }
    }

    pub fn is_internal(&self) -> bool {
        match self {
            Self::Internal => true,
            _ => false,
        }
    }

    pub fn user<'ctx>(
        self,
        ctx: &Context<'ctx>,
    ) -> async_graphql::Result<DataStoreEntry<'ctx, User>> {
        let Self::User(user_id) = self else { return Err("Requires 'user' actor type.".into()) };
        let user_store = ctx.data::<DataStore<User>>()?;
        let user = user_store
            .get(user_id)
            .ok_or::<async_graphql::Error>("User not found.".into())?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actor_identify_with_token() {
        let user = User::new("tester".to_string());
        let user_store = DataStore::<User>::new();
        user_store.insert(user.clone());
        let actor = Actor::identify_with_token(web::Data::new(user_store), user.token.to_string());
        if let Actor::User(id) = actor {
            assert_eq!(id, user.id);
        } else {
            panic!("Expected Actor::User, got {:?}", actor);
        }
    }
}
