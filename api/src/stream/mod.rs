use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures_util::{Stream, StreamExt};

use crate::{model::Model, types::id::Id};

#[derive(Clone)]
pub struct StreamControl<S: Model + 'static, T: Model + 'static> {
    publishers: Arc<Mutex<Vec<ModelPublisher<S, T>>>>,
}

impl<S: Model + 'static, T: Model + 'static> StreamControl<S, T> {
    pub fn new() -> Self {
        Self {
            publishers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn subscribe<Sub: Subscriber<S, T> + 'static>(&self, sub: Sub) -> ModelSubscriber<S, T> {
        let (tx, rx) = mpsc::unbounded::<T>();

        let subscriber_id = sub.subscriber_id();
        let topic_id = sub.topic_id();

        let publisher = ModelPublisher::new(subscriber_id.clone(), topic_id.clone(), tx);
        self.publishers.lock().unwrap().push(publisher);

        ModelSubscriber::new(Box::new(sub), rx, &self)
    }

    pub fn get_publisher(
        &self,
        subscriber_id: &Id<S>,
        topic_id: &Id<T>,
    ) -> Option<ModelPublisher<S, T>> {
        self.publishers
            .lock()
            .unwrap()
            .iter()
            .find(|s| &s.subscriber_id == subscriber_id && &s.topic_id == topic_id)
            .cloned()
    }

    pub fn publish(&self, msg: T) {
        let topic_id = msg.model_id().clone();
        let mut publishers = self.publishers.lock().unwrap();
        for publisher in publishers.iter_mut().filter(|p| p.topic_id == topic_id) {
            publisher.publish(msg.clone());
        }
    }

    pub fn disconnect(&self, subscriber_id: &Id<S>) -> usize {
        let mut count = 0usize;
        let mut publishers = self.publishers.lock().unwrap();
        for publisher in publishers
            .iter()
            .filter(|p| &p.subscriber_id == subscriber_id)
        {
            publisher.disconnect();
            count += 1;
        }
        publishers.retain(|s| &s.subscriber_id != subscriber_id);
        count
    }

    fn handle_disconnect(&self, subscriber_id: &Id<S>) {
        let mut publishers = self.publishers.lock().unwrap();
        publishers.retain(|s| (&s.subscriber_id != subscriber_id));
    }
}

#[derive(Clone)]
pub struct ModelPublisher<S: Model + 'static, T: Model + 'static> {
    subscriber_id: Id<S>,
    topic_id: Id<T>,
    sender: UnboundedSender<T>,
}

impl<S: Model + 'static, T: Model + 'static> ModelPublisher<S, T> {
    fn new(subscriber_id: Id<S>, topic_id: Id<T>, sender: UnboundedSender<T>) -> Self {
        Self {
            subscriber_id,
            topic_id,
            sender,
        }
    }

    fn publish(&mut self, msg: T) {
        self.sender.start_send(msg).ok(); // don't care if send fails, if its fails it means we were disconnected
    }

    fn disconnect(&self) {
        self.sender.close_channel();
    }
}

pub struct ModelSubscriber<'a, S: Model + 'static, T: Model + 'static> {
    subscriber: Box<dyn Subscriber<S, T>>,
    subscriber_id: Id<S>,
    topic_id: Id<T>,
    receiver: UnboundedReceiver<T>,
    ctl: &'a StreamControl<S, T>,
}

impl<'a, S: Model + 'static, T: Model + 'static> Unpin for ModelSubscriber<'a, S, T> {}

impl<'a, S: Model + 'static, T: Model + 'static> ModelSubscriber<'a, S, T> {
    fn new(
        sub: Box<dyn Subscriber<S, T>>,
        receiver: UnboundedReceiver<T>,
        ctl: &'a StreamControl<S, T>,
    ) -> Self {
        let subscriber_id = sub.subscriber_id().clone();
        let topic_id = sub.topic_id().clone();
        Self {
            subscriber: sub,
            subscriber_id,
            topic_id,
            receiver,
            ctl,
        }
    }
}

impl<'a, S: Model + 'static, T: Model + 'static> Stream for ModelSubscriber<'a, S, T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let poll = self.receiver.poll_next_unpin(cx);
        if let Poll::Ready(Some(msg)) = poll {
            let msg = self.subscriber.map_msg(msg);
            return Poll::Ready(msg);
        }
        poll
    }
}

impl<'a, S: Model + 'static, T: Model + 'static> Drop for ModelSubscriber<'a, S, T> {
    fn drop(&mut self) {
        self.subscriber.on_disconnect();
        self.ctl.handle_disconnect(&self.subscriber_id);
    }
}

pub trait Subscriber<S: Model + 'static, T: Model + 'static>: Send + Sync {
    fn subscriber_id(&self) -> &Id<S>;
    fn topic_id(&self) -> &Id<T>;
    fn on_disconnect(&mut self);
    fn map_msg(&self, msg: T) -> Option<T>;
}

#[cfg(test)]
mod tests {
    use crate::{
        model::{room::Room, user::User},
        store::DataStore,
    };

    use super::*;

    #[derive(Clone)]
    struct UserRoomSubscriber {
        room_store: DataStore<Room>,
        user: Id<User>,
        room: Id<Room>,
    }

    impl UserRoomSubscriber {
        fn new(room_store: DataStore<Room>, user: Id<User>, room: Id<Room>) -> Self {
            Self {
                room_store,
                user,
                room,
            }
        }
    }

    impl Subscriber<User, Room> for UserRoomSubscriber {
        fn subscriber_id(&self) -> &Id<User> {
            &self.user
        }

        fn topic_id(&self) -> &Id<Room> {
            &self.room
        }

