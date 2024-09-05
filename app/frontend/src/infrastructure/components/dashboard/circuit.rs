use crate::utils::{f32_as_f64, f32_as_i32, f64_as_f32, usize_as_f64};

use crate::infrastructure::components::dashboard::suggestions;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::CustomEvent;
use web_sys::{CanvasRenderingContext2d, CustomEventInit, HtmlCanvasElement, MouseEvent};
use yew::prelude::*;

pub const CANVAS_ID: &str = "circuit_canvas";
pub const CANVAS_HEIGHT: f64 = 480.;
pub const CANVAS_WIDTH: f64 = 800.;
pub const CANVAS_MARGIN: f64 = 50.;
pub const UPDATE_CIRCUIT_POINTER_EVENT: &str = "update_circuit_pointer";

#[wasm_bindgen(module = "/assets/scripts/plotly_interop.js")]
extern "C" {
    #[wasm_bindgen(js_name = "updatePlotlyHover")]
    fn updatePlotlyHover(div_ids: &JsValue, point: &JsValue);
}

#[wasm_bindgen]
pub fn hover_event_from_plotly(distance: f32) {
    let document = web_sys::window().unwrap().document().unwrap();

    let mut event_init = CustomEventInit::new();
    event_init.detail(&JsValue::from_f64(f32_as_f64(distance)));

    let event =
        CustomEvent::new_with_event_init_dict(UPDATE_CIRCUIT_POINTER_EVENT, &event_init).unwrap();
    document.dispatch_event(&event).unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GpsCoord {
    lat: f64,
    lon: f64,
    dist: f32,
}

impl GpsCoord {
    pub const fn new(lat: f64, lon: f64, dist: f32) -> Self {
        Self { lat, lon, dist }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
    dist: f32,
}

impl Point {
    pub const fn new(x: f64, y: f64, dist: f32) -> Self {
        Self { x, y, dist }
    }
}

fn normalize_coordinates(coords: &[GpsCoord], width: f64, height: f64, margin: f64) -> Vec<Point> {
    let min_lat = coords.iter().map(|c| c.lat).fold(f64::INFINITY, f64::min);
    let max_lat = coords
        .iter()
        .map(|c| c.lat)
        .fold(f64::NEG_INFINITY, f64::max);
    let min_lon = coords.iter().map(|c| c.lon).fold(f64::INFINITY, f64::min);
    let max_lon = coords
        .iter()
        .map(|c| c.lon)
        .fold(f64::NEG_INFINITY, f64::max);

    coords
        .iter()
        .map(|GpsCoord { lat, lon, dist }| Point {
            x: ((*lat - min_lat) / (max_lat - min_lat))
                .mul_add(2.0f64.mul_add(-margin, width), margin),
            y: ((*lon - min_lon) / (max_lon - min_lon))
                .mul_add(2.0f64.mul_add(-margin, height), margin),
            dist: *dist,
        })
        .collect()
}

fn find_nearest_point_with_index(
    points: &[Point],
    mouse_x: f64,
    mouse_y: f64,
) -> Option<(Point, usize)> {
    let mut min_dist = f64::INFINITY;
    let mut nearest_point = None;

    for (index, point) in points.iter().enumerate() {
        let dx = point.x - mouse_x;
        let dy = point.y - mouse_y;
        let d = dx.hypot(dy);
        if d < min_dist {
            min_dist = d;
            nearest_point = Some((point.clone(), index));
        }
    }

    nearest_point
}

fn find_nearest_point_by_distance(points: &[Point], distance: f32) -> Option<Point> {
    points
        .iter()
        .min_by_key(|p| f32_as_i32((p.dist - distance).abs() * 1000.0))
        .cloned()
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub width: f64,
    pub height: f64,
    pub margin: f64,
    pub latitudes: Vec<f64>,
    pub longitudes: Vec<f64>,
    pub distances: Vec<f32>,
}

#[function_component(Circuit)]
pub fn circuit(props: &Props) -> Html {
    let canvas_ref = use_node_ref();
    let width = props.width;
    let height = props.height;
    let margin = props.margin;
    let latitudes = props.latitudes.clone();
    let longitudes = props.longitudes.clone();
    let distances = props.distances.clone();
    let plot_div_ids = vec![
        "speed_plot",
        "throttle_plot",
        "gear_plot",
        "brake_plot",
        "steering_wheel_angle_plot",
    ];

    let gps_coords = gps_coord(&latitudes, &longitudes, &distances);
    let normalized_points = normalize_coordinates(&gps_coords, width, height, margin);

    {
        let normalized_points = normalized_points.clone();
        let canvas_ref = canvas_ref.clone();
        use_effect_with(canvas_ref, move |canvas_ref| {
            let document = web_sys::window().unwrap().document().unwrap();
            let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::CustomEvent| {
                let distance = f64_as_f32(event.detail().as_f64().unwrap_or_default());
                if let Some(closest_point) =
                    find_nearest_point_by_distance(&normalized_points, distance)
                {
                    context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
                    draw_circuit(&context, &normalized_points);
                    context.begin_path();
                    context
                        .arc(
                            closest_point.x,
                            closest_point.y,
                            8.0,
                            0.0,
                            2.0 * std::f64::consts::PI,
                        )
                        .unwrap();
                    context.set_fill_style(&JsValue::from_str("red"));
                    context.fill();
                }
            });

            document
                .add_event_listener_with_callback(
                    UPDATE_CIRCUIT_POINTER_EVENT,
                    closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            closure.forget();

            || ()
        });
    }

    let onmousemove = {
        let canvas_ref = canvas_ref.clone();
        let normalized_points = normalized_points.clone();
        Callback::from(move |event: MouseEvent| {
            let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let rect = canvas.get_bounding_client_rect();
            let mouse_x = f64::from(event.client_x()) - rect.left();
            let mouse_y = f64::from(event.client_y()) - rect.top();
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            // Encontrar el punto más cercano
            if let Some((closest_point, index)) =
                find_nearest_point_with_index(&normalized_points, mouse_x, mouse_y)
            {
                // Redibujar el canvas
                context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
                draw_circuit(&context, &normalized_points);
                context.begin_path();
                context
                    .arc(
                        closest_point.x,
                        closest_point.y,
                        8.0,
                        0.0,
                        2.0 * std::f64::consts::PI,
                    )
                    .unwrap();
                context.set_fill_style(&JsValue::from_str("red"));
                context.fill();

                // Enviar evento a Plotly
                let point_json = to_value(&closest_point).unwrap();
                let plot_div_ids_js = to_value(&plot_div_ids).unwrap();
                updatePlotlyHover(&plot_div_ids_js, &point_json);

                // Emitir un evento personalizado con el índice del punto más cercano
                let document = web_sys::window().unwrap().document().unwrap();
                let mut event_init = CustomEventInit::new();
                event_init.detail(&JsValue::from_f64(usize_as_f64(index)));
                let event = CustomEvent::new_with_event_init_dict(
                    suggestions::UPDATE_SUGGESTION_EVENT,
                    &event_init,
                )
                .unwrap();
                document.dispatch_event(&event).unwrap();
            }
        })
    };

    use_effect_with(canvas_ref.clone(), move |canvas_ref| {
        if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();
            context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
            draw_circuit(&context, &normalized_points);
        }
        || ()
    });

    html! {
        <canvas ref={canvas_ref} width={CANVAS_WIDTH.to_string()} height={CANVAS_HEIGHT.to_string()} {onmousemove}></canvas>
    }
}

fn draw_circuit(context: &CanvasRenderingContext2d, points: &[Point]) {
    context.set_stroke_style(&JsValue::from_str("white"));
    context.set_line_width(6.0);
    context.begin_path();
    for (i, point) in points.iter().enumerate() {
        if i == 0 {
            context.move_to(point.x, point.y);
        } else {
            context.line_to(point.x, point.y);
        }
    }
    context.stroke();
}

fn gps_coord(lat: &[f64], lon: &[f64], dist: &[f32]) -> Vec<GpsCoord> {
    assert_eq!(lat.len(), lon.len());
    assert_eq!(lat.len(), dist.len());

    lat.iter()
        .zip(lon.iter())
        .zip(dist.iter())
        .map(|((&lat, &lon), &d)| GpsCoord::new(lat, lon, d))
        .collect()
}
