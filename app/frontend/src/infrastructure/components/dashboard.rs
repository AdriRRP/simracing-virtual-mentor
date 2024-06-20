mod hook;
mod plot;
mod circuit;

use crate::infrastructure::components::dashboard::circuit::add_mouse_move_event;
use crate::infrastructure::components::dashboard::circuit::create_pointer_layer;
use std::cell::Cell;
use std::collections::HashMap;
use crate::infrastructure::components::dashboard::hook::{use_analyses, use_plotly_draw};
use crate::infrastructure::components::dashboard::plot::{create_plot, PlotType};
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
use shared::analysis::domain::analysis::Analysis;
use wasm_bindgen::__rt::IntoJsResult;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::Object;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, HtmlElement, MouseEvent};
use crate::infrastructure::components::dashboard::circuit::create_circuit;

const CANVAS_ID: &str = "circuit_canvas";
const CANVAS_HEIGHT: f64 = 800.;
const CANVAS_WIDTH: f64 = 800.;

// components/fx

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = Plotly, js_name = newPlot)]
    async fn js_new_plot_(id: &str, obj: &Object) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen(module = "/assets/scripts/utils.js")]
extern "C" {
    #[wasm_bindgen(js_name = "sync_plotly_hover")]
    fn sync_plotly_hover(div_id: JsValue, sync_div_ids: Vec<JsValue>);
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

pub struct PlotDiv {
    id: String,
    plot_type: PlotType,
    node_ref: NodeRef
}

impl PlotDiv {
    fn new(id: String, plot_type: PlotType) -> Self {
        Self {
            id,
            plot_type,
            node_ref: NodeRef::default(),
        }
    }
}

pub struct Canvas {
    id: String,
    height: f64,
    width: f64,
    node_ref: NodeRef,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            id: CANVAS_ID.to_string(),
            height: CANVAS_HEIGHT,
            width: CANVAS_WIDTH,
            node_ref: NodeRef::default(),
        }
    }
}

pub struct PlotlyDrawer {
    state: PlotlyDrawerState,
    plot_divs: Vec<PlotDiv>,
    canvas_pos: Canvas,
    canvas_circuit: Canvas,
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
    SyncCanvas(String),
    Error(String),
}

impl Component for PlotlyDrawer {
    type Message = PlotlyDrawerMsg;
    type Properties = PlotlyDrawerProps;
    fn create(_ctx: &Context<Self>) -> Self {

        let target_divs: Vec<PlotDiv> = vec![
            PlotDiv::new("speed_plot".to_string(), PlotType::Speed),
            PlotDiv::new("throttle_plot".to_string(), PlotType::Throttle),
            PlotDiv::new("gear_plot".to_string(), PlotType::Gear),
            PlotDiv::new("brake_plot".to_string(), PlotType::Brake),
            PlotDiv::new("steering_wheel_angle_plot".to_string(), PlotType::SteeringWheelAngle),
        ];

        Self {
            state: PlotlyDrawerState::NotFetching,
            plot_divs: target_divs,
            canvas_pos: Canvas::default(),
            canvas_circuit: Canvas::default(),
            dom_node_inserted_listener: None,
            plotly_hover_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        info!("Entering PlotlyDrawer");

        let analysis = ctx.props().analysis.clone();

        let Some(analysis) = analysis else {
            return html! {<div>{"Oooops! No Analysis found..."}</div>};
        };

        let analysis = Rc::new(analysis);

        ctx.link().send_future({
            let div_id = self.canvas_circuit.id.clone();
            let analysis = Rc::clone(&analysis);
            async move {
                info!("Starting canvas binding");

                let lat = &analysis.target_lap_metrics.latitude[..];
                let lon = &analysis.target_lap_metrics.longitude[..];
                let dist = &analysis.union_distances[..];
                match create_circuit(div_id.as_str(), CANVAS_WIDTH, CANVAS_HEIGHT, lat, lon, dist).await {
                    Ok(_) => Self::Message::SyncCanvas(div_id),
                    Err(e) => Self::Message::Error(format!("{e:?}")),
                }
            }
        });

        ctx.link().send_future({
            let div_id = self.canvas_pos.id.clone();
            let analysis = Rc::clone(&analysis);
            
            async move {
                info!("Starting canvas binding");
                let analysis = Rc::clone(&analysis);
                let lat = &analysis.target_lap_metrics.latitude[..];
                let lon = &analysis.target_lap_metrics.longitude[..];
                let dist = &analysis.union_distances[..];
                match create_pointer_layer(div_id.as_str(), CANVAS_WIDTH, CANVAS_HEIGHT, lat, lon, dist).await {
                    Ok(_) => {
                        let _ = add_mouse_move_event(div_id.clone(), CANVAS_WIDTH, CANVAS_HEIGHT, lat, lon, dist).await;
                        Self::Message::SyncCanvas(div_id.clone())
                    },
                    Err(e) => Self::Message::Error(format!("{e:?}")),
                }
            }
        });

        for target_div in &self.plot_divs {
            ctx.link().send_future({
                let plot = create_plot(&target_div.plot_type, &analysis);
                let div_id = target_div.id.clone();
                async move {
                    info!("Starting plotly binding");
                    match js_new_plot_(div_id.as_str(), &plot.to_js_object()).await {
                        Ok(_) => Self::Message::SyncPlotlyHover(div_id),
                        Err(e) => Self::Message::Error(format!("{e:?}")),
                    }
                }
            });
        }

        html! {
            <div class="fixed-grid">
                <div class="grid">
                    <div class="cell">
                        <canvas
                            id={self.canvas_circuit.id.clone()}
                            width={self.canvas_circuit.width.to_string()}
                            height={self.canvas_circuit.height.to_string()}
                            ref={self.canvas_circuit.node_ref.clone()}
                            class="px-4 py-4" />
                        <canvas
                            id={self.canvas_pos.id.clone()}
                            width={self.canvas_pos.width.to_string()}
                            height={self.canvas_pos.height.to_string()}
                            ref={self.canvas_pos.node_ref.clone()}
                            class="px-4 py-4 is-overlay" />
                    </div>
                    <div class="cell is-col-start-3" /*ref={self.target_div.clone()}*/ >
                        {
                            self.plot_divs.iter().map(|target_div| {
                                html!{<div id={target_div.id.clone()} ref={target_div.node_ref.clone()} />}
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </div>

        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::PlotlyHover => false,
            Self::Message::SyncPlotlyHover(div_id) => {
                let div_id = JsValue::from(div_id);
                let sync_div_ids: Vec<JsValue> = self.plot_divs.iter().map(|target_div| { JsValue::from(target_div.id.clone()) }).collect();
                sync_plotly_hover(div_id, sync_div_ids);
                false
            }
            Self::Message::Error(e) => {
                error!("{e}");
                // TODO: Manage error
                true
            }
            PlotlyDrawerMsg::SyncCanvas(_) => {false}
        }
    }
}

impl PlotlyDrawer {


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