use async_graphql::{SimpleObject, Error, Enum, ComplexObject, Context};

use crate::{types::{time::Time, token::Token, color::{Color, ColorType}}, auth::authority::Authority, store::DataStore};

use super::user::User;

pub const MAX_ROOM_SIZE: usize = 5;
pub const MAX_ROOM_NAME_LENGTH: usize = 20;
pub const INVITE_EXPIRY_MINUTES: usize = 5;

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Room {
    pub id: u32,
    pub name: Option<String>,
    pub owner: Option<u32>,
    pub members: Vec<RoomMember>,
    pub remote: Option<u32>,
    pub messages: Vec<RoomChatMsg>,
    //#[graphql(skip)] TODO: uncomment
    pub create_token: Token,
    pub invites: Vec<RoomInvite>,
}

#[ComplexObject]
impl Room {
    async fn get_my_member<'ctx>(&self, ctx: &Context<'ctx>) -> async_graphql::Result<RoomMember> {
        let user = ctx.actor_user()?;
        if let Some(member) = self.members.iter().find(|m| m.user == user.id) {
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
    pub fn new(id: u32) -> Self {
        Self {
            id,
            name: None,
            owner: None,
            members: vec![],
            remote: None,
            messages: vec![],
            create_token: Token::new(),
            invites: vec![],
        }
    }

    pub fn is_init(&self) -> bool {
        self.name.is_some() && self.owner.is_some()
    }

    pub fn init_owner(&mut self, owner: &User, color: Option<ColorType>) -> Result<RoomMember, Error> {
        self.set_owner(owner.id);
        self.set_name(format!("{}'s Room", &owner.name));
        self.remote = Some(owner.id);
        let member = self.join(owner, color)?;
        self.create_token.invalidate();
        Ok(member)
    }

    pub fn set_name(&mut self, name: impl ToString) {
        self.name = Some(name.to_string());
    }

    pub fn set_owner(&mut self, owner: u32) {
        self.owner = Some(owner);
    }

    pub fn join(&mut self, user: &User, color: Option<ColorType>) -> Result<RoomMember, Error> {
        if self.members.iter().any(|m| m.user == user.id) {
            return Err("User already in room".into());
        }
        // todo: check if color is available, if not return err
        let member = RoomMember::new(user.id, &self, color);
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

    pub fn is_member(&self, id: u32) -> bool {
        self.members.iter().any(|m| m.user == id)
    }

    pub fn is_owner(&self, id: u32) -> bool {
        if let Some(owner) = self.owner {
            owner == id
        } else {
            false
        }
    }

    pub fn can_pass_remote(&self, from: u32, to: u32) -> bool {
        if !self.is_member(to) {
            return false
        }
        if let Some(owner) = self.owner {
            if owner == from {
                return true
            }
        }
        if let Some(remote) = self.remote {
            if remote == from {
                return true
            }
        }
        false
    }

    pub fn pass_remote(&mut self, from: u32, to: u32) -> Result<(), Error> {
        if !self.is_member(from) || !self.is_member(to) {
            return Err("User not in room".into())
        }
        if !self.can_pass_remote(from, to) {
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

    pub fn get_member(&self, id: u32) -> Option<&RoomMember> {
        self.members.iter().find(|m| m.user == id)
    }

    pub fn create_invite(&mut self, user_id: u32) -> Result<RoomInvite, Error> {
        if !self.is_member(user_id) {
            return Err("User not in room".into())
        }

        if let Some(invite) = self.invites.iter().find(|i| i.token.is_valid() && i.inviter == user_id) {
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
        if !self.is_member(author.id) {
            return Err("User not in room".into())
        }
        let id = self.messages.iter().map(|m| m.id).max().unwrap_or(0) + 1;
        let msg = RoomChatMsg::new(id, author.id, msg.to_string(), Time::now());
        self.add_chat_msg(msg.clone());
        Ok(msg)
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct RoomMember {
    #[graphql(skip)]
    pub user: u32,
    pub color: Color,
    // todo connection shit
}

impl RoomMember {
    pub fn new(user_id: u32, room: &Room, color: Option<ColorType>) -> Self {
        Self {
            user: user_id,
            color: color.unwrap_or(room.pick_available_color()).into(),
        }
    }
}

#[ComplexObject]
impl RoomMember {
    async fn user(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        let store = ctx.data::<DataStore>()?;
        let user_store_lock = store.user_store();
        let user_store = user_store_lock.read().unwrap();
        let user = user_store.get_user_by_id(self.user).ok_or("User not found")?;
        Ok(user)
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct RoomChatMsg {
    pub id: u32,
    pub author: u32,
    pub msg: String,
    pub time: Time,
}

impl RoomChatMsg {
    fn new(id: u32, author: u32, msg: String, time: Time) -> Self {
        Self {
            id,
            author,
            msg,
            time
        }
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct RoomInvite {
    pub token: Token,
    pub inviter: u32, // user id
}

impl RoomInvite {
    pub fn new(inviter: u32) -> Self {
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
    pub room: u32,
    pub update_type: RoomMemberUpdateType,
    pub room_member: RoomMember,
    pub user: User,
}

impl RoomMemberUpdate {
    pub fn new_join(room: u32, room_member: RoomMember, user: User) -> Self {
        Self {
            room,
            update_type: RoomMemberUpdateType::Join,
            room_member,
            user,
        }
    }

    pub fn new_leave(room: u32, room_member: RoomMember, user: User) -> Self {
        Self {
            room,
            update_type: RoomMemberUpdateType::Leave,
            room_member,
            user,
        }
    }
}

#[derive(Debug, SimpleObject, Clone)]
pub struct RoomRemoteUpdate {
    pub room: u32,
    pub from: Option<u32>,
    pub to: u32,
}

impl RoomRemoteUpdate {
    pub fn new(room: u32, from: Option<u32>, to: u32) -> Self {
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
        let mut room = Room::new(1);
        let owner = User::new(0, "owner".into());
        let friend = User::new(1, "friend".into());
        room.init_owner(&owner, None).unwrap();
        room.join(&friend, None).unwrap();
        (room, owner, friend)
    }

    #[test]
    fn room_join() {
        let (room, owner, friend) = create_room_with_members();
        assert_eq!(room.members.len(), 2);
        assert!(room.is_member(0));
        assert!(room.is_member(1));
    }

    #[test]
    fn room_leave() {
        let (mut room, _owner, friend) = create_room_with_members();
        room.leave(&friend).unwrap();
        assert_eq!(room.members.len(), 1);
        assert!(room.is_member(0));
        assert!(!room.is_member(1));
    }
}