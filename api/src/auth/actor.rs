use actix_web::{web, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::{store::DataStore, model::user::User};

use super::action::{Action};

#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Actor {
    None,
    User(User),
    Internal,
}

impl Actor {
    pub async fn identify(data_store: web::Data<DataStore>, request: HttpRequest) -> Self {
        let data_store = &data_store.into_inner();
        if let Some(identifier) = request.headers().get("Authorization") {
            let identifier = identifier.to_str().unwrap();
            if identifier == option_env!("API_TOKEN").unwrap_or("penis") {
                return Self::Internal;
            } else {
                let user_store_lock = data_store.user_store().clone();
                let user_store = user_store_lock.read().unwrap();
                if let Some(user) = user_store.get_user_by_token(identifier) {
                    return Self::User(user);
                }
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

}
