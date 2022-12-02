use crate::{auth::authority::Authority, model::user::User, store::DataStore, types::id::Id};
use async_graphql::*;

#[derive(Default)]
pub struct UserQuery;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserQuery {
    async fn user<'ctx>(&self, ctx: &Context<'ctx>, id: Id<User>) -> Result<Option<User>> {
        let user_store = ctx.data::<DataStore<User>>()?;
        let user = user_store.get(&id)?;
        Ok(user.as_deref().cloned())
    }

    async fn me<'ctx>(&self, ctx: &Context<'ctx>) -> Result<User> {
        let user = ctx.actor_user()?;
        Ok(user.clone())
    }
}

#[Object]
impl UserMutation {
    async fn create_user<'ctx>(&self, ctx: &Context<'ctx>, name: Option<String>) -> Result<User> {
        let name = name.unwrap_or("".to_string());
        let user_store = ctx.data::<DataStore<User>>()?;
        let user = User::new(name);
        user_store.insert(user.clone());
        Ok(user)
    }

    async fn set_user_name<'ctx>(&self, ctx: &Context<'ctx>, name: String) -> Result<User> {
        let mut user = ctx.actor_user()?;
        user.name = name;
        // saved implicitly when dropped
        Ok(user.clone())
    }
}
