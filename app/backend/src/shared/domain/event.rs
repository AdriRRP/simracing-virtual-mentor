pub mod bus;
pub mod subscriber;

use std::fmt::Debug;

pub trait Event: Debug + Send + Sync + 'static {
    fn id() -> &'static str
    where
        Self: Sized;
}
