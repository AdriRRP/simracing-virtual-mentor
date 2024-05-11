use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RadioFrequency {
    pub frequency_num: Option<i32>,
    pub frequency_name: Option<String>,
    pub priority: Option<i32>,
    pub car_idx: Option<i32>,
    pub entry_idx: Option<i32>,
    #[serde(rename = "ClubID")]
    pub club_id: Option<i32>,
    pub can_scan: Option<i32>,
    pub can_squawk: Option<i32>,
    pub muted: Option<i32>,
    pub is_mutable: Option<i32>,
    pub is_deletable: Option<i32>,
}
