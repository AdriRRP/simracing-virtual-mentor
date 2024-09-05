use crate::infrastructure::components::navbar::NavBar;
use crate::infrastructure::components::repository_context::Repositories;
use crate::infrastructure::components::routes::switch;
use crate::infrastructure::components::routes::Route;

use yew::prelude::*;
use yew_router::BrowserRouter;
use yew_router::Switch;

#[function_component(App)]
pub fn app() -> Html {
    let ctx = use_state(Repositories::default);
    html! {
        <ContextProvider<Repositories> context={(*ctx).clone()}>
            <BrowserRouter>
                <NavBar />
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<Repositories>>
    }
}
