use std::{
    any::{Any, TypeId},
    collections::HashMap,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures_util::{Stream, StreamExt};
use slab::Slab;

use crate::{
    model::{room::Room, user::User},
    store::DataStore,
    types::id::Id,
};

pub struct StreamSenders<T: Send + Sync + Clone + 'static>(Slab<UnboundedSender<T>>);

#[derive(Clone)]
pub struct StreamController {
    subscribers: Arc<Mutex<HashMap<TypeId, Box<dyn Any + Send>>>>,
}

impl StreamController {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn publish<T: Send + Sync + Clone + 'static>(&self, msg: T) {
        with_senders::<T, _, _>(&self, |senders| {
            for (_, sender) in senders.0.iter_mut() {
                sender.start_send(msg.clone()).ok();
            }
        });
    }

    pub fn subscribe<'a, T, F>(
        &'a self,
        user: &Id<User>,
        room: &Id<Room>,
        user_store: DataStore<User>,
        room_store: DataStore<Room>,
        on_drop: F,
    ) -> impl Stream<Item = T> + 'a
    where
        T: Send + Sync + Clone + 'static,
        F: FnOnce(&UserStreamData) + 'a,
    {
        with_senders::<T, _, _>(&self, |senders| {
            let (sender, receiver) = mpsc::unbounded();
            let id = senders.0.insert(sender);
            UserStreamSubscriber::<T, F>::new(
                id, receiver, user, room, user_store, room_store, on_drop, &self,
            )
        })
    }
}

fn with_senders<T, F, R>(ctl: &StreamController, func: F) -> R
where
    T: Send + Sync + Clone + 'static,
    F: FnOnce(&mut StreamSenders<T>) -> R,
{
    let mut map = ctl.subscribers.lock().unwrap();
    let senders = map
        .entry(TypeId::of::<StreamSenders<T>>())
        .or_insert_with(|| Box::new(StreamSenders::<T>(Default::default())));
    func(senders.downcast_mut::<StreamSenders<T>>().unwrap())
}

pub struct UserStreamData {
    pub user: Id<User>,
    pub room: Id<Room>,
    pub user_store: DataStore<User>,
    pub room_store: DataStore<Room>,
}

impl UserStreamData {
    fn new(
        user: Id<User>,
        room: Id<Room>,
        user_store: DataStore<User>,
        room_store: DataStore<Room>,
    ) -> Self {
        Self {
            user,
            room,
            user_store,
            room_store,
        }
    }
}

pub struct UserStreamSubscriber<
    'a,
    T: Send + Sync + Clone + 'static,
    F: FnOnce(&UserStreamData),
> {
    id: usize,
    receiver: UnboundedReceiver<T>,
    on_drop: Option<Box<F>>,
    data: UserStreamData,
    ctl: &'a StreamController,
}

impl<'a, T: Send + Sync + Clone + 'static, F: FnOnce(&UserStreamData)>
    UserStreamSubscriber<'a, T, F>
{
    pub fn new(
        id: usize,
        receiver: UnboundedReceiver<T>,
        user: &Id<User>,
        room: &Id<Room>,
        user_store: DataStore<User>,
        room_store: DataStore<Room>,
        on_drop: F,
        ctl: &'a StreamController,
    ) -> Self {
        let on_drop = Some(Box::new(on_drop));
        let data = UserStreamData::new(user.clone(), room.clone(), user_store, room_store);
        Self {
            id,
            receiver,
            on_drop,
            data,
            ctl,
        }
    }
}

impl<'a, T: Send + Sync + Clone + 'static, F: FnOnce(&UserStreamData)> Stream
    for UserStreamSubscriber<'a, T, F>
{
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_next_unpin(cx)
    }
}

impl<'a, T: Send + Sync + Clone + 'static, F: FnOnce(&UserStreamData)> Drop
    for UserStreamSubscriber<'a, T, F>
{
    fn drop(&mut self) {
        if let Some(on_drop) = self.on_drop.take() {
            on_drop(&self.data);
        }
        with_senders::<T, _, _>(&self.ctl, |senders| senders.0.remove(self.id));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        model::{room::RoomChatMsg, user::User},
        store::DataStore,
        types::time::Time,
    };

    #[tokio::test]
    async fn stream_subscribe_and_publish() {
        let user_store = DataStore::<User>::new();
        let room_store = DataStore::<Room>::new();
        let ctl = StreamController::new();

        let user = User::new("jonah".to_string());
        let user_id = user.id.clone();
        let mut room = Room::new("test".to_string());
        let room_id = room.id.clone();
        room.init_owner(&user);
        room.join(&user, None).unwrap();
        room_store.insert(room);
        user_store.insert(user);

        let sub_ctl = ctl.clone();
        let sub_user_id = user_id.clone();
        let sub_room_id = room_id.clone();
        let sub_task = tokio::spawn(async move {
            //println!("subscribing...");
            let mut stream = sub_ctl.subscribe::<RoomChatMsg, _>(
                &sub_user_id,
                &sub_room_id,
                user_store.clone(),
                room_store.clone(),
                move |data| {
                    let room = &mut data.room_store.get(&data.room).unwrap().unwrap();
                    let user = &data.user_store.get(&data.user).unwrap().unwrap();
                    room.leave(&user.id).unwrap();
                    //println!("dropped");
                    assert!(!room.is_member(&user.id))
                },
            );
            //println!("subscribed");

            while let Some(msg) = &stream.next().await {
                //println!("{:?}", msg);
                assert_eq!(msg.msg, "hello".to_string());
                assert_eq!(&msg.room, &sub_room_id);
                assert_eq!(&msg.author, &sub_user_id);
                break; // abort after 1 message
            }
            //println!("done");
        });

        let pub_task = tokio::spawn(async move {
            ctl.publish(RoomChatMsg::new(
                Id::new(),
                room_id.clone(),
                user_id.clone(),
                "hello".to_string(),
                Time::now(),
            ));
            //println!("published!");
        });

        pub_task.await.unwrap();
        sub_task.await.unwrap();
    }
}
