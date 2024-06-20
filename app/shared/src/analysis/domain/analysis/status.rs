use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub enum Status {
    Pending { ref_id: Uuid, target_id: Uuid },
    Error(String),
    Completed,
}
