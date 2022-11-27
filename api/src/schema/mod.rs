pub mod user;
pub mod room;
use async_graphql::*;
use self::{room::{RoomSubscription, RoomQuery, RoomMutation}, user::{UserQuery, UserMutation}};

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, RoomQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, RoomMutation);

#[derive(MergedSubscription, Default)]
pub struct Subscription(RoomSubscription);

