use serde::Deserialize;

#[derive(PartialEq, Eq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Camera {
    pub camera_num: Option<i32>,
    pub camera_name: Option<String>,
}
