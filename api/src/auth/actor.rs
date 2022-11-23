use actix_web::{web, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::{store::DataStore, model::user::User};

use super::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Actor {
    None,
    User(User),
    Internal,
}

impl Actor {
    pub fn identify(data_store: web::Data<DataStore>, request: HttpRequest) -> Self {
        if let Some(identifier) = request.headers().get("Authorization") {
            let identifier = identifier.to_str().unwrap();
            return Self::identify_with_token(data_store, identifier)
        }
        Self::None
    }

    pub fn identify_with_token(data_store: web::Data<DataStore>, identifier: impl ToString) -> Self {
        let identifier = identifier.to_string();
        if identifier == option_env!("API_TOKEN").unwrap_or("ORBT_INTERNAL") {
            return Self::Internal;
        } else {
            let data_store = &data_store.into_inner();
            let user_store_lock = data_store.user_store().clone();
            let user_store = user_store_lock.read().unwrap();
            if let Some(user) = user_store.get_user_by_token(identifier) {
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
            Self::None => false,
            Self::User(_) => true,
            Self::Internal => true,
        }
    }

    pub fn is_user(&self) -> bool {
        match self {
            Self::User(_) => true,
            _ => false
        }
    }

    pub fn is_internal(&self) -> bool {
        match self {
            Self::Internal => true,
            _ => false
        }
    }

    pub fn as_user(&self) -> Option<User> {
        match self {
            Self::User(user) => Some(user.clone()),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actor_identify_with_token() {
        let user = User::new(0,"tester".to_string());
        let data_store = DataStore::new();
        let user_store_lock = data_store.user_store();
        let user_store = user_store_lock.write().unwrap();
        user_store.save(user.clone());
        drop(user_store); // necessary to prevent deadlock
        let actor = Actor::identify_with_token(web::Data::new(data_store), user.token.to_string());
        assert_eq!(
            actor.as_user().unwrap().token.token.as_ref().unwrap(), 
            user.token.token.as_ref().unwrap()
        );
    }
}