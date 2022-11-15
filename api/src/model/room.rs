use serde::{Serialize, Deserialize};

use crate::types::id::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub id: Id<Room>,
    pub name: String,
    pub owner: Id<User>,
    pub members: Vec<RoomMember>,
    pub remote: Option<Id<User>>,
    pub messages: Vec<RoomChatMsg>,
}

pub struct RoomMember {
    pub user: Id<User>,
    pub 
}

pub struct RoomChatMsg {
    pub id: u32,
    pub author: Id<User>,
    pub msg: String,
    pub time: Time
}