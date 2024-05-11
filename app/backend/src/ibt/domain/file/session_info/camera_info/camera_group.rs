pub mod camera;

use crate::ibt::domain::file::session_info::camera_info::camera_group::camera::Camera;
use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CameraGroup {
    pub group_num: Option<i32>,
    pub group_name: Option<String>,
    pub cameras: Option<Vec<Camera>>,
}
