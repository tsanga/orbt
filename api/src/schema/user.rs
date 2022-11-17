use std::time::Duration;

use async_graphql::*;
use futures::Stream;
use tokio_stream::StreamExt;
use crate::{model::user::User, store::DataStore};

#[derive(Default)]
pub struct UserQuery;

#[derive(Default)]
pub struct UserMutation;

#[derive(Default)]
pub struct UserSubscription;

#[Object]
impl UserQuery {
    async fn get<'ctx>(&self, ctx: &Context<'ctx>, id: u32) -> Result<Option<User>> {
        let store = ctx.data::<DataStore>()?.user_store();
        let user_store = store.read().unwrap();
        let user = user_store.get_user_by_id(id);
        Ok(user)
    }
}

#[Object]
impl UserMutation {
    async fn create<'ctx>(&self, ctx: &Context<'ctx>, name: String) -> Result<User> {
        let store = ctx.data::<DataStore>()?.user_store();
        let mut user_store = store.write().unwrap();
        let user = user_store.new_user(name)?;
        Ok(user)
    }

    async fn set_name<'ctx>(&self, ctx: &Context<'ctx>, id: u32, name: String) -> Result<User> {
        let store = ctx.data::<DataStore>()?.user_store();
        let user_store = store.write().unwrap();
        let mut user = user_store.get_user_by_id(id).ok_or::<async_graphql::Error>("User not found".into())?;
        user.name = name;
        user_store.save(user.clone());
        Ok(user)
    }
}

#[Subscription]
impl UserSubscription {
    async fn integers(&self, step: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(move |_| {
                value += step;
                value
            })
    }
}