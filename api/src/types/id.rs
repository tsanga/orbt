use crate::model::Model;
use std::{hash::Hash, fmt::Display};

use async_graphql::{InputValueError, Scalar, ScalarType, Value};
use serde::{Deserialize, Serialize};

pub trait IdType:
    Send + Sync + ToString + PartialEq + Eq + Hash + Sized + Clone + std::fmt::Debug
{
    type Error: std::error::Error;
    fn new() -> Self;
    fn from_str(id: impl ToString) -> Result<Self, Self::Error>;
}

#[derive(Debug, Clone, Eq, Hash)]
pub struct Id<M: Model>(pub <M as Model>::Id);

impl<M: Model> Id<M> {
    pub fn new() -> Self {
        Self(<M as Model>::Id::new())
    }
    pub fn from_str(id: impl ToString) -> Result<Self, <<M as Model>::Id as IdType>::Error> {
        Ok(Self(<M as Model>::Id::from_str(id)?))
    }
    pub fn from_model_id(id: <M as Model>::Id) -> Self {
        Self(id)
    }
}

impl<M: Model> PartialEq for Id<M> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[Scalar]
impl<M: Model> ScalarType for Id<M> {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let Value::String(value_str) = &value {
            if let Ok(id) = Self::from_str(value_str) {
                Ok(id)
            } else {
                Err(InputValueError::expected_type(value))
            }
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> async_graphql::Value {
        Value::String(self.0.to_string())
    }
}

impl<M: Model> Serialize for Id<M> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
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
        Ok(Id::from_str(value).map_err(|_| E::custom("invalid id"))?)
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
    fn to_id(&self) -> Result<<M as Model>::Id, <<M as Model>::Id as IdType>::Error>;
}

impl<M: Model> ToId<M> for Id<M> {
    fn to_id(&self) -> Result<<M as Model>::Id, <<M as Model>::Id as IdType>::Error> {
        Ok(self.0.clone())
    }
}

impl<M: Model> ToId<M> for &Id<M> {
    fn to_id(&self) -> Result<<M as Model>::Id, <<M as Model>::Id as IdType>::Error> {
        Ok(self.0.clone())
    }
}

impl<M: Model> ToId<M> for String {
    fn to_id(&self) -> Result<<M as Model>::Id, <<M as Model>::Id as IdType>::Error> {
        <M as Model>::Id::from_str(&self)
    }
}

impl<M: Model> ToId<M> for &str {
    fn to_id(&self) -> Result<<M as Model>::Id, <<M as Model>::Id as IdType>::Error> {
        <M as Model>::Id::from_str(&self)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct UuidId(pub String);

impl Display for UuidId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl IdType for UuidId {
    type Error = std::convert::Infallible;
    fn new() -> Self {
        let uuid_str = uuid::Uuid::new_v4().to_string().replace("-", "")[0..6].to_string();
        Self(uuid_str)
    }
    fn from_str(id: impl ToString) -> Result<Self, Self::Error> {
        Ok(Self(id.to_string()))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct NumId(pub u32);

impl NumId {
    pub fn from_u32(id: u32) -> Self {
        Self(id)
    }
}

impl Display for NumId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl IdType for NumId {
    type Error = std::num::ParseIntError;
    fn new() -> Self {
        Self(0)
    }
    fn from_str(id: impl ToString) -> Result<Self, Self::Error> {
        Ok(Self(id.to_string().parse()?))
    }
}