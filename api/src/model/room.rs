
use async_graphql::{SimpleObject, Error, Enum, ComplexObject, Context};

use crate::{types::{time::Time, token::Token, color::{Color, ColorType}, id::{Id, UuidId, NumId}}, auth::authority::Authority, store::DataStore, stream::StreamController};

use super::{user::User, Model};

pub const MAX_ROOM_SIZE: usize = 5;
pub const INVITE_EXPIRY_MINUTES: usize = 5;

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Room {
    pub id: Id<Self>,
    pub name: String,
    pub owner: Option<Id<User>>,
    pub members: Vec<RoomMember>,
    pub remote: Option<Id<User>>,
    pub messages: Vec<RoomChatMsg>,
    pub invites: Vec<RoomInvite>,
}

impl Model for Room {
    type Id = UuidId;

    fn id(&self) -> &Self::Id {
        &self.id.0
    }
}

#[ComplexObject]
impl Room {
    async fn get_my_member<'ctx>(&self, ctx: &Context<'ctx>) -> async_graphql::Result<RoomMember> {
        let user = ctx.actor_user()?;
        if let Some(member) = self.members.iter().find(|m| &m.user == &user.id) {
            return async_graphql::Result::Ok(member.clone())
        } else {
            return async_graphql::Result::Err("You are not a member of this room".into())
        }
    }

    async fn member_capacity(&self) -> usize {
        crate::model::room::MAX_ROOM_SIZE
    }

    async fn member_count(&self) -> usize {
        self.members.len()
    }
}

impl Room {
    pub fn new(name: String) -> Self {
        Self {
            id: Id::new(),
            name,
            owner: None,
            members: vec![],
            remote: None,
            messages: vec![],
            invites: vec![],
        }
    }

    pub fn is_init(&self) -> bool {
        self.owner.is_some()
    }

    pub fn init_owner(&mut self, owner: &User, color: Option<ColorType>) -> Result<RoomMember, Error> {
        self.set_owner(owner.id.clone());
        self.remote = Some(owner.id.clone());
        let member = self.join(owner, color)?;
        Ok(member)
    }

    pub fn set_name(&mut self, name: impl ToString) {
        self.name = name.to_string();
    }

    pub fn set_owner(&mut self, owner: Id<User>) {
        self.owner = Some(owner);
    }

    pub fn join(&mut self, user: &User, color: Option<ColorType>) -> Result<RoomMember, Error> {
        if self.members.iter().any(|m| m.user == user.id) {
            return Err("User already in room".into());
        }
        // todo: check if color is available, if not return err
        let member = RoomMember::new(user.id.clone(), &self, color);
        self.members.push(member.clone());
        Ok(member)
    }

    pub fn leave(&mut self, user: &User) -> Result<RoomMember, Error> {
        if let Some(index) = self.members.iter().position(|m| m.user == user.id) {
            let room_member = self.members.remove(index);
            return Ok(room_member);
        }
        Err("User not in room".into())
    }

    pub fn is_member(&self, id: &Id<User>) -> bool {
        self.members.iter().any(|m| &m.user == id)
    }

    pub fn is_owner(&self, id: &Id<User>) -> bool {
        if let Some(owner) = &self.owner {
            owner == id
        } else {
            false
        }
    }

    pub fn can_pass_remote(&self, from: &Id<User>, to: &Id<User>) -> bool {
        if !self.is_member(to) {
            return false
        }
        if let Some(owner) = &self.owner {
            if owner == from {
                return true
            }
        }
        if let Some(remote) = &self.remote {
            if remote == from {
                return true
            }
        }
        false
    }

    pub fn pass_remote(&mut self, from: &Id<User>, to: Id<User>) -> Result<(), Error> {
        if !self.is_member(&from) || !self.is_member(&to) {
            return Err("User not in room".into())
        }
        if !self.can_pass_remote(&from, &to) {
            return Err("You don't have the remote".into())
        }
        self.remote = Some(to);
        Ok(())
    }

    pub fn pick_available_color(&self) -> ColorType {
        *self.get_available_colors().first().expect("What the fuck, there are no colors")
    }

    pub fn get_available_colors(&self) -> Vec<ColorType> {
        ColorType::all().into_iter().filter(|c| self.is_color_available(*c)).collect()
    }

    pub fn is_color_available(&self, color: ColorType) -> bool {
        !self.members.iter().any(|m| m.color.name == color)
    }

    pub fn get_member(&self, id: &Id<User>) -> Option<&RoomMember> {
        self.members.iter().find(|m| &m.user == id)
    }

    pub fn get_member_mut(&mut self, id: &Id<User>) -> Option<&mut RoomMember> {
        self.members.iter_mut().find(|m| &m.user == id)
    }

    pub fn create_invite(&mut self, user_id: Id<User>) -> Result<RoomInvite, Error> {
        if !self.is_member(&user_id) {
            return Err("User not in room".into())
        }

        if let Some(invite) = self.invites.iter().find(|i| i.token.is_valid() && &i.inviter == &user_id) {
            return Ok(invite.clone())
        }

        let invite = RoomInvite::new(user_id);
        self.invites.push(invite.clone());
        
        Ok(invite)
    }

    pub fn revoke_invite(&mut self, token: impl ToString) -> Result<(), Error> {
        let token = token.to_string();
        if let Some(index) = self.invites.iter().position(|i| i.token.check(&token)) {
            self.invites.remove(index);
            return Ok(())
        }
        Err("Invite not found".into())
    }

