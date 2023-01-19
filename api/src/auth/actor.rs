use actix_web::{web, HttpRequest};
use async_graphql::Context;
use musty::Model;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::{
    model::user::User, Database,
};

use super::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Actor {
    None,
    User(User),
    Internal,
}

impl Actor {
    pub async fn identify(db: &Database, request: HttpRequest) -> Self {
        if let Some(identifier) = request.headers().get("Authorization") {
            let identifier = identifier.to_str().unwrap();
            return Self::identify_with_token(db, identifier).await;
        }
        Self::None
    }

    pub async fn identify_with_token(
        db: &Database,
        identifier: impl ToString,
    ) -> Self {
        let identifier = identifier.to_string();
        if identifier == option_env!("API_TOKEN").unwrap_or("ORBT_INTERNAL") {
            return Self::Internal;
        } else {
            let user = User::find_one(&db, doc! { "token": identifier }).await;
            if let Ok(Some(user)) = user
            {
                return Self::User(user);
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

    pub async fn user<'ctx>(
        self,
        ctx: &Context<'ctx>,
    ) -> async_graphql::Result<User> {
        let Self::User(user) = self else { return Err("Requires 'user' actor type.".into()) };
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

   /*  #[test]
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
    }*/
}
