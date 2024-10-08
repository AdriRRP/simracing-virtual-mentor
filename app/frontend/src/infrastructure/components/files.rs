mod filter;
mod list;
mod uploader;

use crate::infrastructure::components::files::filter::FileFilter;
use crate::infrastructure::components::files::list::FileListComponent;
use crate::infrastructure::components::files::uploader::FileUploaderComponent;
use crate::infrastructure::components::repository_context::Repositories;
use crate::infrastructure::repository::file::http::Http as FileRepository;

use shared::common::domain::criteria::Criteria;
use shared::file::domain::files::Files as DomainFiles;

use log::info;
use yew::prelude::*;

pub enum Msg {
    FetchFiles,
    DeleteFile(String),
    SetFiles(DomainFiles),
    SetFilter(Criteria),
    Error(String),
}

#[derive(Default)]
pub struct Files {
    filter: Criteria,
    files: DomainFiles,
    file_repository: FileRepository,
    error: Option<String>,
    is_fetching: bool,
}

impl Component for Files {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut new_self = Self::default();

        let (repo_ctx, _) = ctx
            .link()
            .context::<Repositories>(Callback::noop())
            .expect("No Repositories Context Provided");

        new_self.file_repository = repo_ctx.file;

        ctx.link().send_message(Msg::FetchFiles);
        new_self.is_fetching = true;

        new_self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchFiles => {
                let file_repo = self.file_repository.clone();
                let link = ctx.link().clone();
                let filter = self.filter.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let filter = filter.clone();
                    match file_repo.find_by_criteria(&filter).await {
                        Ok(opt_files) => {
                            info!("laps found!");
                            link.send_message(Msg::SetFiles(opt_files.unwrap_or_default()));
                        }
                        Err(e) => {
                            link.send_message(Msg::Error(e));
                        }
                    }
                });
                false
            }
            Msg::DeleteFile(id) => {
                let file_repo = self.file_repository.clone();
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match file_repo.delete(&id).await {
                        Ok(()) => {
                            link.send_message(Msg::FetchFiles);
                        }
                        Err(e) => {
                            link.send_message(Msg::Error(e));
                        }
                    }
                });
                false
            }
            Msg::SetFilter(filter) => {
                self.is_fetching = true;
                self.filter = filter;
                ctx.link().send_message(Msg::FetchFiles);
                false
            }
            Msg::SetFiles(files) => {
                self.is_fetching = false;
                self.files = files;
                true
            }
            Msg::Error(msg) => {
                self.error = Some(msg);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_filter_change = ctx.link().callback(Msg::SetFilter);
        let fetch_files = ctx.link().callback(|()| Msg::FetchFiles);
        let delete_file_callback = ctx.link().callback(Msg::DeleteFile);

        html! {
            <div class="container">
                <h1 class="title">{"Files"}</h1>
                <FileUploaderComponent on_file_uploaded={fetch_files.clone()} />
                <FileFilter {on_filter_change} />
                <FileListComponent
                    files={self.files.clone()}
                    error={self.error.clone()}
                    {delete_file_callback}
                    fetch_callback={fetch_files.clone()}
                    fetching={self.is_fetching}
                />
            </div>
        }
    }
}
