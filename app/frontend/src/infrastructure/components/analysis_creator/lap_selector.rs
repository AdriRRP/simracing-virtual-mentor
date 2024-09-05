use crate::infrastructure::components::laps::list::LapListComponent;
use crate::infrastructure::components::repository_context::Repositories;
use crate::infrastructure::repository::analysis::http::Request;

use shared::lap::domain::lap::header::Header as Lap;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{function_component, html, Html};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub reference_lap: Option<Lap>,
    #[prop_or_default]
    pub target_lap: Option<Lap>,
}

pub enum Msg {
    ShowModal,
    HideModal,
    FetchLaps,
}

#[function_component(LapSelector)]
pub fn lap_selector(props: &Props) -> Html {
    let ref_lap = props.reference_lap.clone();
    let target_lap = props.target_lap.clone();

    let repositories = use_context::<Repositories>().expect("no Repository ctx found");
    let analysis_repo = repositories.analysis;

    let request = use_state(|| Option::<Request>::None);
    let name = use_state(String::new);
    let modal_content = use_state(String::new);
    let show_modal = use_state(|| false);

    let on_name_change = {
        let name = name.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            name.set(value);
        })
    };

    let create_lap = {
        let modal_content = modal_content.clone();
        let show_modal = show_modal.clone();

        Callback::from(move |_: MouseEvent| {
            if let (Some(ref_lap), Some(target_lap)) = (&ref_lap, &target_lap) {
                if ref_lap.circuit != target_lap.circuit {
                    modal_content.set("The selected laps are from different circuits.".to_string());
                    show_modal.set(true);
                } else if name.is_empty() {
                    modal_content.set("No name provided for the analysis.".to_string());
                    show_modal.set(true);
                } else {
                    let new_request = Request::new((*name).clone(), ref_lap.id, target_lap.id);
                    request.set(Some(new_request.clone()));

                    let modal_content = modal_content.clone();
                    let analysis_repo = analysis_repo.clone();
                    let name = name.clone();
                    let show_modal = show_modal.clone();

                    wasm_bindgen_futures::spawn_local(async move {
                        let result = analysis_repo.create(new_request).await;
                        match result {
                            Ok(()) => {
                                modal_content.set("Analysis created successfully!".to_string());
                                name.set(String::new());
                                //ref_lap.set(None);
                                //target_lap.set(None);
                            }
                            Err(e) => {
                                modal_content.set(format!("Failed to create analysis: {e}"));
                            }
                        }
                        show_modal.set(true); // Mostrar modal después de la operación asíncrona
                    });
                }
            } else {
                modal_content.set("You must select a reference lap and a target lap in order to create the analysis.".to_string());
                show_modal.set(true); // Mostrar modal si faltan vueltas
            }
        })
    };

    let close_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_| {
            show_modal.set(false);
        })
    };

    html! {
        <div class="box mt-4">
            <h2 class="subtitle is-5">{"Select Laps to compare"}</h2>
            <div class="content">
                <label class="label">{"Analysis Name"}</label>
                <div class="control">
                    <input
                        class="input"
                        type="text"
                        placeholder="Name of the analysis..."
                        onchange={on_name_change}
                    />
                </div>
            </div>
            <div class="grid">
                {draw_lap(props.reference_lap.clone(), "Reference Lap", "🏆")}
                {draw_lap(props.target_lap.clone(), "Target Lap", "🫥")}
                <div class="cell">
                    <div class="has-text-centered">
                        <button
                            class="button is-primary is-large"
                            onclick={create_lap}
                        >{"Create Analysis"}</button>
                    </div>
                </div>
            </div>

            <div class={classes!(if *show_modal { "modal is-active" } else { "modal" })}>
                <div class="modal-background" onclick={close_modal.clone()}></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{"Create Analysis"}</p>
                        <button class="delete" aria-label="close" onclick={close_modal.clone()}></button>
                    </header>
                    <section class="modal-card-body">
                        {(*modal_content).clone()}
                    </section>
                    <footer class="modal-card-foot">
                        <button class="button" onclick={close_modal.clone()}>{"OK"}</button>
                    </footer>
                </div>
            </div>
        </div>
    }
}

fn draw_lap(opt_lap: Option<Lap>, lap_type: &str, icon: &str) -> Html {
    opt_lap.map_or_else(
        || html!{
            <div class="cell">
                <label class="label">{format!("{icon} {lap_type}")}</label>
                <div class="box">
                    <div class="has-text-centered">
                        <p class="is-primary is-large">{format!("Select a {lap_type} using {icon} button")}</p>
                    </div>
                </div>
            </div>
        },
        |lap| {
        html!{
            <div class="cell">
                <article class="media is-hoverable">
                    <figure class="media-left">
                        <p class="image is-64x64">
                            <h1 class="title is-1 is-center">{icon}</h1>
                        </p>
                    </figure>
                    <div class="media-content">
                        <div class="content">
                            <p>
                                <p class="title is-6">{LapListComponent::lap_name(&lap)}</p>
                                <small class="subtitle is-6">{lap.circuit.clone()}</small>
                                <br/>
                                <small><b>{"Car: "}</b>{lap.car.clone()}</small>
                                <small><b>{" | Category: "}</b>{lap.category.clone()}</small>
                                <small><b>{" | Date: "}</b>{lap.date.to_string()}</small>
                            </p>
                        </div>
                    </div>
                </article>
            </div>
        }
    })
}
