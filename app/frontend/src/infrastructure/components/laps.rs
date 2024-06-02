use shared::lap::domain::lap::header::Header as LapHeader;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LapHeadersProps {
    pub lap_headers: Vec<LapHeader>,
}

#[function_component(LapHeadersHtml)]
pub fn lap_headers(LapHeadersProps { lap_headers }: &LapHeadersProps) -> Html {
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