    pub fn check_invite(&self, token: impl ToString) -> bool {
        let token = token.to_string();
        self.invites.iter().any(|i| i.token.check(&token))
    }

    pub fn add_chat_msg(&mut self, chat: RoomChatMsg) {
        self.messages.push(chat);
    }

    pub fn create_chat_msg(&mut self, author: &User, msg: impl ToString) -> Result<RoomChatMsg, Error> {
        if !self.is_member(&author.id) {
            return Err("User not in room".into())
        }
        let id = self.messages.iter().map(|m| m.id.0.0).max().unwrap_or(0u32) + 1u32;
        let msg = RoomChatMsg::new(Id(NumId::from_u32(id)), self.id.clone(), author.id.clone(), msg.to_string(), Time::now());
        self.add_chat_msg(msg.clone());
        Ok(msg)
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct RoomMember {
    #[graphql(skip)]
    pub user: Id<User>,
    pub color: Color,
    pub connection: RoomMemberConnection,
    // todo connection shit
}

impl RoomMember {
    pub fn new(user: Id<User>, room: &Room, color: Option<ColorType>) -> Self {
        Self {
            user,
            color: color.unwrap_or(room.pick_available_color()).into(),
            connection: RoomMemberConnection::new(),
        }
    }

    pub fn handle_disconnect(&self, room_id: &Id<Room>, stream_ctl: &StreamController) {
        let room_member_update = RoomMemberUpdate::new_leave(room_id.clone(), self.clone());
        stream_ctl.publish(room_member_update);
    }

    pub fn is_connected(&self) -> bool {
        self.connection.connected_chat || self.connection.connected_members || self.connection.connected_remote
    }
}

#[ComplexObject]
impl RoomMember {
    async fn user(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        let user_store = ctx.data::<DataStore<User>>()?;
        let user = user_store.get(&self.user)?.ok_or("User not found")?;
        Ok(user.clone())
    }

    async fn online(&self) -> bool {
        self.is_connected()
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct RoomMemberConnection {
    pub connected_chat: bool,
    pub connected_members: bool,
    pub connected_remote: bool,
}

impl RoomMemberConnection {
    fn new() -> Self {
        Self {
            connected_chat: false,
            connected_members: false,
            connected_remote: false,
        }
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct RoomChatMsg {
    pub id: Id<Self>,
    pub room: Id<Room>,
    pub author: Id<User>,
    pub msg: String,
    pub time: Time,
}

impl RoomChatMsg {
    pub fn new(id: Id<Self>, room: Id<Room>, author: Id<User>, msg: String, time: Time) -> Self {
        Self {
            id,
            room,
            author,
            msg,
            time
        }
    }
}

impl Model for RoomChatMsg {
    type Id = NumId;
    fn id(&self) -> &Self::Id {
        &self.id.0
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct RoomInvite {
    pub token: Token,
    pub inviter: Id<User>, // user id
}

impl RoomInvite {
    pub fn new(inviter: Id<User>) -> Self {
        let duration = INVITE_EXPIRY_MINUTES as u64 * 60 * 1000;
        let expiry = Time::duration(duration);
        Self {
            token: Token::new_with_expiry(expiry),
            inviter,
        }
    }
}

#[derive(Debug, Enum, PartialEq, Eq, Clone, Copy)]
pub enum RoomMemberUpdateType {
    Join,
    Leave,
}

#[derive(Debug, SimpleObject, Clone)]
pub struct RoomMemberUpdate {
    pub room: Id<Room>,
    pub update_type: RoomMemberUpdateType,
    pub room_member: RoomMember,
}

impl RoomMemberUpdate {
    pub fn new_join(room: Id<Room>, room_member: RoomMember) -> Self {
        Self {
            room,
            update_type: RoomMemberUpdateType::Join,
            room_member,
        }
    }

    pub fn new_leave(room: Id<Room>, room_member: RoomMember) -> Self {
        Self {
            room,
            update_type: RoomMemberUpdateType::Leave,
            room_member,
        }
    }
}

#[derive(Debug, SimpleObject, Clone)]
pub struct RoomRemoteUpdate {
    pub room: Id<Room>,
    pub from: Option<Id<User>>,
    pub to: Id<User>,
}

impl RoomRemoteUpdate {
    pub fn new(room: Id<Room>, from: Option<Id<User>>, to: Id<User>) -> Self {
        Self {
            room,
            from,
            to
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_room_with_members() -> (Room, User, User) {
        let mut room = Room::new("test".to_string());
        let owner = User::new("owner".into());
        let friend = User::new("friend".into());
        room.init_owner(&owner, None).unwrap();
        room.join(&friend, None).unwrap();
        (room, owner, friend)
    }

    #[test]
    fn room_name() {
        let mut room = Room::new("test".into());
        let owner = User::new("owner".into());
        room.init_owner(&owner, None).unwrap();
        assert_eq!(&room.name, "test");
    }

    #[test]
    fn room_join() {
        let (room, owner, friend) = create_room_with_members();
        assert_eq!(room.members.len(), 2);
        assert!(room.is_member(&owner.id));
        assert!(room.is_member(&friend.id));
    }

    #[test]
    fn room_leave() {
        let (mut room, owner, friend) = create_room_with_members();
        room.leave(&friend).unwrap();
        assert_eq!(room.members.len(), 1);
        assert!(room.is_member(&owner.id));
        assert!(!room.is_member(&friend.id));
    }
}