use chrono::{DateTime, Utc};
use config::ValueKind::String;
use log::{debug, error, info};
use shared::lap::domain::lap::header::Header as Lap;
use shared::common::domain::criteria::Criteria;

use yew::{function_component, html, Html, props};
use yew::prelude::*;
use uuid::Uuid;
use web_sys::HtmlInputElement;
use shared::common::domain::criteria::filter::condition::Condition;
use crate::infrastructure::components::laps::list::LapList;
use crate::infrastructure::components::repository_context::Repositories;
use crate::infrastructure::repository::analysis::http::Request;


#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub reference_lap: Option<Lap>,
    #[prop_or_default]
    pub target_lap: Option<Lap>,
}

pub enum Msg {
    FetchLaps,
}

#[function_component(LapSelector)]
pub fn lap_selector(props: &Props) -> Html {
    let ref_lap = props.reference_lap.clone();
    let target_lap = props.target_lap.clone();

    let repositories = use_context::<Repositories>().expect("no Repository ctx found");
    let analysis_repo = repositories.analysis;

    let request = use_state(|| Option::<Request>::None);
    let name = use_state(|| "".to_string());

    let on_name_change = {
        let name = name.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            name.set(value);
        })
    };
    
    let create_lap = {
        let name = name.clone();
        let request = request.clone();
        let analysis_repo = analysis_repo.clone();
        let ref_lap = ref_lap.clone();
        let target_lap =  target_lap.clone();
        Callback::from(move |_: MouseEvent| {
            let ref_lap = ref_lap.clone();
            let target_lap =  target_lap.clone();
            let name = name.clone();
            let request = request.clone();
            if let (Some(ref_lap), Some(target_lap)) = (&ref_lap, &target_lap) {
                if !(*name).is_empty() {
                    request.set(Some(Request::new((*name).clone(), ref_lap.id.clone(), target_lap.id.clone())))
                }
                error!("No name provided!");
            } else {
                error!("2 laps not found!");
            }
            
            let request = (*request).clone();
            let analysis_repo = analysis_repo.clone();
            if let Some(request) = request {
                let request = request.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    info!("Into spawn repo");
                    match analysis_repo.create(request).await {
                        Ok(_) => {
                            info!("Análisis creado!");
                        }
                        Err(e) => {
                            error!("Lamentablemente no se ha creado el análisis: {e}");
                        }
                    }
                });
            } else {
                error!("No request found");
            }
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
        </div>
    }
}

fn draw_lap(opt_lap: Option<Lap>, lap_type: &str, icon: &str) -> Html {
    opt_lap.map(|lap| {
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
                                <p class="title is-6">{LapList::lap_name(&lap)}</p>
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
    }).unwrap_or_else(|| {
        html!{
            <div class="cell">
                <label class="label">{format!("{icon} {lap_type}")}</label>
                <div class="box">
                    <div class="has-text-centered">
                        <p class="is-primary is-large">{format!("Select a {lap_type} using {icon} button")}</p>
                    </div>
                </div>
            </div>
        }
    })
}
