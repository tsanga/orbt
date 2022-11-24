use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;

pub trait Id: Send + Sync + ToString + PartialEq + Eq + Hash + Sized + Clone + std::fmt::Debug {
    type Error: std::error::Error;
    fn new() -> Self;
    fn from(id: impl ToString) -> Result<Self, Self::Error>;
}

pub trait Model : Clone {
    type Id: Id;
    fn id(&self) -> &Self::Id;
}

#[derive(Clone, Debug)]
pub struct ModelId<M: Model>(<M as Model>::Id);

impl<M: Model> ModelId<M> {
    pub fn new() -> Self {
        Self(<M as Model>::Id::new())
    }
    pub fn get<'a>(&self, data_store: &'a DataStore<M>) -> Option<&'a M> {
        data_store.get(&self.0)
    }
}

impl<M: Model> Deref for ModelId<M> {
    type Target = <M as Model>::Id;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct DataStore<M: Model> {
    data: HashMap<<M as Model>::Id, M>,
}

impl<M: Model> DataStore<M> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, id: &<M as Model>::Id) -> Option<&M> {
        self.data.get(id)
    }

    pub fn insert(&mut self, model: M) {
        self.data.insert(<M as Model>::id(&model).clone(), model);
    }
}

#[derive(Clone, Debug)]
pub struct User {
    id: ModelId<Self>,
}

impl Model for User {
    type Id = StringId;
    fn id(&self) -> &Self::Id {
        &self.id.0
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct StringId(String);

impl ToString for StringId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Id for StringId {
    type Error = std::convert::Infallible;
    fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
    fn from(id: impl ToString) -> Result<Self, Self::Error> {
        Ok(Self(id.to_string()))
    }
}

#[cfg(test)]
mod test {
    use async_graphql::Context;

    use super::*;
    #[test]
    fn test() {
        let mut user_data_store = DataStore::<User>::new();
        let id = ModelId::<User>::new();
        let user = User { id: id.clone() };
        user_data_store.insert(user);
        
        let retrieved_user = user_data_store.get(&id).unwrap();
        assert_eq!(&id.0.0, &retrieved_user.id().0);

        // or
        let u = id.get(&user_data_store).unwrap();
    }

    fn test_ctx(ctx: &Context<'_>) {
        let user_data_store = ctx.data::<DataStore<User>>().unwrap();
        let user = user_data_store.get(&StringId("test".to_string())).unwrap();
    }
}