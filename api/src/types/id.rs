use crate::{model::Model, schema::Node};
use std::{fmt::Display, hash::Hash, marker::PhantomData, any::Any};

use async_graphql::{InputValueError, Scalar, ScalarType, Value};
use serde::{Deserialize, Serialize};

pub trait Identifiable: Sized + Send + Sync + 'static {
    const MODEL_IDENT: &'static str;
}

#[derive(Debug)]
pub struct Id<M: Any> {
    pub id: String,
    pub model_ident: String,
    _phantom: PhantomData<M>
}

impl<M: Any> Id<M> {
    pub fn new_from_id(id: String, model_ident: String) -> Self {
        Self { id, model_ident, _phantom: PhantomData }
    }
}

impl<M: Identifiable> Id<M> {
    pub fn new() -> Self {
        let uid = uuid::Uuid::new_v4().to_string().replace("-", "")[..6].to_string();
        Self::new_from_id(uid, M::MODEL_IDENT.to_string())
    }

    pub fn from_str(id: impl ToString) -> Self {
        let id = id.to_string();
        if id.contains("$") {
            if let Ok((id, model_ident)) = parse_id(id.clone()) {
                return Self::new_from_id(id, model_ident)
            }
        }
        Self::new_from_id(id, M::MODEL_IDENT.to_string())
    }

    pub fn try_from_str(id: impl ToString) -> Option<Self> {
        let id = id.to_string();
        if id.contains("$") {
            if let Ok((id, model_ident)) = parse_id(id.clone()) {
                if model_ident == M::MODEL_IDENT || M::MODEL_IDENT == "n" {
                    return Some(Self::new_from_id(id, model_ident))
                }
            }
        }
        None
    }
}

impl<M: Any> Display for Id<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}${}", self.id, self.model_ident)
    }
}

impl<M: Any> PartialEq for Id<M> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.model_ident == other.model_ident
    }
}

impl<M: Any> Eq for Id<M> {}

impl<M: Any> Clone for Id<M> {
    fn clone(&self) -> Self {
        Self::new_from_id(self.id.clone(), self.model_ident.clone())
    }
}

impl<M: Any> Hash for Id<M> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.model_ident.hash(state);
    }
}

#[Scalar]
impl<M: Identifiable> ScalarType for Id<M> {
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

impl<M: Identifiable> Serialize for Id<M> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct IdVisitor<M: Identifiable>(std::marker::PhantomData<M>);

impl<'de, M: Identifiable> serde::de::Visitor<'de> for IdVisitor<M> {
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

pub trait ToId<M: Identifiable> {
    fn to_id(&self) -> Option<Id<M>>;
}

impl<M: Identifiable> ToId<M> for Id<M> {
    fn to_id(&self) -> Option<Id<M>> {
        Some(self.clone())
    }
}

impl<M: Identifiable> ToId<M> for &Id<M> {
    fn to_id(&self) -> Option<Id<M>> {
        (*self).to_id()
    }
}

impl<M: Identifiable> ToId<M> for String {
    fn to_id(&self) -> Option<Id<M>> {
        Id::<M>::try_from_str(&self)
    }
}

impl<M: Identifiable> ToId<M> for &str {
    fn to_id(&self) -> Option<Id<M>> {
        Id::<M>::try_from_str(&self)
    }
}

impl<M: Identifiable> From<Id<M>> for String {
    fn from(id: Id<M>) -> Self {
        id.to_string()
    }
}

impl<M: Identifiable> From<&Id<M>> for String {
    fn from(id: &Id<M>) -> Self {
        id.to_string()
    }
}

impl<M: Identifiable> From<String> for Id<M> {
    fn from(s: String) -> Self {
        Id::<M>::from_str(s)
    }
}

impl<M: Model> From<Id<Node>> for Id<M> {
    fn from(i: Id<Node>) -> Self {
        Id::new_from_id(i.id, i.model_ident)
    }
}

impl<M: Model> From<Id<M>> for Id<Node> {
    fn from(i: Id<M>) -> Self {
        Id::new_from_id(i.id, i.model_ident)
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