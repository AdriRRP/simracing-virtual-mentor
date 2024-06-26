pub(crate) mod list;
pub(crate) mod filter;

use crate::infrastructure::components::laps::filter::LapFilter;
use crate::infrastructure::components::repository_context::Repositories;
use crate::infrastructure::repository::lap::http::Http as LapRepository;
use crate::infrastructure::components::laps::list::LapList;

use shared::common::domain::criteria::Criteria;
use shared::lap::domain::lap::headers::Headers as DomainLaps;

use yew::prelude::*;
use log::info;
use uuid::Uuid;

pub enum Msg {
    FetchLaps,
    DeleteLap(Uuid),
    SetLaps(DomainLaps),
    SetFilter(Criteria),
    Error(String),
}

#[derive(Default)]
pub struct Laps {
    filter: Criteria,
    laps: DomainLaps,
    lap_repository: LapRepository,
    error: Option<String>,
    is_fetching: bool,
}

impl Component for Laps {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        let mut _self = Self::default();

        let (repo_ctx, _) = ctx
            .link()
            .context::<Repositories>(Callback::noop())
            .expect("No Repositories Context Provided");

        _self.lap_repository = repo_ctx.lap.clone();
        
        ctx.link().send_message(Msg::FetchLaps);
        _self.is_fetching = true;

        _self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchLaps => {
                let lap_repo = self.lap_repository.clone();
                let link = ctx.link().clone();
                let filter = self.filter.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let filter = filter.clone();
                    match lap_repo.find_header_by_criteria(&filter).await {
                        Ok(opt_laps) => {
                            info!("laps found!");
                            link.send_message(Msg::SetLaps(opt_laps.unwrap_or_default()));
                        }
                        Err(e) => {
                            link.send_message(Msg::Error(e));
                        }
                    }
                });
                false
            }
            Msg::DeleteLap(id) => {
                let lap_repo = self.lap_repository.clone();
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match lap_repo.delete(&id).await {
                        Ok(()) => {
                            link.send_message(Msg::FetchLaps);
                        }
                        Err(e) => {
                            link.send_message(Msg::Error(e));
                        }
                    }
                });
                false
            }
            Msg::SetFilter(filter) => {
                info!("setting new filter {:?}", filter.clone());
                self.filter = filter;
                self.is_fetching = true;
                ctx.link().send_message(Msg::FetchLaps);
                false
            }
            Msg::SetLaps(laps) => {
                self.is_fetching = false;
                self.laps = laps;
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
        let fetch_laps = ctx.link().callback(|_| Msg::FetchLaps);
        let delete_lap_callback = ctx.link().callback(Msg::DeleteLap);

        html! {
            <div class="container">
                <h1 class="title">{"Laps"}</h1>
                <LapFilter {on_filter_change} />
                <LapList
                    laps={self.laps.clone()}
                    error={self.error.clone()}
                    {delete_lap_callback}
                    fetch_callback={fetch_laps.clone()}
                    fetching={self.is_fetching.clone()}
                />
            </div>
        }
    }
}