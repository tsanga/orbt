pub mod user;

use async_graphql::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Query;

#[derive(Default)]
pub struct Mutation;

#[Object]
impl Query {
    async fn version(&self) -> &'static str {
        VERSION
    }

    async fn user<'ctx>(&self) -> user::UserQuery {
        user::UserQuery::default()
    }
}

#[Object]
impl Mutation {
    async fn user<'ctx>(&self) -> user::UserMutation {
        user::UserMutation::default()
    }
}