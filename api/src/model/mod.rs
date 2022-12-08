use crate::types::id::{Id, Identifiable};

pub mod room;
pub mod user;

pub trait Model: Send + Sync + Clone + Identifiable {
    fn model_id(&self) -> &Id<Self>;
}
