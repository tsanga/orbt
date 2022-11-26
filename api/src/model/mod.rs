use crate::types::id::IdType;

pub mod user;
pub mod room;

pub trait Model: Clone {
    type Id: IdType;
    fn id(&self) -> &Self::Id;
}