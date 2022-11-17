pub mod user;
pub mod room;

use std::time::Duration;

use async_graphql::*;
use futures::Stream;
use tokio_stream::StreamExt;

use crate::{model::room::RoomChatMsg, types::time::Time};

use self::{room::RoomSubscription, user::UserSubscription};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Query;

#[derive(Default)]
pub struct Mutation;

#[derive(MergedSubscription, Default)]
pub struct Subscription(RoomSubscription, UserSubscription);

#[Object]
impl Query {
    async fn version(&self) -> &'static str {
        VERSION
    }

    async fn user<'ctx>(&self) -> user::UserQuery {
        user::UserQuery::default()
    }

    async fn room<'ctx>(&self) -> room::RoomQuery {
        room::RoomQuery::default()
    }
}

#[Object]
impl Mutation {
    async fn user<'ctx>(&self) -> user::UserMutation {
        user::UserMutation::default()
    }

    async fn room<'ctx>(&self) -> room::RoomMutation {
        room::RoomMutation::default()
    }
}