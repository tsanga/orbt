pub mod room;
pub mod user;
use self::{
    room::{RoomMutation, RoomQuery, RoomSubscription},
    user::{UserMutation, UserQuery},
};
use async_graphql::*;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, RoomQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, RoomMutation);

#[derive(MergedSubscription, Default)]
pub struct Subscription(RoomSubscription);
