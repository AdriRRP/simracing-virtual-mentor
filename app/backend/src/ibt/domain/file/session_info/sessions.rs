pub mod session;

use crate::ibt::domain::file::session_info::sessions::session::Session;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Sessions {
    pub sessions: Option<Vec<Session>>,
}
