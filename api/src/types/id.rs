use std::{str::FromStr, hash::Hash};
use wither::bson::oid::ObjectId;

pub struct Id<T>(String, PhantomData<T>);

impl<T> Id<T> {
    pub fn from(id: String) -> Self {
        Self(id, PhantomData)
    }
    pub fn new() -> Self {
        let id = ObjectId::new().to_string();
        Self::from(id)
    }
}

impl<T> ToString for Id<T> {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl<T> From<String> for Id<T> {
    fn from(id: String) -> Self {
        Self::from(id)
    }
}

impl<T> From<&str> for Id<T> {
    fn from(id: &str) -> Self {
        Self::from(id.to_string())
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}