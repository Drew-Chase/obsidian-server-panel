use crate::data::{EventManager, Subscriber};
use std::ops::{AddAssign, SubAssign};

pub mod data;

impl<S: Clone> Default for EventManager<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Clone> EventManager<S> {
    pub fn new() -> Self {
        EventManager {
            data: Vec::new(),
            subscribers: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, subscriber: &Subscriber<S>) {
        //        self.subscribers.push(&*subscriber);
    }

    pub fn unsubscribe(&mut self, id: u32) {
        self.subscribers.retain(|subscriber| subscriber.id != id);
    }

    pub fn publish(&self, data: S) {
        for subscriber in self.subscribers.iter() {
            (subscriber.callback)(subscriber, data.clone());
        }
    }
}

impl AddAssign for EventManager<u32> {
    fn add_assign(&mut self, other: Self) {
        self.subscribe(&other.subscribers[0]);
    }
}
impl SubAssign for EventManager<u32> {
    fn sub_assign(&mut self, other: Self) {
        self.unsubscribe(other.subscribers[0].id);
    }
}
