use crate::types::id::IdType;

pub mod room;
pub mod user;

pub trait Model: Send + Sync + Clone {
    type Id: IdType;
    fn id(&self) -> &Self::Id;
}
