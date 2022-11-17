use async_graphql::{SimpleObject, Error};
use chrono::{Duration, Utc};

use crate::types::{time::Time, token::Token, color::Color};

use super::user::User;

pub const MAX_ROOM_SIZE: usize = 5;
pub const MAX_ROOM_NAME_LENGTH: usize = 20;
pub const INVITE_EXPIRY_MINUTES: usize = 5;

#[derive(Debug, Clone, SimpleObject)]
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

    pub fn init_owner(&mut self, owner: &User, color: Option<Color>) -> Result<RoomMember, Error> {
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

    pub fn join(&mut self, user: &User, color: Option<Color>) -> Result<RoomMember, Error> {
        if self.members.iter().any(|m| m.user == user.id) {
            return Err("User already in room".into());
        }
        let member = RoomMember::new(user.id, &self, color);
        self.members.push(member.clone());
        Ok(member)
    }

    pub fn leave(&mut self, user: &User) -> Result<(), Error> {
        if let Some(index) = self.members.iter().position(|m| m.user == user.id) {
            self.members.remove(index);
            return Ok(());
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

    fn can_pass_remote(&self, from: u32, to: u32) -> bool {
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

    pub fn pick_available_color(&self) -> Color {
        *self.get_available_colors().first().expect("What the fuck, there are no colors")
    }

    pub fn get_available_colors(&self) -> Vec<Color> {
        Color::all().into_iter().filter(|c| self.is_color_available(*c)).collect()
    }

    pub fn is_color_available(&self, color: Color) -> bool {
        !self.members.iter().any(|m| m.color == color)
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
}

#[derive(Debug, Clone, SimpleObject)]
pub struct RoomMember {
    pub user: u32,
    pub color: Color,
    // todo connection shit
}

impl RoomMember {
    pub fn new(user_id: u32, room: &Room, color: Option<Color>) -> Self {
        Self {
            user: user_id,
            color: color.unwrap_or(room.pick_available_color()),
        }
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct RoomChatMsg {
    pub id: u32,
    pub author: u32,
    pub msg: String,
    pub time: Time,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct RoomInvite {
    pub token: Token,
    pub inviter: u32, // user id
}

impl RoomInvite {
    pub fn new(inviter: u32) -> Self {
        let now = Utc::now().timestamp_millis() as u128;
        let duration = Duration::minutes(INVITE_EXPIRY_MINUTES as i64).num_milliseconds() as u128;
        let expiry = Time::from(now + duration);
        Self {
            token: Token::new_with_expiry(expiry),
            inviter,
        }
    }
}