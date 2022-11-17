use std::fmt::Debug;

use super::actor::Actor;

pub trait Action<M>: Debug + Clone {
    fn can_act(&self, actor: &Actor, model: &M) -> bool;
}