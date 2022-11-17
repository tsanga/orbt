use std::{sync::{Mutex, Arc, RwLock}, collections::HashMap};
use crate::model::room::Room;
use crate::prelude::*;

pub struct RoomStore {
    pub rooms: Arc<RwLock<HashMap<u32, Room>>>,
    id_counter: Arc<Mutex<u32>>,
}

impl RoomStore {
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(RwLock::new(HashMap::new())),
            id_counter: Arc::new(Mutex::new(0)),
        }
    }

    fn get_new_id(&self) -> Result<u32> {
        let mut id_counter = self.id_counter.lock().unwrap();
        let id = *id_counter;
        *id_counter += 1;
        Ok(id)
    }

    pub fn new_room(&mut self) -> Result<Room> {
        let id = self.get_new_id()?;
        let room = Room::new(id);
        let rooms = &mut self.rooms.write().unwrap();
        rooms.insert(id, room.clone());
        Ok(room)
    }

    pub fn delete_room(&self, id: u32){
        self.rooms.write().unwrap().remove(&id);
    }

    pub fn get_room_by_id(&self, id: u32) -> Option<Room> {
        self.rooms.read().unwrap().get(&id).cloned()
    }

    pub fn save(&self, room: Room) {
        let mut rooms = self.rooms.write().unwrap();
        rooms.insert(room.id, room);
    }
}