use std::any::*;
use std::boxed::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::mpsc::{self, Receiver, Sender};

pub struct Listener<M: 'static + Message + Clone + Send> {
    inbox: Receiver<Box<dyn Any + Send>>,
    phantom: PhantomData<M>,
}

impl<M: 'static + Message + Clone + Send> Listener<M> {
    pub fn fetch(&self) -> Vec<M> {
        self.inbox
            .try_iter()
            .map(|msg| *msg.downcast::<M>().unwrap())
            .collect()
    }
}

#[derive(Default)]
pub struct MessageBus {
    listeners: HashMap<TypeId, Vec<Sender<Box<dyn Any + Send>>>>,
}

impl MessageBus {
    pub fn new() -> Self {
        MessageBus {
            listeners: HashMap::new(),
        }
    }

    pub fn add_listener<T: 'static + Message + Clone + Send>(&mut self) -> Listener<T> {
        let (tx, rx) = mpsc::channel();
        let listeners_list = self
            .listeners
            .entry(TypeId::of::<T>())
            .or_insert(Vec::new());
        listeners_list.push(tx.clone());

        Listener {
            inbox: rx,
            phantom: PhantomData,
        }
    }
    pub fn notify<T: 'static + Message + Clone + Send>(&self, message: T) {
        let outbox = match self.listeners.get(&TypeId::of::<T>()) {
            Some(o) => o,
            None => return,
        };

        for subscriber in outbox {
            subscriber.send(Box::new(message.clone())).unwrap();
        }
    }
}

pub trait Message {
    fn log_entry(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Debug)]
    struct TestMessage1 {
        val: i32,
    }

    impl Message for TestMessage1 {
        fn log_entry(&self) -> String {
            format!("Test message 1, contains: {:?}.", self.val)
        }
    }

    #[derive(Clone, PartialEq, Debug)]
    struct TestMessage2 {
        val: String,
    }

    impl Message for TestMessage2 {
        fn log_entry(&self) -> String {
            format!("Test message 2, contains: {:?}.", self.val)
        }
    }

    #[test]
    fn message_passing() {
        let mut bus = MessageBus::new();
        let listener1 = bus.add_listener::<TestMessage1>();
        let listener2 = bus.add_listener::<TestMessage2>();

        let msg1 = TestMessage1 { val: 5 };
        let msg2 = TestMessage2 {
            val: "test string".to_owned(),
        };

        bus.notify(msg1.clone());

        assert!(listener2.fetch().is_empty());
        assert_eq!(listener1.fetch(), vec![msg1]);

        bus.notify(msg2.clone());

        assert!(listener1.fetch().is_empty());
        assert_eq!(listener2.fetch(), vec![msg2]);
    }

    #[test]
    fn async_messaging() {
        let mut bus = MessageBus::new();
        let listener = bus.add_listener::<TestMessage1>();
        std::thread::spawn(move || loop {
            let messages = listener.fetch();
            let mut i = 0;
            messages.into_iter().for_each(|msg| {
                println!("{:?}", msg.log_entry());
                assert_eq!(msg, TestMessage1 { val: i });
                i += 1;
            });
        });

        for i in 0..5 {
            let msg = TestMessage1 { val: i };
            bus.notify(msg);
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

}
