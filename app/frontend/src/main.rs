//mod application;
//mod domain;
pub mod infrastructure {
    pub mod components {
        pub mod app;
        pub mod scatter_plot;
    }
}

use crate::infrastructure::components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}