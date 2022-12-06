pub mod room;
pub mod user;
use crate::model::room::RoomMember;
use crate::model::{user::User, room::Room};
use crate::store::DataStore;
use crate::types::id::Id;

use self::{
    room::{RoomMutation, RoomQuery, RoomSubscription},
    user::{UserMutation, UserQuery},
};
use async_graphql::*;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, RoomQuery, NodeQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, RoomMutation);

#[derive(MergedSubscription, Default)]
pub struct Subscription(RoomSubscription);

#[derive(Interface)]
#[graphql(field(name = "id", type = "String"))]
pub enum Node {
    User(User),
    Room(Room),
    RoomMember(RoomMember),
}

impl Node {
    pub fn fetch_node(id: String, ctx: &Context<'_>) -> Option<Self> {
        if let Some(id) = Id::<User>::from_str(&id) {
            let store = ctx.data_opt::<DataStore<User>>()?;
            if let Some(entry) = store.get(&id) {
                return Some(Node::User(entry.clone()));
            }
        }

        if let Some(id) = Id::<Room>::from_str(&id) {
            let store = ctx.data_opt::<DataStore<Room>>()?;
            if let Some(entry) = store.get(&id) {
                return Some(Node::Room(entry.clone()));
            }
        }

        if let Some(id) = Id::<RoomMember>::from_str(&id) {
            let store = ctx.data_opt::<DataStore<Room>>()?;
            if let Some(room) = Room::get_by_member(store, &id) {
                if let Some(member) = room.get_member(&id) {
                    return Some(Node::RoomMember(member.clone()));
                }
            }
        }

        None // todo
    }
}

#[derive(Default)]
pub struct NodeQuery;

#[Object]
impl NodeQuery {
    async fn node<'ctx>(&self, ctx: &Context<'ctx>, id: String) -> Result<Node, Error> {
        let node = Node::fetch_node(id, ctx).ok_or::<async_graphql::Error>("Node not found".into())?;
        Ok(node)
    }
}