use yew::prelude::*;
//use crate::infrastructure::components::scatter_plot::ScatterPlot;
//use crate::infrastructure::repository::lap::http::Http;

#[function_component(App)]
pub fn app() -> Html {
    //let repository = Http::default();
    //let use_state_handle: UseStateHandle<Option<Session>> = use_state(|| None);
    //use_effect(move || {
    //    use_state_handle.set(
    //        repository.find_by_id(String::from("00000"))
    //    );
    //});

    html! {
        <main>
            //<ScatterPlot session={repository.find_by_id(String::from("00000"))}/>
            //<ScatterPlot session={None}/>
        </main>
    }
}
