use async_graphql::SimpleObject;
use serde::{Serialize, Deserialize};

use crate::types::{token::Token};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub token: Token,
}

impl User {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            token: Token::new(),
        }
    }
}