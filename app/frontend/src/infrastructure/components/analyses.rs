//pub(crate) mod filter;
pub(crate) mod list;

use crate::infrastructure::components::analyses::list::AnalysisListComponent;
use crate::infrastructure::components::repository_context::Repositories;
use crate::infrastructure::components::routes::Route;
use crate::infrastructure::repository::analysis::http::Http as AnalysesRepository;

use shared::analysis::domain::analysis::headers::Headers as DomainAnalyses;
use shared::common::domain::criteria::Criteria;

use log::info;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::Link;

pub enum Msg {
    FetchAnalyses,
    DeleteAnalysis(Uuid),
    SetAnalyses(DomainAnalyses),
    SetFilter(Criteria),
    Error(String),
}

#[derive(Default)]
pub struct Analyses {
    filter: Criteria,
    analyses: DomainAnalyses,
    analyses_repository: AnalysesRepository,
    error: Option<String>,
    is_fetching: bool,
}

impl Component for Analyses {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut new_self = Self::default();

        let (repo_ctx, _) = ctx
            .link()
            .context::<Repositories>(Callback::noop())
            .expect("No Repositories Context Provided");

        new_self.analyses_repository = repo_ctx.analysis;

        ctx.link().send_message(Msg::FetchAnalyses);
        new_self.is_fetching = true;

        new_self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchAnalyses => {
                let analyses_repo = self.analyses_repository.clone();
                let link = ctx.link().clone();
                let filter = self.filter.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let filter = filter.clone();
                    match analyses_repo.find_header_by_criteria(&filter).await {
                        Ok(opt_analyses) => {
                            info!("analyses found!");
                            link.send_message(Msg::SetAnalyses(opt_analyses.unwrap_or_default()));
                        }
                        Err(e) => {
                            link.send_message(Msg::Error(e));
                        }
                    }
                });
                false
            }
            Msg::DeleteAnalysis(id) => {
                let analyses_repo = self.analyses_repository.clone();
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match analyses_repo.delete(&id).await {
                        Ok(()) => {
                            link.send_message(Msg::FetchAnalyses);
                        }
                        Err(e) => {
                            link.send_message(Msg::Error(e));
                        }
                    }
                });
                false
            }
            Msg::SetFilter(filter) => {
                self.filter = filter;
                self.is_fetching = true;
                ctx.link().send_message(Msg::FetchAnalyses);
                false
            }
            Msg::SetAnalyses(analyses) => {
                self.is_fetching = false;
                self.analyses = analyses;
                true
            }
            Msg::Error(msg) => {
                self.error = Some(msg);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let fetch_analyses = ctx.link().callback(|()| Msg::FetchAnalyses);
        let delete_analysis_callback = ctx.link().callback(Msg::DeleteAnalysis);

        html! {
            <div class="container">
                <h1 class="title">{"Analyses"}</h1>
                <div class="has-text-centered">
                  <Link<Route> to={Route::AnalysisCreator}><button class="button is-link is-rounded is-primary">{"➕ New Analysis"}</button></Link<Route>>
                </div>
                //<AnalysisFilter {on_filter_change} />
                <AnalysisListComponent
                    analyses={self.analyses.clone()}
                    error={self.error.clone()}
                    {delete_analysis_callback}
                    fetch_callback={fetch_analyses.clone()}
                    fetching={self.is_fetching}
                />
            </div>
        }
    }
}
