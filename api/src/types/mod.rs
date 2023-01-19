pub mod color;
pub mod time;
pub mod token;

use async_graphql::*;
use musty::prelude::*;

struct IdScalar<M: Model>(Id<M>);

#[Scalar]
impl<M: Model> ScalarType for IdScalar<M>{
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        let json = value.into_json().unwrap();
        let id = serde_json::from_value(json).unwrap();
        Ok(IdScalar(id))
    }

    fn to_value(&self) -> async_graphql::Value {
        let json = serde_json::to_value(&self.0).unwrap();
        Value::from_json(json).unwrap()
    }
}