use crate::model::Model;
use std::{fmt::Display, hash::Hash, marker::PhantomData, any::Any};

use async_graphql::{InputValueError, Scalar, ScalarType, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Id<M: Any> {
    pub id: String,
    pub node_suffix: String,
    _phantom: PhantomData<M>
}

impl<M: Any> Id<M> {
    pub fn new_from_id(id: String, node_suffix: String) -> Self {
        Self { id, node_suffix, _phantom: PhantomData }
    }

    pub fn as_ref<T: Any>(id: &Id<T>) -> &Id<M> {
        unsafe { &*(id as *const Id<T> as *const Id<M>) }
    }
}

impl<M: Model> Id<M> {
    pub fn new() -> Self {
        let uid = uuid::Uuid::new_v4().to_string().replace("-", "")[..6].to_string();
        Self::new_from_id(uid, M::NODE_SUFFIX.to_string())
    }

    pub fn from_str(id: impl ToString) -> Self {
        let id = id.to_string();
        if id.contains("$") {
            if let Ok((id, node_suffix)) = parse_id_parts(id.clone()) {
                return Self::new_from_id(id, node_suffix)
            }
        }
        Self::new_from_id(id, M::NODE_SUFFIX.to_string())
    }

    pub fn try_from_str(id: impl ToString) -> Option<Self> {
        let id = id.to_string();
        if id.contains("$") {
            if let Ok((id, node_suffix)) = parse_id_parts(id.clone()) {
                if node_suffix == M::NODE_SUFFIX || M::NODE_SUFFIX == "n" {
                    return Some(Self::new_from_id(id, node_suffix))
                }
            }
        }
        None
    }
    pub fn from_model_id(id: <M as Model>::Id) -> Self {
        Self(id)
    }
}

impl<M: Any> Display for Id<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}${}", self.id, self.node_suffix)
    }
}

impl<M: Any> PartialEq for Id<M> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.node_suffix == other.node_suffix
    }
}

impl<M: Any> Eq for Id<M> {}

impl<M: Any> Clone for Id<M> {
    fn clone(&self) -> Self {
        Self::new_from_id(self.id.clone(), self.node_suffix.clone())
    }
}

impl<M: Any> Hash for Id<M> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.node_suffix.hash(state);
    }
}

#[Scalar]
impl<M: Model> ScalarType for Id<M> {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let Value::String(value_str) = &value {
            if let Some(id) = Self::try_from_str(value_str) {
                Ok(id)
            } else {
                Err(InputValueError::expected_type(value))
            }
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> async_graphql::Value {
        Value::String(self.to_string())
    }
}

impl<M: Model> Serialize for Id<M> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct IdVisitor<M: Model>(std::marker::PhantomData<M>);

impl<'de, M: Model> serde::de::Visitor<'de> for IdVisitor<M> {
    type Value = Id<M>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Id::try_from_str(value).ok_or(E::custom("invalid id"))
    }
}

impl<'de, M: Model> Deserialize<'de> for Id<M> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(IdVisitor(std::marker::PhantomData))
    }
}

pub trait ToId<M: Model> {
    fn to_id(&self) -> Option<Id<M>>;
}

impl<M: Model> ToId<M> for Id<M> {
    fn to_id(&self) -> Option<Id<M>> {
        Some(self.clone())
    }
}

impl<M: Model> ToId<M> for &Id<M> {
    fn to_id(&self) -> Option<Id<M>> {
        (*self).to_id()
    }
}

impl<M: Model> ToId<M> for String {
    fn to_id(&self) -> Option<Id<M>> {
        Id::<M>::try_from_str(&self)
    }
}

impl<M: Model> ToId<M> for &str {
    fn to_id(&self) -> Option<Id<M>> {
        Id::<M>::try_from_str(&self)
    }
}

impl<M: Model> From<Id<M>> for String {
    fn from(id: Id<M>) -> Self {
        id.to_string()
    }
}

impl<M: Model> From<&Id<M>> for String {
    fn from(id: &Id<M>) -> Self {
        id.to_string()
    }
}

impl<M: Model> From<String> for Id<M> {
    fn from(s: String) -> Self {
        Id::<M>::from_str(s)
    }
}


/*impl From<Id<User>> for Id<Node> {
    fn from(i: Id<User>) -> Self {
        Id::<Node>::new_from_id(i.id, i.node_suffix)
    }
}

impl<'a> From<&'a Id<User>> for &'a Id<Node> {
    fn from(i: &'a Id<User>) -> Self {
        Id::<Node>::as_ref(i)
    }
}

impl From<Id<Room>> for Id<Node> {
    fn from(i: Id<Room>) -> Self {
        Id::<Node>::new_from_id(i.id, i.node_suffix)
    }
}

impl<'a> From<&'a Id<Room>> for &'a Id<Node> {
    fn from(i: &'a Id<Room>) -> Self {
        Id::<Node>::as_ref(i)
    }
}

impl From<Id<RoomMember>> for Id<Node> {
    fn from(i: Id<RoomMember>) -> Self {
        Id::<Node>::new_from_id(i.id, i.node_suffix)
    }
}

impl<'a> From<&'a Id<RoomMember>> for &'a Id<Node> {
    fn from(i: &'a Id<RoomMember>) -> Self {
       Id::<Node>::as_ref(i)
    }
}*/

fn parse_id_parts(id: String) -> Result<(String, String), anyhow::Error> { // id, suffix
    let mut split = id.split("$");
    let id = split.next().ok_or(anyhow::anyhow!("Missing part 1 (id) for id"))?.to_string();
    let suffix = split.next().ok_or(anyhow::anyhow!("Missing part 2 (suffix) for id"))?.to_string();
    Ok((id, suffix))
}