use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Time(u128);

impl Time {
    pub fn now() -> Self {
        Self(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis())
    }
}