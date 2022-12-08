use async_graphql::{connection::*, ComplexObject, Context, Error, SimpleObject};

use crate::{
    auth::authority::Authority,
    store::{DataStore, DataStoreEntry},
    types::{
        color::{Color, ColorType},
        id::{Id, Identifiable},
        time::Time,
        token::Token,
    },
};

use super::{user::User, Model};

pub const MAX_ROOM_SIZE: usize = 5;
pub const INVITE_EXPIRY_MINUTES: usize = 5;

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Room {
    #[graphql(skip)]
    pub id: Id<Self>,
    pub name: String,
    pub owner: Option<Id<User>>,
    pub members: Vec<RoomMember>,
    pub remote: Option<Id<User>>,
    #[graphql(skip)]
    pub messages: Vec<RoomChatMsg>,
    pub invites: Vec<RoomInvite>,
}

impl Model for Room {
    fn model_id(&self) -> &Id<Self> {
        &self.id
    }
}

impl Identifiable for Room {
    const MODEL_IDENT: &'static str = "r";
}

#[ComplexObject]
impl Room {
    pub async fn id(&self) -> Id<Self> {
        self.id.clone()
    }

    async fn get_my_member<'ctx>(&self, ctx: &Context<'ctx>) -> async_graphql::Result<RoomMember> {
        let user = ctx.user()?;
        if let Some(member) = self.members.iter().find(|m| &m.user == &user.id) {
            return async_graphql::Result::Ok(member.clone());
        } else {
            return async_graphql::Result::Err("You are not a member of this room".into());
        }
    }

    async fn member_capacity(&self) -> usize {
        crate::model::room::MAX_ROOM_SIZE
    }

    async fn member_count(&self) -> usize {
        self.members.len()
    }

    async fn messages(
        &self,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> async_graphql::Result<Connection<usize, RoomChatMsg, EmptyFields, EmptyFields>> {
        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let messages_len = self.messages.len();

                let mut start = after.map(|after| after + 1).unwrap_or(0);
                let mut end = before.unwrap_or(messages_len);
                if let Some(first) = first {
                    end = (start + first).min(end);
                }
                if let Some(last) = last {
                    start = if last > end - start { end } else { end - last };
                }
                let mut connection = Connection::new(start > 0, end < messages_len);
                for (i, m) in self
                    .messages
                    .iter()
                    .enumerate()
                    .skip(start)
                    .take(end - start)
                {
                    connection.edges.push(Edge::new(i, m.clone()));
                }
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
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

    pub fn init_owner(&mut self, owner: &User) {
        self.set_owner(owner.id.clone());
        self.remote = Some(owner.id.clone());
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
        if let Some(color) = color {
            if !self.is_color_available(color) {
                return Err("Color not available.".into());
            }
        }
        let member = RoomMember::new(user.id.clone(), &self, color);
        self.members.push(member.clone());
        Ok(member)
    }

    pub fn leave(&mut self, user: &Id<User>) -> Result<RoomMember, Error> {
        if let Some(index) = self.members.iter().position(|m| &m.user == user) {
            let room_member = self.members.remove(index);
            return Ok(room_member);
        }
        Err("User not in room".into())
    }

    pub fn is_member_user(&self, id: &Id<User>) -> bool {
        self.members.iter().any(|m| &m.user == id)
    }

    pub fn is_member(&self, id: &Id<RoomMember>) -> bool {
        self.members.iter().any(|m| &m.id == id)
    }

    pub fn is_connected(&self, id: &Id<User>) -> bool {
        self.get_member_by_user_id(id).map(|m| m.connected).unwrap_or(false)
    }

    pub fn is_owner(&self, id: &Id<User>) -> bool {
        if let Some(owner) = &self.owner {
            owner == id
        } else {
            false
        }
    }

    pub fn can_pass_remote(&self, from: &Id<User>, to: &Id<User>) -> bool {
        if !self.is_member_user(to) {
            return false;
        }
        if let Some(owner) = &self.owner {
            if owner == from {
                return true;
            }
        }
        if let Some(remote) = &self.remote {
            if remote == from {
                return true;
            }
        }
        false
    }

    pub fn pass_remote(&mut self, from: &Id<User>, to: Id<User>) -> Result<(), Error> {
        if !self.is_member_user(&from) || !self.is_member_user(&to) {
            return Err("User not in room".into());
        }
        if !self.can_pass_remote(&from, &to) {
            return Err("You don't have the remote".into());
        }
        self.remote = Some(to);
        Ok(())
    }

    pub fn pick_available_color(&self) -> ColorType {
        *self
            .get_available_colors()
            .first()
            .expect("What the fuck, there are no colors")
    }

    pub fn get_available_colors(&self) -> Vec<ColorType> {
        ColorType::all()
            .into_iter()
            .filter(|c| self.is_color_available(*c))
            .collect()
    }

    pub fn is_color_available(&self, color: ColorType) -> bool {
        !self.members.iter().any(|m| m.color.name == color)
    }

    pub fn get_member_by_user_id(&self, id: &Id<User>) -> Option<&RoomMember> {
        self.members.iter().find(|m| &m.user == id)
    }

    pub fn get_member_by_user_id_mut(&mut self, id: &Id<User>) -> Option<&mut RoomMember> {
        self.members.iter_mut().find(|m| &m.user == id)
    }

    pub fn get_member(&self, id: &Id<RoomMember>) -> Option<&RoomMember> {
        self.members.iter().find(|m| &m.id == id)
    }

    pub fn get_member_mut(&mut self, id: &Id<RoomMember>) -> Option<&mut RoomMember> {
        self.members.iter_mut().find(|m| &m.id == id)
    }

    pub fn create_invite(&mut self, user_id: Id<User>) -> Result<RoomInvite, Error> {
        if !self.is_member_user(&user_id) {
            return Err("User not in room".into());
        }

        if let Some(invite) = self
            .invites
            .iter()
            .find(|i| i.token.is_valid() && &i.inviter == &user_id)
        {
            return Ok(invite.clone());
        }

        let invite = RoomInvite::new(user_id);
        self.invites.push(invite.clone());

        Ok(invite)
    }

    pub fn revoke_invite(&mut self, token: impl ToString) -> Result<(), Error> {
        let token = token.to_string();
        if let Some(index) = self.invites.iter().position(|i| i.token.check(&token)) {
            self.invites.remove(index);
            return Ok(());
        }
        Err("Invite not found".into())
    }

    pub fn check_invite(&self, token: impl ToString) -> bool {
        let token = token.to_string();
        self.invites.iter().any(|i| i.token.check(&token))
    }

    pub fn is_invited_or_owner(&self, user: &User, token: Option<String>) -> bool {
        if let Some(token) = token {
            return self.check_invite(token);
        }
        if self.is_owner(&user.id) {
            return true;
        }
        false
    }

    pub fn add_chat_msg(&mut self, chat: RoomChatMsg) {
        self.messages.push(chat);
    }

    pub fn create_chat_msg(
        &mut self,
        author: &User,
        msg: impl ToString,
    ) -> Result<RoomChatMsg, Error> {
        if !self.is_member_user(&author.id) {
            return Err("User not in room".into());
        }
        let msg = RoomChatMsg::new(
            Id::new(),
            author.id.clone(),
            msg.to_string(),
            Time::now(),
        );
        Ok(msg)
    }
}

// helpers
impl Room {
    pub fn any_room<F: Fn(&Room) -> bool>(room_store: &DataStore<Room>, func: F) -> bool {
        room_store.data.lock().unwrap().values().any(|r| func(r))
    }

    pub fn find_room<F: Fn(&Room) -> bool>(
        room_store: &DataStore<Room>,
        func: F,
    ) -> Option<DataStoreEntry<Room>> {
        let id = room_store
            .data
            .lock()
            .unwrap()
            .values()
            .find(|r| func(r))
            .map(|r| r.id.clone());
        if let Some(id) = id {
            room_store.get(&id)
        } else {
            None
        }
    }

    pub fn get_by_member_user_id<'a>(room_store: &'a DataStore<Room>, user: &'a Id<User>) -> Option<DataStoreEntry<'a, Room>> {
        Self::find_room(room_store, |r| r.is_member_user(user))
    }

    pub fn get_by_member<'a>(room_store: &'a DataStore<Room>, id: &'a Id<RoomMember>) -> Option<DataStoreEntry<'a, Room>> {
        Self::find_room(room_store, |r| r.is_member(id))
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct RoomMember {
    #[graphql(skip)]
    pub id: Id<Self>,
    #[graphql(skip)]
    pub user: Id<User>,
    pub color: Color,
    pub connected: bool,
    pub typing: bool,
}

impl RoomMember {
    pub fn new(user: Id<User>, room: &Room, color: Option<ColorType>) -> Self {
        Self {
            id: Id::new(),
            user,
            color: color.unwrap_or(room.pick_available_color()).into(),
            connected: false,
            typing: false,
        }
    }
}

impl Model for RoomMember {
    fn model_id(&self) -> &Id<Self> {
        &self.id
    }
}

impl Identifiable for RoomMember {
    const MODEL_IDENT: &'static str = "rm";
}

#[ComplexObject]
impl RoomMember {
    pub async fn id(&self) -> Id<Self> {
        self.id.clone()
    }

    async fn user(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        let user_store = ctx.data::<DataStore<User>>()?;
        let user = user_store.get(&self.user).ok_or("User not found")?;
        Ok(user.clone())
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct RoomChatMsg {
    pub id: Id<Self>,
    pub author: Id<User>,
    pub msg: String,
    pub time: Time,
}

impl RoomChatMsg {
    pub fn new(id: Id<Self>, author: Id<User>, msg: String, time: Time) -> Self {
        Self {
            id,
            author,
            msg,
            time,
        }
    }
}

impl Model for RoomChatMsg {
    fn model_id(&self) -> &Id<Self> {
        &self.id
    }
}

impl Identifiable for RoomChatMsg {
    const MODEL_IDENT: &'static str = "msg";
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_room_with_members() -> (Room, User, User) {
        let mut room = Room::new("test".to_string());
        let owner = User::new("owner".into());
        let friend = User::new("friend".into());
        room.init_owner(&owner);
        room.join(&owner, None).unwrap();
        room.join(&friend, None).unwrap();
        (room, owner, friend)
    }

    #[test]
    fn room_name() {
        let mut room = Room::new("test".into());
        let owner = User::new("owner".into());
        room.init_owner(&owner);
        assert_eq!(&room.name, "test");
    }

    #[test]
    fn room_join() {
        let (room, owner, friend) = create_room_with_members();
        assert_eq!(room.members.len(), 2);
        assert!(room.is_member_user(&owner.id));
        assert!(room.is_member_user(&friend.id));
    }

    #[test]
    fn room_leave() {
        let (mut room, owner, friend) = create_room_with_members();
        room.leave(&friend.id).unwrap();
        assert_eq!(room.members.len(), 1);
        assert!(room.is_member_user(&owner.id));
        assert!(!room.is_member_user(&friend.id));
    }
}
