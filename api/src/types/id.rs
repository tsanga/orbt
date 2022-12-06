use crate::model::Model;
use std::{fmt::Display, hash::Hash, marker::PhantomData};

use async_graphql::{InputValueError, Scalar, ScalarType, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Id<M: Model>(String, PhantomData<M>);

impl<M: Model> Id<M> {
    pub fn new() -> Self {
        let uid = uuid::Uuid::new_v4().to_string().replace("-", "")[..6].to_string();
        Self(uid, PhantomData)
    }

    pub fn from_str(id: impl ToString) -> Option<Self> {
        let id = id.to_string();
        if id.contains("$") {
            let (prefix, suffix) = parse_id(id).ok()?;
            if suffix == M::ID_SUFFIX {
                Some(Self(prefix, PhantomData))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<M: Model> Display for Id<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffix = <M as Model>::ID_SUFFIX;
        write!(f, "{}${}", self.0, suffix)
    }
}

impl<M: Model> PartialEq for Id<M> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<M: Model> Eq for Id<M> {}

impl<M: Model> Clone for Id<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<M: Model> Hash for Id<M> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[Scalar]
impl<M: Model> ScalarType for Id<M> {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let Value::String(value_str) = &value {
            if let Some(id) = Self::from_str(value_str) {
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
        Id::from_str(value).ok_or(E::custom("invalid id"))
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
        Id::<M>::from_str(&self)
    }
}

impl<M: Model> ToId<M> for &str {
    fn to_id(&self) -> Option<Id<M>> {
        Id::<M>::from_str(&self)
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct UuidId(pub String);

impl Display for UuidId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn parse_id(id: String) -> Result<(String, String), anyhow::Error> { // id, suffix
    let mut split = id.split("$");
    let id = split.next().ok_or(anyhow::anyhow!("Missing part 1 (id) for id"))?.to_string();
    let suffix = split.next().ok_or(anyhow::anyhow!("Missing part 2 (suffix) for id"))?.to_string();
    Ok((id, suffix))
}