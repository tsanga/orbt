use crate::{auth::authority::Authority, model::user::User, Database};
use async_graphql::*;
use musty::prelude::{Model, Id};

#[derive(Default)]
pub struct UserQuery;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserQuery {
    async fn user<'ctx>(&self, ctx: &Context<'ctx>, id: Id<User>) -> Result<Option<User>> {
        let db = ctx.data::<Database>()?;
        Ok(User::get_by_id(&db, id).await?)
    }

    async fn me<'ctx>(&self, ctx: &Context<'ctx>) -> Result<User> {
        let user = ctx.user().await?;
        Ok(user)
    }
}

#[Object]
impl UserMutation {
    async fn create_user<'ctx>(&self, ctx: &Context<'ctx>, name: Option<String>) -> Result<User> {
        let db = ctx.data::<Database>()?;
        let name = name.unwrap_or("".to_string());
        let mut user = User::new(name);
        user.save(&db).await?;
        Ok(user)
    }

    async fn set_user_name<'ctx>(&self, ctx: &Context<'ctx>, name: String) -> Result<User> {
        let db = ctx.data::<Database>()?;
        let mut user = ctx.user().await?;
        user.name = name;

        user.save(&db).await?;
        
        Ok(user)
    }
}
