pub mod user;
pub mod room;

use std::sync::{Arc, RwLock};

pub use user::UserStore;
pub use room::RoomStore;

#[derive(Clone)]
pub struct DataStore {
    inner: Arc<DataStoreInner>,
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