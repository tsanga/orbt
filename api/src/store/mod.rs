use std::sync::{Mutex, MutexGuard};
use std::{collections::HashMap, sync::Arc};
use std::ops::{Deref, DerefMut};
use crate::model::Model;
use crate::types::id::{ToId, IdType};

#[derive(Debug, Clone)]
pub struct DataStore<M: Model> {
    pub data: Arc<Mutex<HashMap<<M as Model>::Id, M>>>,
}

impl<M: Model> DataStore<M> {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn get<'a, I: ToId<M>>(&'a self, id: &'a I) -> Result<Option<DataStoreEntry<'a, M>>, <<M as Model>::Id as IdType>::Error> {
        let mut lock = self.data.lock().unwrap();
        let id = id.to_id()?;
        let item = lock.remove(&id);
        if let Some(model) = item {
            let entry = DataStoreEntry::new(id, model, true, lock);
            return Ok(Some(entry))
        }
        Ok(None)
    }

    pub fn insert(&self, model: M) {
        (&*self.data).lock().unwrap().insert(<M as Model>::id(&model).clone(), model);
    }

    pub fn delete<I: ToId<M>>(&self, id: &I) -> Result<(), <<M as Model>::Id as IdType>::Error> {
        let mut lock = self.data.lock().unwrap();
        let id = id.to_id()?;
        lock.remove(&id);
        Ok(())
    }
}

#[derive(Debug)]
pub struct DataStoreEntry<'a, M: Model> {
    id: <M as Model>::Id,
    model: Option<M>,
    save: bool,
    lock: MutexGuard<'a, HashMap<<M as Model>::Id, M>>,
}

impl<'a, M: Model> DataStoreEntry<'a, M> {
    fn new(id: <M as Model>::Id, model: M, save: bool, lock: MutexGuard<'a, HashMap<<M as Model>::Id, M>>) -> Self {
        Self {
            id,
            model: Some(model),
            save,
            lock,
        }
    }
}

impl<'a, M: Model> Deref for DataStoreEntry<'a, M> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        self.model.as_ref().expect("Failed to deref for DataStoreEntry Model") // this will never panic since Option is always Some before value is dropped
    }
}

impl<'a, M: Model> DerefMut for DataStoreEntry<'a, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.model.as_mut().expect("Failed to deref_mut for DataStoreEntry Model")
    }
}

impl <'a, M: Model> Drop for DataStoreEntry<'a, M> {
    fn drop(&mut self) {
        if self.save {
            if let Some(model) = self.model.take() {
                self.lock.insert(self.id.clone(), model);
            }
        }
        // self.lock implicitly dropped
    }
}

#[cfg(test)]
mod test {
    use crate::{model::user::User, types::id::Id};

    use super::*;
    #[test]
    fn store() {
        let user_data_store = DataStore::<User>::new();
        let user = User::new("tester123".to_string());
        let id = user.id.clone();
        user_data_store.insert(user);

        assert!(user_data_store.data.lock().unwrap().contains_key(&id.0));

        {
            let mut set = user_data_store.get(&id).unwrap().unwrap();
            set.name = "jonah".to_string();
            println!("set2");
            // drop is called here, so the value gets saved
        }

        let get2 = user_data_store.get(&id).unwrap().unwrap();

        assert_eq!("jonah", &get2.name);
    }
}
