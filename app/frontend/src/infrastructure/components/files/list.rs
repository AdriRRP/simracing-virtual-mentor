use shared::file::domain::file::Status;
use shared::file::domain::files::Files;
use shared::common::domain::criteria::Criteria;

use std::future::Future;
use yew::{classes, html, Callback, Component, Context, Html};
use yew::Properties;

#[derive(Properties, Clone, PartialEq)]
pub struct FileListProps {
    pub files: Files,
    pub error: Option<String>,
    pub fetch_callback: Callback<()>,
    pub delete_file_callback: Callback<String>,
}

pub enum Msg {
    ShowModal,
    HideModal,
    Error(String),
}

#[derive(Default)]
pub struct FileList {
    filter: Criteria,
    fetching: bool,
    show_modal: bool,
    error: Option<String>
}

impl Component for FileList {
    type Message = Msg;
    type Properties = FileListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Error(e) => {
                self.fetching = false;
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
                                <p>{"Error fetching files"}</p>
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
                } else if self.fetching {
                    <div class="block">
                        {"Fetching Files..."}
                    </div>
                    <progress class="progress is-large is-primary" max="100" />
                } else if ctx.props().files.is_empty() {
                    <div class="block has-text-centered">
                        <h1 class="subtitle is-3">{"No files yet! Start adding a file."}</h1>
                    </div>
                } else {
                    {Self::view_files(ctx, self.show_modal)}
                }
            </div>
        }
    }
}

impl FileList {
    pub fn view_files(ctx: &Context<Self>, modal: bool) -> Html {
        let files = &ctx.props().files;

        html! {
            files.iter().map(|file| {
                let file_id = file.id.clone();
                html!{
                    <article class="media is-hoverable">
                        <figure class="media-left">
                            <p class="image is-64x64">
                                <h1 class="title is-1 is-center">{
                                    match file.status {
                                        Status::Accepted => {"⏳"}
                                        Status::Success => {"📄"}
                                        Status::Fail(_) => {"🚫"}
                                    }
                                }</h1>
                            </p>
                        </figure>
                        <div class="media-content">
                            <div class="content">
                                <p>
                                    <strong>{file.name.clone()}</strong>
                                    <br />
                                    <small>{file.created_on.to_string()}</small>
                                    <br />
                                    {
                                        match &file.status {
                                            Status::Fail(msg) => html!{
                                                <small class="has-text-danger">{msg}</small>
                                            },
                                            _ => html!{},
                                        }
                                    }
                                </p>
                            </div>
                        </div>
                        <div class="media-right">
                            <button
                                class="button is-info is-dark is-outlined is-large mr-4"
                                disabled={!matches!(file.status, Status::Success)}
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
                                                onclick={
                                                    let link = ctx.link().clone();
                                                    let callback = ctx.props().delete_file_callback.clone();
                                                    link.callback(move |_| {
                                                        callback.emit(file_id.clone());
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
