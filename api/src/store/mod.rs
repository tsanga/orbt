pub mod user;
pub mod room;

use std::{sync::{Arc, RwLock}, ops::Deref};

pub use user::UserStore;
pub use room::RoomStore;

#[derive(Clone)]
pub struct DataStore {
    inner: Arc<DataStoreInner>,
}

impl Deref for DataStore {
    type Target = Arc<DataStoreInner>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DataStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(DataStoreInner::new()),
        }
    }

    pub fn user_store(&self) -> Arc<RwLock<UserStore>> {
        self.inner.clone().user_store.clone()
    }

    pub fn room_store(&self) -> Arc<RwLock<RoomStore>> {
        self.inner.clone().room_store.clone()
    }
}

pub struct DataStoreInner {
    pub user_store: Arc<RwLock<UserStore>>,
    pub room_store: Arc<RwLock<RoomStore>>,
}

impl DataStoreInner {
    pub fn new() -> Self {
        Self {
            user_store: Arc::new(RwLock::new(UserStore::new())),
            room_store: Arc::new(RwLock::new(RoomStore::new())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::user::User;

    use super::*;

    #[test]
    fn does_store_user() {
        let data_store = DataStore::new();
        let new_user = User::new(0, "test".to_string());
        let user_store_lock = data_store.user_store();
        let user_store = user_store_lock.write().unwrap();
        user_store.save(new_user);
        assert!(user_store.users.read().unwrap().contains_key(&0u32));
    }
}