use crate::shared::domain::event::subscriber::Subscriber;

use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

pub struct Manager {
    subscribers: Vec<Arc<Mutex<dyn Subscriber>>>,
    handlers: Mutex<HashMap<String, JoinHandle<()>>>,
}

impl Manager {
    #[must_use]
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub async fn run(&self) {
        for subscriber in &self.subscribers {
            let subscriber_clone = Arc::clone(subscriber);
            let handler = tokio::spawn(async move {
                subscriber_clone.lock().await.run().await;
            });
            // TODO: Add more accurate id
            let id: String = format!("{:?}", subscriber.type_id());
            self.handlers.lock().await.insert(id, handler);
        }
    }
}

#[derive(Default)]
pub struct Builder {
    subscribers: Vec<Arc<Mutex<dyn Subscriber>>>,
}

impl Builder {
    #[must_use]
    pub fn build(self) -> Manager {
        Manager {
            subscribers: self.subscribers,
            handlers: Mutex::new(HashMap::default()),
        }
    }

    #[must_use]
    pub fn with_subscriber(mut self, subscriber: Arc<Mutex<dyn Subscriber>>) -> Self {
        self.subscribers.push(subscriber);
        self
    }
}
