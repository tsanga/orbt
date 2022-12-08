pub mod room;
pub mod user;
use crate::auth::authority::Authority;
use crate::model::room::RoomMember;
use crate::model::{user::User, room::Room};
use crate::store::DataStore;
use crate::types::id::{Id, Identifiable};

use self::room::RoomAction;
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
#[graphql(field(name = "id", type = "Id<Node>"))]
pub enum Node {
    User(User),
    Room(Room),
    RoomMember(RoomMember),
}

impl Identifiable for Node {
    const MODEL_IDENT: &'static str = "n";
}

impl Node {
    pub fn fetch_node(id: Id<Self>, ctx: &Context<'_>) -> Result<Option<Self>> {
        if let Some(id) = Id::<User>::try_from_str(&id) {
            let store = ctx.data::<DataStore<User>>()?;
            if let Some(entry) = store.get(&id) {
                // TODO: auth / require_act
                return Ok(Some(Node::User(entry.clone())));
            }
        }

        if let Some(id) = Id::<Room>::try_from_str(&id) {
            let store = ctx.data::<DataStore<Room>>()?;
            if let Some(entry) = store.get(&id) {
                ctx.require_act(RoomAction::Get, &*entry)?;
                return Ok(Some(Node::Room(entry.clone())));
            }
        }

        if let Some(id) = Id::<RoomMember>::try_from_str(&id) {
            let store = ctx.data::<DataStore<Room>>()?;
            if let Some(room) = Room::get_by_member(store, &id) {
                ctx.require_act(RoomAction::Get, &*room)?;
                if let Some(member) = room.get_member(&id) {
                    return Ok(Some(Node::RoomMember(member.clone())));
                }
            }
        }

        Ok(None)
    }
}

#[derive(Default)]
pub struct NodeQuery;

#[Object]
impl NodeQuery {
    async fn node<'ctx>(&self, ctx: &Context<'ctx>, id: Id<Node>) -> Option<Node> {
        Node::fetch_node(id, ctx).ok().flatten()
    }
}