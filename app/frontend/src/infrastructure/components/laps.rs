use crate::infrastructure::repository::lap::http::Http as LapHttpRepository;
use crate::infrastructure::settings::Settings;

use shared::lap::domain::lap::headers::Headers as LapHeaders;

use log::info;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LapHeadersProps {
    pub lap_headers: LapHeaders,
}

#[function_component(Laps)]
pub fn laps(LapHeadersProps { lap_headers }: &LapHeadersProps) -> Html {
    info!("Entering Laps view()");
    let rows = lap_headers
        .iter()
        .map(|lap_header| {
            html! {
                <tr>
                    <td>{ format!("{}", lap_header.number) }</td>
                    <td>{ format!("{}", lap_header.time) }</td>
                    <td>{ format!("{}", lap_header.driver) }</td>
                    <td>{ format!("{}", lap_header.car) }</td>
                    <td>{ format!("{}", lap_header.circuit) }</td>
                    <td>
                        <button onclick={Callback::from(|_| { /* Eliminar file */ })}>
                            { "🗑️ Delete" }
                        </button>
                    </td>
                </tr>
            }
        })
        .collect::<Html>();
    html! {
        <div class="files">
            <div class="card">
                <h2>{ "Search Laps" }</h2>
                <form>
                    <input type="text" placeholder="Filter by name" />
                    <button type="submit">{ "Search" }</button>
                </form>
            </div>
            <div class="table">
                <h2>{ "Laps" }</h2>
                <table class="styled-table">
                <thead>
                    <tr>
                        <th>{"Lap"}</th>
                        <th>{"Time"}</th>
                        <th>{"Player"}</th>
                        <th>{"Car"}</th>
                        <th>{"Circuit"}</th>
                        <th>{ "Actions" }</th>
                    </tr>
                </thead>
                <tbody>
                    { rows }
                </tbody>
            </table>
            </div>
        </div>
    }
}

#[function_component(LapsWithState)]
pub fn laps_with_state() -> Html {
    info!("Entering Laps with state view()");
    let settings = Settings::default();
    let lap_repo = LapHttpRepository::new(&settings);
    let lap_headers = use_state(LapHeaders::default);

    {
        let lap_headers = lap_headers.clone();
        let lap_repo = lap_repo.clone();
        use_effect_with((), move |_| {
            let lap_headers = lap_headers.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_lap_headers = lap_repo.find_header_by_criteria("").await;
                info!("{:?}", fetched_lap_headers);
                if let Ok(Some(headers)) = fetched_lap_headers {
                    lap_headers.set(headers);
                }
            });
            || ()
        });
    }

    html! { <Laps lap_headers={(*lap_headers).clone()} /> }
}
