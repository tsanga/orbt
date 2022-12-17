use crate::types::id::Id;

pub mod room;
pub mod user;

pub trait Model: Send + Sync + Clone + 'static {
    const NODE_SUFFIX: &'static str;
    fn model_id(&self) -> &Id<Self>;
}