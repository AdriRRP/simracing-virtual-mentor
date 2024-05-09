pub mod front;
pub mod in_car_dials;
pub mod left_front;
pub mod left_rear;
pub mod rear;
pub mod right_front;
pub mod right_rear;

use serde::Deserialize;

use crate::ibt::domain::file::session_info::car_setup::chassis::front::Front;
use crate::ibt::domain::file::session_info::car_setup::chassis::in_car_dials::InCarDials;
use crate::ibt::domain::file::session_info::car_setup::chassis::left_front::LeftFront;
use crate::ibt::domain::file::session_info::car_setup::chassis::left_rear::LeftRear;
use crate::ibt::domain::file::session_info::car_setup::chassis::rear::Rear;
use crate::ibt::domain::file::session_info::car_setup::chassis::right_front::RightFront;
use crate::ibt::domain::file::session_info::car_setup::chassis::right_rear::RightRear;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Chassis {
    pub front: Option<Front>,
    pub left_front: Option<LeftFront>,
    pub left_rear: Option<LeftRear>,
    pub in_car_dials: Option<InCarDials>,
    pub right_front: Option<RightFront>,
    pub right_rear: Option<RightRear>,
    pub rear: Option<Rear>,
}