        fn on_disconnect(&mut self) {
            // intentionally blank
        }

        fn map_msg(&self, msg: Room) -> Option<Room> {
            Some(msg)
        }
    }

    #[tokio::test]
    async fn test_subscribe_publish() {
        // SETUP CONTROL, STORE, MODELS
        let ctl = StreamControl::<User, Room>::new();
        let room_store = DataStore::<Room>::new();
        let room = Room::new("test_room".into());
        let room_id = room.id.clone();
        room_store.insert(room);
        let user = User::new("test_user".into());

        // SUBSCRIBE
        let sub_ctl = ctl.clone();
        let sub_room_store = room_store.clone();
        let sub_user_id = user.id.clone();
        let sub_room_id = room_id.clone();

        let task_subscribe = tokio::spawn(async move {
            // subscribe
            let mut sub_stream = sub_ctl.subscribe(UserRoomSubscriber::new(
                sub_room_store,
                sub_user_id,
                sub_room_id,
            ));

            // wait for msg
            while let Some(msg) = &sub_stream.next().await {
                //println!("Subscriber received msg: {:?}", msg);
                assert_eq!(&msg.name, "new room name");
                break; // abort after 1 msg
            }
        });

        let mut room = room_store.get(&room_id).unwrap();
        room.name = "new room name".into();

        let msg = room.clone();
        let task_publish = tokio::spawn(async move {
            //println!("Publishing... {:?}", msg);
            ctl.publish(msg);
            //println!("Published!");
        });

        task_publish.await.unwrap();
        task_subscribe.await.unwrap();
    }

    #[tokio::test]
    async fn test_subscribe_disconnect() {
        let ctl = StreamControl::<User, Room>::new();
        let room_store = DataStore::<Room>::new();
        let room = Room::new("test_room".into());
        let room_id = room.id.clone();
        room_store.insert(room);
        let user = User::new("test_user".into());

        // SUBSCRIBE
        let sub_ctl = ctl.clone();
        let sub_room_store = room_store.clone();
        let sub_user_id = user.id.clone();
        let sub_room_id = room_id.clone();

        let task_subscribe = tokio::spawn(async move {
            // subscribe
            //println!("subscribing");
            let mut stream = sub_ctl.subscribe(UserRoomSubscriber::new(
                sub_room_store,
                sub_user_id,
                sub_room_id,
            ));
            while let Some(msg) = stream.next().await {
                //println!("Subscriber(1) received msg: {:?}", msg);
                assert_eq!(&msg.name, "new room name");
                break; // abort after 1 msg
            }
        });

        let dis_ctl = ctl.clone();
        let task_disconnect = tokio::spawn(async move {
            //println!("disconnecting");
            let disconnect_count = dis_ctl.disconnect(&user.id);
            //println!("disconnected {}", disconnect_count);
            assert_eq!(disconnect_count, 1);
        });

        let mut room = room_store.get(&room_id).unwrap();
        room.name = "new room name".into();

        let msg = room.clone();
        let task_publish = tokio::spawn(async move {
            //println!("Publishing... {:?}", msg);
            ctl.publish(msg);
            //println!("Published!");
        });

        task_publish.await.unwrap();
        task_subscribe.await.unwrap();
        task_disconnect.await.unwrap();
    }

    #[tokio::test]
    async fn test_subscribe_publish_multiple_subscribers() {
        // SETUP CONTROL, STORE, MODELS
        let ctl = StreamControl::<User, Room>::new();
        let room_store = DataStore::<Room>::new();
        let room = Room::new("test_room".into());
        let room_id = room.id.clone();
        room_store.insert(room);

        // SUBSCRIBE USER 1
        let user1 = User::new("test_user1".into());
        let sub_ctl = ctl.clone();
        let sub_room_store = room_store.clone();
        let sub_user_id = user1.id.clone();
        let sub_room_id = room_id.clone();

        let task_subscribe1 = tokio::spawn(async move {
            // subscribe
            let mut sub_stream = sub_ctl.subscribe(UserRoomSubscriber::new(
                sub_room_store,
                sub_user_id,
                sub_room_id,
            ));

            // wait for msg
            while let Some(msg) = &sub_stream.next().await {
                //println!("Subscriber(1) received msg: {:?}", msg);
                assert_eq!(&msg.name, "new room name");
                break; // abort after 1 msg
            }
        });

        // SUBSCRIBE USER 2
        let user2 = User::new("test_user2".into());
        let sub_ctl = ctl.clone();
        let sub_room_store = room_store.clone();
        let sub_user_id = user2.id.clone();
        let sub_room_id = room_id.clone();

        let task_subscribe2 = tokio::spawn(async move {
            // subscribe
            let mut sub_stream = sub_ctl.subscribe(UserRoomSubscriber::new(
                sub_room_store,
                sub_user_id,
                sub_room_id,
            ));

            // wait for msg
            while let Some(msg) = &sub_stream.next().await {
                //println!("Subscriber(2) received msg: {:?}", msg);
                assert_eq!(&msg.name, "new room name");
                break; // abort after 1 msg
            }
        });

        // GET ROOM
        let mut room = room_store.get(&room_id).unwrap();

        // SET NAME
        room.name = "new room name".into();

        let msg = room.clone();
        // PUBLISH
        let task_publish = tokio::spawn(async move {
            //println!("Publishing...");
            ctl.publish(msg);
            //println!("Published!");
        });

        // AWAIT
        task_publish.await.unwrap();
        let (s1, s2) = futures::join!(task_subscribe1, task_subscribe2);
        s1.unwrap();
        s2.unwrap();
    }
}
