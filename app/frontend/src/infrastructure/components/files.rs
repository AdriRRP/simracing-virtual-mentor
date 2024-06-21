pub mod file_lister;
pub mod file_uploader;

use crate::infrastructure::components::files::file_lister::FileLister;
use crate::infrastructure::components::files::file_uploader::FileUploader;
use gloo::console::info;

use yew::prelude::*;

pub enum Msg {
    Reload,
}
pub struct Files {}

impl Default for Files {
    fn default() -> Self {
        Self {}
    }
}

impl Component for Files {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reload => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        info!("Entering files view!");
        let reload = ctx.link().callback(|()| Msg::Reload);
        html! {
            <div class="container">
                <FileUploader notify_parent={reload} />
                <FileLister />
            </div>
        }
    }
}

//#[function_component(Files)]
//pub fn files() -> Html {
//    let trigger = use_force_update();
//
//    let reload =
//        Callback::from(move |()| {
//            info!("Trying to update");
//            trigger.force_update()
//        });
//
//    html! {
//        <div class="container">
//            <FileUploader notify_parent={reload} />
//            <FileLister />
//        </div>
//    }
//}
