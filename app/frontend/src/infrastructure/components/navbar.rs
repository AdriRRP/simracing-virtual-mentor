use crate::infrastructure::components::routes::Route;

use yew::function_component;
use yew::html;
use yew::Html;
use yew_router::prelude::*;

#[function_component(NavBar)]
pub fn header() -> Html {
    html! {
    <nav class="navbar" role="navigation" aria-label="main navigation">
        <input id="menu-switch" name="menu-switch" type="checkbox"/>

        <div class="navbar-brand">
            <div class="is-size-3 mx-6 my-2">
                {"Simracing Virtual Mentor"}
            </div>
            <label for="menu-switch" class="navbar-burger" role="button" aria-expanded="false">
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
            </label>
        </div>

        <div class="navbar-menu">

            <div class="navbar-start">
                <a class="navbar-item">
                    <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                </a>
                <a class="navbar-item">
                    <Link<Route> to={Route::Files}>{ "Files" }</Link<Route>>
                </a>
                <a class="navbar-item">
                    <Link<Route> to={Route::Laps}>{ "Laps" }</Link<Route>>
                </a>
                <a class="navbar-item">
                    <Link<Route> to={Route::Analyses}>{ "Analyses" }</Link<Route>>
                </a>
            </div>

            <div class="navbar-end">
                <div class="navbar-item">
                    <div class="buttons">
                        <a class="button is-primary">
                            <strong>{"Notifications"}</strong>
                        </a>
                    </div>
                </div>
            </div>

        </div>
    </nav>
    }
}
