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
    pub fn identify(user_store: web::Data<DataStore<User>>, request: HttpRequest) -> Self {
        if let Some(identifier) = request.headers().get("Authorization") {
            let identifier = identifier.to_str().unwrap();
            return Self::identify_with_token(user_store, identifier)
        }
        Self::None
    }

    pub fn identify_with_token(user_store: web::Data<DataStore<User>>, identifier: impl ToString) -> Self {
        let identifier = identifier.to_string();
        if identifier == option_env!("API_TOKEN").unwrap_or("ORBT_INTERNAL") {
            return Self::Internal;
        } else {
            if let Some(user) = user_store.data.lock().unwrap().values().find(|u| u.token.check(&identifier)) {
                return Self::User(user.clone());
            }
        }
        Self::None
    }

    pub fn can_act<M>(&self, action: impl Action<M>, model: &M) -> bool {
        action.can_act(&self, model)
    }

    #[allow(dead_code)]
    pub fn is_user_or_internal(&self) -> bool {
        match self {
            Self::None => false,
            Self::User(_) => true,
            Self::Internal => true,
        }
    }

    #[allow(dead_code)]
    pub fn is_user(&self) -> bool {
        match self {
            Self::User(_) => true,
            _ => false
        }
    }

    #[allow(dead_code)]
    pub fn is_internal(&self) -> bool {
        match self {
            Self::Internal => true,
            _ => false
        }
    }

    #[allow(dead_code)]
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
        let user = User::new("tester".to_string());
        let user_store = DataStore::<User>::new();
        user_store.insert(user.clone());
        let actor = Actor::identify_with_token(web::Data::new(user_store), user.token.to_string());
        assert_eq!(
            actor.as_user().unwrap().token.token.as_ref().unwrap(), 
            user.token.token.as_ref().unwrap()
        );
    }
}