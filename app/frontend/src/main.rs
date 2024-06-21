use symracing_virtual_mentor_frontend_lib::infrastructure::components::app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}