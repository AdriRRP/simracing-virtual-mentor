extern crate symracing_virtual_mentor_frontend_lib as frontend_lib;

use frontend_lib::infrastructure::components::app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
