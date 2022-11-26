pub mod room;
pub mod user;
use async_graphql::*;

use self::{room::*, user::*};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
struct QueryExtensions;

#[Object]
impl QueryExtensions {
    async fn version(&self) -> &'static str {
        VERSION
    }
}

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, RoomQuery, QueryExtensions);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, RoomMutation);

#[derive(MergedSubscription, Default)]
pub struct Subscription(UserSubscription, RoomSubscription);
