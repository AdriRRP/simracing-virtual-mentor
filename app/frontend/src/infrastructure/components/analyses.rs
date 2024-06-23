use crate::infrastructure::components::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(Analyses)]
pub fn analyses() -> Html {
    html! {
        <div class="container">
            <h1 class="title">{"Analyses"}</h1>
            <div class="card">
                <h2>{ "Search Analyses" }</h2>
                <form>
                    <input type="text" placeholder="Filter by name" />
                    <button type="submit">{ "Search" }</button>
                </form>
            </div>
            <div class="styled-table">
                <h2>{ "Analyses" }</h2>
                <table>
                    <thead>
                        <tr>
                            <th>{ "ID" }</th>
                            <th>{ "Name" }</th>
                            <th>{ "Date" }</th>
                            <th>{ "Actions" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>{ "1" }</td>
                            <td>{ "Analysis 1" }</td>
                            <td>{ "2024-06-03" }</td>
                            <td>
                                <button onclick={Callback::from(|_| { /* Eliminar file */ })}>
                                    { "🗑️ Delete" }
                                </button>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <Link<Route> to={Route::AnalysisCreator}><button class="button is-primary is-rounded">{"➕"}</button></Link<Route>>
        </div>
    }
}
