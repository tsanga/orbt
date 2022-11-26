use std::hash::Hash;

use async_graphql::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, Display)]
pub struct Id(String);

impl Default for Id {
    fn default() -> Self {
        Id(Uuid::new_v4().to_string())
    }
}

#[Scalar]
impl ScalarType for Id {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            Ok(value.parse().map(Id)?)
        } else {
            // If the type does not match
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
