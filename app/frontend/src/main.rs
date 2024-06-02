extern crate symracing_virtual_mentor_frontend_lib as frontend_lib;

use frontend_lib::infrastructure::components::app::App;
use frontend_lib::infrastructure::settings::Settings;

fn main() {
    let settings = Settings::load().unwrap(); // TODO: Revisar
    yew::Renderer::<App>::new().render();
}
