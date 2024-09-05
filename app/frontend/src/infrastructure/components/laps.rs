pub(crate) mod filter;
pub(crate) mod list;

use crate::infrastructure::components::laps::list::LapListComponent;
use crate::infrastructure::components::repository_context::Repositories;
use crate::infrastructure::repository::lap::http::Http as LapRepository;

use shared::common::domain::criteria::Criteria;
use shared::lap::domain::lap::headers::Headers as DomainLaps;

use log::info;
use uuid::Uuid;
use yew::prelude::*;

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
        let mut new_self = Self::default();

        let (repo_ctx, _) = ctx
            .link()
            .context::<Repositories>(Callback::noop())
            .expect("No Repositories Context Provided");

        new_self.lap_repository = repo_ctx.lap;

        ctx.link().send_message(Msg::FetchLaps);
        new_self.is_fetching = true;

        new_self
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
                info!("setting new filter {:?}", filter);
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
        let fetch_laps = ctx.link().callback(|()| Msg::FetchLaps);
        let delete_lap_callback = ctx.link().callback(Msg::DeleteLap);

        html! {
            <div class="container">
                <h1 class="title">{"Laps"}</h1>
                //<LapFilter {on_filter_change} />
                <LapListComponent
                    laps={self.laps.clone()}
                    error={self.error.clone()}
                    {delete_lap_callback}
                    fetch_callback={fetch_laps.clone()}
                    fetching={self.is_fetching}
                />
            </div>
        }
    }
}
