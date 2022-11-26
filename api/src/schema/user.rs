use async_graphql::*;
use crate::{model::user::User, store::DataStore, auth::authority::Authority, types::id::Id};

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
        Ok(user)
    }
}

#[Object]
impl UserMutation {
    async fn user_create<'ctx>(&self, ctx: &Context<'ctx>, name: Option<String>) -> Result<User> {
        let name = name.unwrap_or("".to_string());
        let user_store = ctx.data::<DataStore<User>>()?;
        let user = User::new(name);
        user_store.insert(user.clone());
        Ok(user)
    }
    
    async fn user_set_name<'ctx>(&self, ctx: &Context<'ctx>, id: Id<User>, name: String) -> Result<User> {
        let user_store = ctx.data::<DataStore<User>>()?;
        let mut user = user_store.get(&id)?.ok_or::<async_graphql::Error>("User not found".into())?;
        user.name = name;
        // saved implicitly when dropped
        Ok(user.clone())
    }
}