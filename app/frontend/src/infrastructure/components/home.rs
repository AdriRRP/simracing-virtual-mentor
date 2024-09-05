use crate::infrastructure::components::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container">
            <h1 class="title px-4">{"Home"}</h1>
            <nav class="grid px-4 pt-4">
                <div class="cell mx-2 my-2">
                    <Link<Route> to={Route::Files}>
                        <div class="card">
                            <div class="card-image">
                                <figure class="image is-128x128 container">
                                    <img
                                        src="https://cdn-icons-png.freepik.com/256/12409/12409363.png"
                                        alt="Files"
                                    />
                                </figure>
                            </div>
                            <div class="card-content">
                                <p class="title is-4">{"Files"}</p>
                                <p class="subtitle is-6">{"Upload and manage files"}</p>
                            </div>
                        </div>
                    </Link<Route>>
                </div>
                <div class="cell mx-2 my-2">
                    <Link<Route> to={Route::Laps}>
                        <div class="card">
                            <div class="card-image">
                                <figure class="image is-128x128 container">
                                    <img
                                        src="https://cdn-icons-png.flaticon.com/256/1850/1850750.png"
                                        alt="Laps"
                                    />
                                </figure>
                            </div>
                            <div class="card-content">
                                <p class="title is-4">{"Laps"}</p>
                                <p class="subtitle is-6">{"Search and manage Laps"}</p>
                            </div>
                        </div>
                    </Link<Route>>
                </div>
                <div class="cell mx-2 my-2">
                    <Link<Route> to={Route::Analyses}>
                        <div class="card">
                            <div class="card-image">
                                <figure class="image is-128x128 container">
                                    <img
                                        src="https://cdn-icons-png.flaticon.com/256/10797/10797446.png"
                                        alt="Analyses"
                                    />
                                </figure>
                            </div>
                            <div class="card-content">
                                <p class="title is-4">{"Analyses"}</p>
                                <p class="subtitle is-6">{"Create and manage Analyses"}</p>
                            </div>
                        </div>
                    </Link<Route>>
                </div>
            </nav>
        </div>
    }
}

/*

<div class="home">
    <div class="card">
        <img src="https://cdn-icons-png.freepik.com/256/12409/12409363.png?semt=ais_hybrid" alt="Files" />
        <h2>{ "Files" }</h2>
        <p>{ "Upload and manage files" }</p>
    </div>
    <div class="card">
        <img src="https://cdn-icons-png.flaticon.com/256/1850/1850750.png" alt="Laps" />
        <h2>{ "Laps" }</h2>
        <p>{ "Search and manage Laps" }</p>
    </div>
    <div class="card">
        <img src="https://cdn-icons-png.flaticon.com/256/10797/10797446.png" alt="Analyses" />
        <h2>{ "Analyses" }</h2>
        <p>{ "Create and manage Analyses" }</p>
    </div>
    <div class="styled-table">
        <h2>{ "Last Analyses" }</h2>
        <table>
            <thead>
                <tr>
                    <th>{ "ID" }</th>
                    <th>{ "Name" }</th>
                    <th>{ "Date" }</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>{ "1" }</td>
                    <td>{ "Analysis 1" }</td>
                    <td>{ "2024-06-03" }</td>
                </tr>
            </tbody>
        </table>
    </div>
</div>
*/
