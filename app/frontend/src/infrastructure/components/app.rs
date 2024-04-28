use yew::prelude::*;
use shared::telemetry::domain::repository::Repository;
use crate::infrastructure::components::scatter_plot::ScatterPlot;
use shared::telemetry::infrastructure::repository::in_memory::InMemory;


#[function_component(App)]
pub fn app() -> Html {
    let repository = InMemory::default();
    //let use_state_handle: UseStateHandle<Option<Session>> = use_state(|| None);
    //use_effect(move || {
    //    use_state_handle.set(
    //        repository.find_by_id(String::from("00000"))
    //    );
    //});

    html! {
        <main>
            <ScatterPlot session={repository.find_by_id(String::from("00000"))}/>
            //<ScatterPlot session={None}/>
        </main>
    }
}