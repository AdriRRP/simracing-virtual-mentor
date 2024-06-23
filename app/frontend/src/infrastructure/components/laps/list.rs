use shared::file::domain::file::Status;
use shared::lap::domain::lap::headers::Headers as Laps;
use shared::lap::domain::lap::header::Header as Lap;
use shared::common::domain::criteria::Criteria;

use std::future::Future;
use uuid::Uuid;
use yew::{classes, html, Callback, Component, Context, Html};
use yew::Properties;

#[derive(Properties, Clone, PartialEq)]
pub struct LapListProps {
    pub laps: Laps,
    pub error: Option<String>,
    pub fetching: bool,
    pub fetch_callback: Callback<()>,
    pub delete_lap_callback: Callback<Uuid>,
}

pub enum Msg {
    ShowModal,
    HideModal,
    Error(String),
}

#[derive(Default)]
pub struct LapList {
    filter: Criteria,
    show_modal: bool,
    error: Option<String>
}

impl Component for LapList {
    type Message = Msg;
    type Properties = LapListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Error(e) => {
                self.error = Some(e);
                true
            }
            Msg::ShowModal => {
                self.show_modal = true;
                true
            }
            Msg::HideModal => {
                self.show_modal = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="box mt-4">
                if let Some(msg) = &ctx.props().error {
                    <div class="block mx-2">
                        <article class="message is-danger">
                            <div class="message-header">
                                <p>{"Error fetching laps"}</p>
                                <button class="delete"
                                    aria-label="delete"
                                    onclick={
                                        let callback = ctx.props().fetch_callback.clone();
                                        Callback::from(move |_| {callback.emit(())})
                                    }
                                />
                            </div>
                            <div class="message-body">
                                {msg}
                            </div>
                        </article>
                    </div>
                } else if ctx.props().fetching {
                    <div class="block">
                        {"Fetching Laps..."}
                    </div>
                    <progress class="progress is-large is-primary" max="100" />
                } else if ctx.props().laps.is_empty() {
                    <div class="block has-text-centered">
                        <h1 class="subtitle is-3">{"No laps yet! Start adding a file."}</h1>
                    </div>
                } else {
                    {Self::view_laps(ctx, self.show_modal)}
                }
            </div>
        }
    }
}

impl LapList {

    fn lap_name(lap: &Lap) -> String {
        format!(
            "{} [Lap {}] ({:.2} s)",
            lap.driver,
            lap.number,
            lap.time
        )
    }
    pub fn view_laps(ctx: &Context<Self>, modal: bool) -> Html {
        let laps = &ctx.props().laps;
        html! {
            laps.iter().map(|lap| {
                let lap_id = lap.id.clone();
                let lap_name = Self::lap_name(lap);
                html!{
                    <article class="media is-hoverable">
                        <figure class="media-left">
                            <p class="image is-64x64">
                                <h1 class="title is-1 is-center">{"🏁"}</h1>
                            </p>
                        </figure>
                        <div class="media-content">
                            <div class="content">
                                <p>
                                    <p class="title is-4">{lap_name.clone()}</p>
                                    <p class="subtitle is-5">{lap.circuit.clone()}</p>
                                    <p class="subtitle is-7">{lap.car.clone()}</p>
                                    <p class="subtitle is-7">{lap.category.clone()}</p>
                                    <p class="subtitle is-7">{lap.date.to_string()}</p>
                                </p>
                            </div>
                        </div>
                        <div class="media-right">
                            <button
                                class="button is-danger is-outlined is-large js-modal-trigger"
                                data-target="delete-modal"
                                onclick={ctx.link().callback(move |_| {Msg::ShowModal})}
                            >{"❌"}</button>
                            <div id="delete-modal"
                                class={classes!(if modal {"modal is-active"} else {"modal"})}>
                                <div class="modal-background"></div>
                                <div class="modal-card">
                                    <header class="modal-card-head">
                                        <p class="modal-card-title">{"Delete File"}</p>
                                        <button
                                            class="delete"
                                            aria-label="close"
                                            onclick={ctx.link().callback(move |_| {Msg::HideModal})}
                                        ></button>
                                    </header>
                                    <section class="modal-card-body">
                                        {
                                            format!(
                                                "Are you sure you want to delete the `{}` lap?",
                                                lap_name.clone()
                                            )
                                        }
                                    </section>
                                    <footer class="modal-card-foot">
                                        <div class="buttons">
                                            <button
                                                class="button is-danger"
                                                onclick={
                                                    let link = ctx.link().clone();
                                                    let callback = ctx.props().delete_lap_callback.clone();
                                                    link.callback(move |_| {
                                                        callback.emit(lap_id.clone());
                                                        Msg::HideModal
                                                    })}
                                            >{"Delete"}</button>
                                            <button
                                                class="button is-dark"
                                                onclick={ctx.link().callback(move |_| {Msg::HideModal})}
                                            >{"Cancel"}</button>
                                        </div>
                                    </footer>
                                </div>
                            </div>
                        </div>
                    </article>
                }
            }).collect::<Html>()
        }
    }
}
