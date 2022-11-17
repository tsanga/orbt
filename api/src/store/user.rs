use std::{sync::{Mutex, Arc, RwLock}, collections::HashMap};

use crate::{prelude::*, model::user::User};

pub struct UserStore {
    pub users: Arc<RwLock<HashMap<u32, User>>>,
    id_counter: Arc<Mutex<u32>>,
}

impl UserStore {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            id_counter: Arc::new(Mutex::new(0)),
        }
    }

    pub fn get_new_id(&self) -> Result<u32> {
        let mut id_counter = self.id_counter.lock().unwrap();
        let id = *id_counter;
        *id_counter += 1;
        Ok(id)
    }

    pub fn new_user(&mut self, name: String) -> Result<User> {
        let id = self.get_new_id()?;
        let user = User::new(id, name);
        let users = &mut self.users.write().unwrap();
        users.insert(id, user.clone());
        Ok(user)
    }

    pub fn delete_user(&self, id: u32) {
        self.users.write().unwrap().remove(&id);
    }

    pub fn get_user_by_id(&self, id: u32) -> Option<User> {
        self.users.read().unwrap().get(&id).cloned()
    }

    pub fn get_user_by_token(&self, token: impl ToString) -> Option<User> {
        let token = token.to_string();
        self.users.read().unwrap().values().find(|user| user.token.check(&token)).cloned()
    }

    pub fn save(&self, user: User) {
        let mut users = self.users.write().unwrap();
        users.insert(user.id, user);
    }
}