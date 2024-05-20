use crate::api::domain::event::ibt_parsed::IbtParsed;
use crate::shared::domain::event::Event;
use crate::shared::domain::event::subscriber::Subscriber;

use std::sync::Arc;
use async_trait::async_trait;

struct X {}

#[async_trait]
impl Subscriber for X {
    async fn receive(&self, event: Arc<dyn Event>) {
        match event.as_any().downcast_ref::<IbtParsed>() {
            Some(ibt_parsed) => println!("{ibt_parsed:?}"),
            None =>  println!("Can't downcast"),
        }
    }
}