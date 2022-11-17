use async_graphql::scalar;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct Time(pub u128);

impl Time {
    pub fn now() -> Self {
        Self(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis())
    }

    pub fn from(time: u128) -> Self {
        Self(time)
    }
}

scalar!(Time);