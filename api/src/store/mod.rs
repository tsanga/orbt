use std::{collections::HashMap, sync::{Mutex, Arc}};

use crate::{prelude::*, model::user::User};

pub trait DataStore {
    type Error;
    type Model<I: ToString>: DataStoreModel<I>;
    type Id: ToString;

    fn get_by_id<'a>(&'a self, id: &Self::Id) -> Result<Option<&'a Self::Model>, Self::Error>;
    fn save(&mut self, model: &mut Self::Model) -> Result<(), Self::Error>;
    fn delete(&mut self, id: &Self::Id) -> Result<(), Self::Error>;
}

pub trait DataStoreModel<I> where I: ToString {
    fn get_id(&self) -> &I;
}

pub struct UserMemoryDataStore {
    users: HashMap<u32, User>
}

impl DataStore for UserMemoryDataStore {
    type Error = anyhow::Error;
    type Id = u32;
    type Model = User;

    fn get_by_id<'a>(&'a self, id: &Self::Id) -> Result<Option<&'a Self::Model>, Self::Error> {
        let user = self.users.get(id);
        Ok(user)
    }

    fn save(&mut self, model: &mut Self::Model) -> Result<(), Self::Error> {
        if let Some(existing_user) = self.get_by_id(model)
    }

    fn delete(&mut self, id: &Self::Id) -> Result<(), Self::Error> {
        
        Ok(())
    }
}

impl DataStoreModel for User {
    type Id = u32;
    fn get_id(&self) -> &Self::Id {
        &self.id
    }
}