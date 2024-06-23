use crate::infrastructure::components::analyses::Analyses;
use crate::infrastructure::components::dashboard::Dashboard;
use crate::infrastructure::components::files::Files;
use crate::infrastructure::components::home::Home;
use crate::infrastructure::components::laps::Laps;

use yew::html;
use yew::Html;
use yew_router::Routable;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/files")]
    Files,
    #[at("/laps")]
    Laps,
    #[at("/analyses")]
    Analyses,
    #[at("/dashboard")]
    Dashboard,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Files => html! { <Files /> },
        Route::Laps => html! { <Laps /> },
        Route::Analyses => html! { <Analyses /> },
        Route::Dashboard => html! { <Dashboard /> },
    }
}
