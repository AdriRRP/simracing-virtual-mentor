use crate::shared::domain::event::subscriber::Subscriber;

use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinHandle;

pub struct Manager {
    subscribers: Vec<Arc<dyn Subscriber>>,
    handlers: HashMap<String, JoinHandle<()>>,
}

impl Manager {
    #[must_use]
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn run(&mut self) {
        for subscriber in &self.subscribers {
            let id: String = format!("{:?}", subscriber.type_id());
            let subscriber = Arc::clone(subscriber);
            let handler = tokio::spawn(async move {
                subscriber.run().await;
            });
            // TODO: Add more accurate id
            self.handlers.insert(id, handler);
        }
    }
}

#[derive(Default)]
pub struct Builder {
    subscribers: Vec<Arc<dyn Subscriber>>,
}

impl Builder {
    #[must_use]
    pub fn build(self) -> Manager {
        Manager {
            subscribers: self.subscribers,
            handlers: HashMap::default(),
        }
    }

    #[must_use]
    pub fn with_subscriber(mut self, subscriber: Arc<dyn Subscriber>) -> Self {
        self.subscribers.push(subscriber);
        self
    }
}
