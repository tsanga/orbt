use super::actor::Actor;

pub trait Action<T>: ToString {
    fn can_act(&self, actor: &Actor, t: T) -> bool;
}