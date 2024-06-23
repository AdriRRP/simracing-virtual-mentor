use crate::infrastructure::components::repository_context::Repositories;

use gloo::file::{callbacks::FileReader, File};
use log::{error, info};
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, InputEvent, SubmitEvent};
use yew::{html, Callback, Component, Context, Html, NodeRef, Properties};

pub enum Msg {
    Upload(String, Vec<u8>),
    Files(Vec<File>),
    BackToDefault,
    ChangeFileName(InputEvent),
    ChangeFilePath,
    Submit(Vec<File>),
    Success(String),
    Error(String),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_file_uploaded: Callback<()>,
}

pub struct FileUploader {
    readers: HashMap<String, FileReader>,
    file_form_node_ref: NodeRef,
    file_name: String,
    name: String,
    name_disabled: bool,
    submit_disabled: bool,
    submitting: bool,
    success: Option<String>,
    error: Option<String>,
}

impl Default for FileUploader {
    fn default() -> Self {
        Self {
            readers: HashMap::default(),
            file_form_node_ref: NodeRef::default(),
            file_name: "Choose a file...".to_string(),
            name: String::default(),
            name_disabled: true,
            submit_disabled: true,
            submitting: false,
            success: Option::default(),
            error: Option::default(),
        }
    }
}

impl Component for FileUploader {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Upload(file_name, data) => {
                info!("Upload message received");
                info!("{file_name}: {}", data.len());

                let (repo_ctx, _) = ctx
                    .link()
                    .context::<Repositories>(Callback::noop())
                    .expect("No Repositories Context Provided");

