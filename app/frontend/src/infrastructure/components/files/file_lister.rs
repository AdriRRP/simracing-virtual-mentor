use crate::infrastructure::components::repository_context::Repositories;

use shared::file::domain::files::Files;

use std::future::Future;
use log::info;
use yew::{Callback, Component, Context, Html, html, classes};

pub enum Msg {
    Fetch,
    View(Option<Files>),
    DeleteFile(String),
    ShowModal,
    HideModal,
    Error(String),
    BackToDefault,
}

pub struct FileLister {
    pub fetching: bool,
    pub show_modal: bool,
    pub files: Option<Files>,
    pub error: Option<String>,
}

impl Default for FileLister {
    fn default() -> Self {
        Self {
            fetching: true,
            show_modal: false,
            files: Option::default(),
            error: Option::default(),
        }
    }
}

impl Component for FileLister {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetch => {
                let (repo_ctx, _) = ctx
                    .link()
                    .context::<Repositories>(Callback::noop())
                    .expect("No Repositories Context Provided");

                {
                    let file_repo = repo_ctx.file.clone();
                    let link = ctx.link().clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match file_repo.find_by_criteria("").await {
                            Ok(opt_files) => {
                                link.send_message(Msg::View(opt_files));
                            }
                            Err(e) => {
                                link.send_message(Msg::Error(e));
                            }
                        }
                    });
                }
                false
            }
            Msg::View(opt_files) => {
                self.fetching = false;
                self.files = opt_files;
                true
            }
            Msg::Error(e) => {
                self.fetching = false;
                self.error = Some(e);
                true
            }
            Msg::DeleteFile(id) => {
                let (repo_ctx, _) = ctx
                    .link()
                    .context::<Repositories>(Callback::noop())
                    .expect("No Repositories Context Provided");

                {
                    let file_repo = repo_ctx.file.clone();
                    let link = ctx.link().clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match file_repo.delete(&id).await {
                            Ok(_) => {
                                link.send_message(Msg::BackToDefault);
                            }
                            Err(e) => {
                                link.send_message(Msg::Error(e));
                            }
                        }
                    });
                }
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
            Msg::BackToDefault => {
                info!("BackToDefault message received");
                *self = Self::default();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        if self.fetching {
            ctx.link().send_message(Msg::Fetch)
        };

        html!{
            <div class="box mt-4">
                if let Some(msg) = &self.error {
                    <div class="block mx-2">
                        <article class="message is-danger">
                            <div class="message-header">
                                <p>{"Error fetching files"}</p>
                                <button class="delete"
                                    aria-label="delete"
                                    onclick={ctx.link().callback(|_| {Msg::BackToDefault})}
                                />
                            </div>
                            <div class="message-body">
                                {msg}
                            </div>
                        </article>
                    </div>
                } else if self.fetching {
                    <div class="block">
                        {"Fetching Files..."}
                    </div>
                    <progress class="progress is-large is-primary" max="100" />
                } else if let Some(files) = &self.files {
                    {Self::view_files(files, ctx, self.show_modal)}
                } else {
                    <div class="block has-text-centered">
                        <h1 class="subtitle is-3">{"No files yet! Start adding a file."}</h1>
                    </div>
                }
            </div>
        }
    }
}

impl FileLister {
    pub fn view_files(files: &Files, ctx: &Context<Self>, modal: bool) -> Html {
        html! {
            files.iter().map(|file| {
                let file_id = file.id.clone();
                html!{
                    <article class="media is-hoverable">
                        <figure class="media-left">
                            <p class="image is-64x64">
                                <h1 class="title is-1 is-center">{"📄"}</h1>
                            </p>
                        </figure>
                        <div class="media-content">
                            <div class="content">
                                <p>
                                    <strong>{file.name.clone()}</strong>
                                    <br />
                                    <small>{file.complete}</small>
                                    <br />
                                    <small>{"25/09/2023"}</small>
                                </p>
                            </div>
                        </div>
                        <div class="media-right">
                            <button
                                class="button is-info is-dark is-outlined is-large mr-4"
                            >{"🏁"}</button>
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
                                                "Are you sure you want to delete the `{}` file?",
                                                file.name.clone()
                                            )
                                        }      
                                    </section>
                                    <footer class="modal-card-foot">
                                        <div class="buttons">
                                            <button
                                                class="button is-danger"
                                                onclick={ctx.link().callback(move |_| {Msg::DeleteFile(file_id.clone())})}
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