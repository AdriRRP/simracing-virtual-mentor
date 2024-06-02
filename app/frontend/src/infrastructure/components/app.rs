use crate::infrastructure::repository::lap::http::Http as LapHttpRepository;
use crate::infrastructure::settings::Settings;

use shared::lap::domain::lap::header::Header as LapHeader;

use log::{error, info};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct LapHeadersProps {
    lap_headers: Vec<LapHeader>,
}

#[function_component(LapHeadersHtml)]
fn lap_headers(LapHeadersProps { lap_headers }: &LapHeadersProps) -> Html {
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
                </tr>
            }
        })
        .collect::<Html>();

    html! {
        <>
            <table class="styled-table">
                <thead>
                    <tr>
                        <th>{"Lap"}</th>
                        <th>{"Time"}</th>
                        <th>{"Player"}</th>
                        <th>{"Car"}</th>
                        <th>{"Circuit"}</th>
                    </tr>
                </thead>
                <tbody>
                    { rows }
                </tbody>
            </table>
        </>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let settings = Settings::default();
    let lap_repo = LapHttpRepository::new(&settings);
    let lap_headers: UseStateHandle<Vec<LapHeader>> = use_state(Vec::new);
    {
        let lap_headers = lap_headers.clone();
        let lap_repo = lap_repo.clone();
        use_effect_with((), move |_| {
            let lap_headers = lap_headers.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_lap_headers = lap_repo.find_header_by_criteria("").await;
                info!("{:?}", fetched_lap_headers);
                let fetched_lap_headers = fetched_lap_headers.unwrap();
                info!("{:?}", fetched_lap_headers);
                let fetched_lap_headers = fetched_lap_headers.unwrap();
                info!("{:?}", fetched_lap_headers);

                lap_headers.set(fetched_lap_headers.to_vec());
            });
            || ()
        });
    }

    html! {
        <main>

            <LapHeadersHtml lap_headers={(*lap_headers).clone()} />
            //<ScatterPlot session={repository.find_by_id(String::from("00000"))}/>
            //<ScatterPlot session={None}/>
        </main>
    }
}
