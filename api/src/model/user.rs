use async_graphql::SimpleObject;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct User {
    pub id: u32,
    pub name: String,
}