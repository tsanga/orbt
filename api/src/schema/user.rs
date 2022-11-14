use async_graphql::*;
use crate::model::user::User;

#[derive(Default)]
pub struct UserQuery;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserQuery {
    async fn get<'ctx>(&self, _ctx: &Context<'ctx>, name: String) -> Result<User> {
        Ok(User { name })
    }
}

#[Object]
impl UserMutation {
    async fn create<'ctx>(&self, _ctx: &Context<'ctx>, name: String) -> Result<User> {
        Ok(User { name })
    }
}