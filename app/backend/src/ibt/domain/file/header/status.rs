#[derive(PartialEq, Eq, Debug)]
pub enum Status {
    Connected,
    Unknown,
}

impl From<i32> for Status {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Connected,
            _ => Self::Unknown,
        }
    }
}
