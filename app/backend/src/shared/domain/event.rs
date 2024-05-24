pub mod bus;
pub mod subscriber;

use std::any::Any;
use std::fmt::Debug;
use tracing_subscriber::fmt::format;

pub trait Event: Debug + Send + Sync + 'static {
    fn event_id() -> &'static str
    where
        Self: Sized{
        std::any::type_name::<Self>()
    }
    
    fn id(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn as_any(&self) -> &dyn Any;
}