                {
                    let ibt_repo = repo_ctx.ibt.clone();
                    let file_name = file_name.clone();
                    let name = self.name.clone();
                    let link = ctx.link().clone();
                    let on_file_uploaded = ctx.props().on_file_uploaded.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        info!("Into spawn repo");
                        match ibt_repo.upload(name, file_name.clone(), data).await {
                            Ok(_) => {
                                info!("Correctamente subido!");
                                link.send_message(Msg::Success(format!(
                                    "File `{file_name}` successfully uploaded"
                                )));
                                on_file_uploaded.emit(())
                            }
                            Err(e) => {
                                error!("Lamentablemente hay un error: {e}");
                                link.send_message(Msg::Error(e))
                            }
                        }
                    });
                }

                self.readers.remove(&file_name);
                true
            }
            Msg::Files(files) => {
                info!("Files message received");
                for file in files.into_iter() {
                    let file_name = file.name();
                    //let file_type = file.raw_mime_type();

                    let task = {
                        let link = ctx.link().clone();
                        let file_name = file_name.clone();

                        gloo::file::callbacks::read_as_bytes(&file, move |res| {
                            let msg = match res {
                                Ok(data) => Msg::Upload(file_name, data),
                                Err(e) => Msg::Error(e.to_string()),
                            };
                            link.send_message(msg)
                        })
                    };
                    self.readers.insert(file_name, task);
                }
                false
            }
            Msg::BackToDefault => {
                info!("BackToDefault message received");
                *self = Self::default();
                true
            }
            Msg::ChangeFileName(event) => {
                info!("ChangeFileName message received");
                let opt_input_element = event
                    .target()
                    .map(EventTarget::dyn_into::<HtmlInputElement>);
                if let Some(Ok(input)) = opt_input_element {
                    self.name = input.value();
                } else {
                    ctx.link()
                        .callback(|()| Msg::Error("Cannot retrieve file name".to_string()));
                }

                true
            }
            Msg::ChangeFilePath => {
                info!("ChangeFilePath message received");
                let opt_input_element = self.file_form_node_ref.clone().cast::<HtmlInputElement>();
                if let Some(input) = opt_input_element {
                    let file_path = input.value();
                    let file_name = file_path
                        .clone()
                        .split("/")
                        .last()
                        .and_then(|v| v.split("\\").last())
                        .map(|v| v.to_string())
                        .unwrap_or_else(String::default);

                    if file_path.is_empty() || file_name.is_empty() {
                        ctx.link().callback(|()| Msg::BackToDefault);
                    } else {
                        let name = file_name
                            .strip_suffix(".ibt")
                            .map(|v| v.to_string())
                            .unwrap_or_else(String::default);

                        *self = Self {
                            file_name,
                            name,
                            name_disabled: false,
                            submit_disabled: false,
                            ..Self::default()
                        }
                    }
                }
                true
            }
            Msg::Error(msg) => {
                info!("Error message received");
                self.error = Some(msg);
                true
            }
            Msg::Success(msg) => {
                info!("Success message received");
                self.success = Some(msg);
                true
            }
            Msg::Submit(files) => {
                info!("Submit message received");
                self.submitting = true;
                ctx.link().send_message(Msg::Files(files.clone()));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="box mt-4">
                if let Some(msg) = &self.error {
                    <div class="block mx-2">
                        <article class="message is-danger">
                            <div class="message-header">
                                <p>{"Error submitting file"}</p>
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
                } else if let Some(msg) = &self.success {
                    <div class="block mx-2">
                        <article class="message is-success">
                          <div class="message-header">
                            <p>{"Successfully uploaded file"}</p>
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
                } else if self.submitting {
                    <div class="block">
                        <h1 class="title">{format!("Submitting file `{}`", &self.name)}</h1>
                        <h1 class="subtitle">{format!("Original file name: `{}`", &self.file_name)}</h1>
                            <progress class="progress is-large is-primary" max="100" />
                    </div>
                } else {
                    <form
                        class="columns mt-4"
                        onsubmit={
                            let file_form_node_ref = self.file_form_node_ref.clone();
                            ctx.link().callback(move |event: SubmitEvent| {
                                event.prevent_default();
                                match Self::extract_files(&file_form_node_ref) {
                                    Ok(files) => {Msg::Submit(files)}
                                    Err(e) => {Msg::Error(e)}
                                }
                            })
                        }
                        //action={ format!("{}/{}", upload_endpoint.clone(), encode(&*name)) }
                        method="post"
                        enctype="multipart/form-data">

                        <div class="column is-two-quarters">
                            <div class="file has-name is-boxed is-large">
                                <label class="file-label">
                                    <input
                                        ref={self.file_form_node_ref.clone()}
                                        onchange={ctx.link().callback(move |_| {
                                            Msg::ChangeFilePath
                                        })}
                                        id="input-file"
                                        name="input-file"
                                        class="file-input"
                                        type="file"
                                        accept=".ibt"
                                        name="resume" />
                                    <span class="file-cta">
                                        <span class="title">{"⇪"}</span>
                                        <span class="file-label"> {"Upload .ibt file"} </span>
                                    </span>
                                </label>
                            </div>
                        </div>
                        <div class="column is-three-quarters">
                            <div class="block">
                                <label
                                    for="input-file"
                                    class="file-name"
                                > { &self.file_name } </label>
                                <input
                                    type="text"
                                    oninput={ctx.link().callback(move |event: InputEvent| {
                                        Msg::ChangeFileName(event)
                                    })}
                                    class="input is-normal"
                                    placeholder="File name"
                                    value={self.name.clone()}
                                    disabled={self.name_disabled}
                                />
                            </div>
                            <div class="block">
                                <button
                                    type="submit"
                                    class="button"
                                    disabled={self.submit_disabled}
                                >{"Submit"}</button>
                            </div>
                        </div>
                    </form>
                }
            </div>
        }
    }
}

impl FileUploader {
    pub fn extract_files(input_node_ref: &NodeRef) -> Result<Vec<File>, String> {
        let input = input_node_ref
            .cast::<HtmlInputElement>()
            .ok_or("Cannot cast node to HtmlInputElement")?;

        let file_list = input.files().ok_or("Cannot get files from input element")?;

        let mut result: Vec<File> = Vec::new();

        let iter = js_sys::try_iter(&file_list)
            .map_err(|e| {
                e.as_string()
                    .unwrap_or_else(|| "Cannot iterate over FileList".to_string())
            })?
            .ok_or("FileList iterator is empty")?;

        for value in iter {
            let js_value = value.map_err(|e| {
                e.as_string()
                    .unwrap_or_else(|| "Error in FileList iteration".to_string())
            })?;

            let ws_file = web_sys::File::from(js_value);
            result.push(File::from(ws_file));
        }

        Ok(result)
    }
}
