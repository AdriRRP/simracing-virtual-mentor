pub mod status;

use crate::file::domain::file::status::Status;

use uuid::Uuid;

#[derive(Clone)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub status: Status,
}
