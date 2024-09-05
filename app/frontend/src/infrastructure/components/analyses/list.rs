use crate::infrastructure::components::routes::Route;

use shared::analysis::domain::analysis::header::Header as Analysis;
use shared::analysis::domain::analysis::headers::Headers as Analyses;
use shared::analysis::domain::analysis::status::Status;

use uuid::Uuid;
use yew::Properties;
use yew::{classes, html, Callback, Component, Context, Html};
use yew_router::prelude::Link;

#[derive(Properties, Clone, PartialEq)]
pub struct AnalysisListProps {
    pub analyses: Analyses,
    pub error: Option<String>,
    pub fetching: bool,
    pub fetch_callback: Callback<()>,
    #[prop_or_default]
    pub delete_analysis_callback: Option<Callback<Uuid>>,
}

pub enum Msg {
    ShowModal,
    HideModal,
    Error(String),
}

#[derive(Default)]
pub struct AnalysisList {
    show_modal: bool,
    error: Option<String>,
}

impl Component for AnalysisList {
    type Message = Msg;
    type Properties = AnalysisListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                                <p>{"Error fetching analyses"}</p>
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
                        {"Fetching Analyses..."}
                    </div>
                    <progress class="progress is-large is-primary" max="100" />
                } else if ctx.props().analyses.is_empty() {
                    <div class="block has-text-centered">
                        <h1 class="subtitle is-3">{"No Analyses yet! Start adding a analysis."}</h1>
                    </div>
                } else {
                    {Self::view_analyses(ctx, self.show_modal)}
                }
            </div>
        }
    }
}

impl AnalysisList {
    fn view_analyses(ctx: &Context<Self>, modal: bool) -> Html {
        let analyses = &ctx.props().analyses;
        html! {
            analyses.iter().map(|analysis| {
                let analysis_name = Self::analysis_name(analysis);
                html!{
                    <article class="media is-hoverable">
                        <figure class="media-left">
                            <p class="image is-64x64">
                                <h1 class="title is-1 is-center">{
                                    match analysis.status {
                                        Status::Pending { ref_id: _, target_id: _ } => {"⏳"}
                                        Status::Completed => {"🧬"}
                                        Status::Error(_) => {"🚫"}
                                    }
                                }</h1>
                            </p>
                        </figure>
                        <div class="media-content">
                            <div class="content">
                                <p>
                                    <p class="title is-4">{analysis_name.clone()}</p>
                                    <small class="subtitle is-5">{analysis.circuit.clone()}</small>
                                    <br/>
                                    <small><b>{"Date: "}</b>{analysis.date.to_string()}</small>
                                </p>
                            </div>
                        </div>
                        <div class="media-right">
                            {
                            match analysis.status {
                                Status::Completed => html! {
                                    <>
                                        <Link<Route> to={Route::Dashboard { analysis_id: analysis.id.clone() }}>
                                            <button
                                                class="button is-primary is-outlined is-large js-modal-trigger mx-4"
                                            >{"🔎"}</button>
                                        </Link<Route>>
                                        {Self::add_delete_button(ctx, analysis, modal)}
                                    </>
                                },
                                Status::Error(_) => html! {
                                    {Self::add_delete_button(ctx, analysis, modal)}
                                },
                                _ => html! { <></> }, // No se muestra ningún botón si está en pendiente
                            }
                        }
                        </div>
                    </article>
                }
            }).collect::<Html>()
        }
    }

    pub fn analysis_name(analysis: &Analysis) -> String {
        analysis.name.clone()
    }

    fn add_delete_button(ctx: &Context<Self>, analysis: &Analysis, modal: bool) -> Html {
        ctx.props()
            .delete_analysis_callback
            .clone()
            .map(|cb| {
                html! {
                    <>
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
                                <p class="modal-card-title">{"Delete Analysis"}</p>
                                <button
                                    class="delete"
                                    aria-label="close"
                                    onclick={ctx.link().callback(move |_| {Msg::HideModal})}
                                ></button>
                            </header>
                            <section class="modal-card-body">
                                {
                                    format!(
                                        "Are you sure you want to delete the `{}` analysis?",
                                        Self::analysis_name(analysis)
                                    )
                                }
                            </section>
                            <footer class="modal-card-foot">
                                <div class="buttons">
                                    <button
                                        class="button is-danger"
                                        onclick={
                                            let link = ctx.link().clone();
                                            let cb = cb.clone();
                                            let analysis_id = analysis.id.clone();
                                            link.callback(move |_| {
                                                cb.emit(analysis_id);
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
                    </>
                }
            })
            .unwrap_or_else(|| html! {})
    }
}
