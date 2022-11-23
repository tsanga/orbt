pub mod room;
pub mod user;
use async_graphql::*;

use self::{room::RoomSubscription, user::UserSubscription};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct RootQuery;

#[derive(MergedObject, Default)]
pub struct Mutation(user::UserMutation, room::RoomMutation);

#[derive(MergedSubscription, Default)]
pub struct Subscription(RoomSubscription, UserSubscription);

#[Object]
impl RootQuery {
    async fn version(&self) -> &'static str {
        VERSION
    }
}

#[derive(MergedObject, Default)]
pub struct Query(RootQuery, user::UserQuery, room::RoomQuery);
