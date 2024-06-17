mod hook;
mod plot;

use crate::infrastructure::components::dashboard::hook::{use_analyses, use_plotly_draw};
use crate::infrastructure::components::dashboard::plot::create_plot;
use crate::infrastructure::repository::analysis::http::Http as AnalysisHttpRepository;
use crate::infrastructure::settings::Settings;
use std::ops::{Deref, Div};
use std::rc::Rc;

use log::{error, info, trace, warn};
use plotly::color::{Color, NamedColor, Rgb};
use plotly::common::{
    Anchor, AxisSide, DashType, Fill, Font, HoverInfo, HoverOn, Label, Line, Marker, Mode,
    Orientation, Pad, Reference, Side, Title,
};
use plotly::layout::update_menu::{Button, ButtonMethod, UpdateMenu, UpdateMenuDirection};
use plotly::layout::{Axis, HoverMode, Legend};
use plotly::layout::{LayoutGrid, SpikeSnap};
use plotly::Layout;
use plotly::{Plot, Scatter};
use shared::lap::domain::laps::Laps;
use yew::prelude::*;

use plotly::layout::GridPattern;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

use gloo_events::{EventListener, EventListenerOptions};
use gloo_net::websocket::Message;
use wasm_bindgen::__rt::IntoJsResult;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::Object;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, HtmlElement, MouseEvent};
use shared::analysis::domain::analysis::Analysis;

// components/fx

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = Plotly, js_name = newPlot)]
    async fn js_new_plot_(id: &str, obj: &Object) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen(module = "/assets/scripts/utils.js")]
extern "C" {
    #[wasm_bindgen(js_name = "sync_plotly_hover")]
    fn sync_plotly_hover(div_id: JsValue);
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or(None)]
    pub analysis: Option<Analysis>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct PlotlyDrawerProps {
    #[prop_or(None)]
    pub analysis: Option<Analysis>,
}

pub struct PlotlyDrawer {
    state: PlotlyDrawerState,
    target_div: NodeRef,
    dom_node_inserted_listener: Option<EventListener>,
    plotly_hover_listener: Option<EventListener>,
}

pub enum PlotlyDrawerState {
    NotFetching,
    Fetching,
    Success,
    Failed(String),
}

pub enum PlotlyDrawerMsg {
    PlotlyHover,
    SyncPlotlyHover(String),
    Error(String),
}

impl Component for PlotlyDrawer {
    type Message = PlotlyDrawerMsg;
    type Properties = PlotlyDrawerProps;
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: PlotlyDrawerState::NotFetching,
            target_div: NodeRef::default(),
            dom_node_inserted_listener: None,
            plotly_hover_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        info!("Entering PlotlyDrawer");

        let analysis = ctx.props().analysis.clone();
        let div_id = "plotly_dashboard".to_string();

        let Some(analysis) = analysis else {
            return html! {<div>{"Oooops! No Analysis found..."}</div>};
        };

        ctx.link().send_future({
            let plot = create_plot(analysis.clone());
            let div_id = div_id.clone();
            async move {
                info!("Starting plotly binding");
                match js_new_plot_(div_id.as_str(), &plot.to_js_object()).await {
                    Ok(_) => Self::Message::SyncPlotlyHover(div_id),
                    Err(e) => Self::Message::Error(format!("{e:?}")),
                }
            }
        });

        html! { <div id={div_id} ref={self.target_div.clone()}></div> }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::PlotlyHover => false,
            Self::Message::SyncPlotlyHover(div_id) => {
                sync_plotly_hover(JsValue::from(div_id));
                false
            }
            Self::Message::Error(e) => {
                error!("{e}");
                // TODO: Manage error
                true
            }
        }
    }
}

#[function_component(PlotlyLoader)]
pub fn plotly_loader(Props { analysis }: &Props) -> Html {
    info!("Entering PlotlyLoader");
    let fallback = html! {<div>{"Drawing dashboard..."}</div>};

    html! {
        <Suspense {fallback}>
            <PlotlyDrawer analysis={analysis.clone()}/>
        </Suspense>
    }
}

#[function_component(DashboardDataFetcher)]
pub fn dashboard_data_fetcher() -> HtmlResult {
    info!("Entering DashboardDataFetcher");

    let settings = Settings::default();
    let analysis_repo = AnalysisHttpRepository::new(&settings);

    let analysis = use_analyses("", analysis_repo)?;

    Ok(html! { <PlotlyDrawer analysis={analysis.clone()} /> })
}

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    info!("Entering Dashboard");
    let fallback = html! {
        <div class="block">
            <h1 class="title">{format!("Fetching Analysis `...`")}</h1>
            <progress class="progress is-large is-primary" max="100" />
        </div>
    };

    html! {
        <Suspense {fallback}>
            <DashboardDataFetcher />
        </Suspense>
    }
}
