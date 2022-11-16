use async_graphql::{SimpleObject, InputObject};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use super::time::Time;

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, InputObject)]
pub struct Token {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<Time>,
}

impl Token {
    fn generate() -> String {
        let uuid = &Uuid::new_v4().to_string()[..8];
        base64::encode(uuid.to_string())
    }

    pub fn new() -> Self {
        Self {
            token: Some(Self::generate()),
            expires: None,
        }
    }

    pub fn new_with_expiry(expiry: Time) -> Self {
        Self {
            token: Some(Self::generate()),
            expires: Some(expiry),
        }
    }

    pub fn check(&self, other: impl ToString) -> bool {
        if !self.is_valid() {
            return false
        }
        if let Some(token) = self.token.as_ref() {
            token == &other.to_string()
        } else {
            false
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.token.is_none() {
            return false
        }
        if let Some(expires) = self.expires {
            expires > Time::now()
        } else {
            true
        }
    }

    pub fn invalidate(&mut self) {
        self.token = None;
        self.expires = None;
    }
}