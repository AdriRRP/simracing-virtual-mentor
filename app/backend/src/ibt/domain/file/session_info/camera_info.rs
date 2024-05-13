pub mod camera_group;

use crate::ibt::domain::file::session_info::camera_info::camera_group::CameraGroup;

use serde::Deserialize;

#[derive(PartialEq, Eq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CameraInfo {
    pub groups: Option<Vec<CameraGroup>>,
}
