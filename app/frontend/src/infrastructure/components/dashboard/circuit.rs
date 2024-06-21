use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::__rt::IntoJsResult;
use web_sys::HtmlCanvasElement;
use wasm_bindgen::closure::Closure;
use yew::MouseEvent;
use crate::infrastructure::components::dashboard::Canvas;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GpsCoord {
    lat: f64,
    lon: f64,
    dist: f32,
}

impl GpsCoord {
    pub fn new(lat: f64, lon: f64, dist: f32) -> Self {
        Self {lat, lon, dist}
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
    dist: f32,
}

impl Point {
    pub fn new(x: f64, y: f64, dist: f32) -> Self {
        Self {x, y, dist}
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn lat_lon_to_xy(lat: f64, lon: f64) -> (f64, f64) {
    let r = 6378137.0;
    let x = r * lon.to_radians();
    let y = r * (lat.to_radians().tan() + (std::f64::consts::PI / 4.0)).ln();
    (x, y)
}

fn normalize_coordinates(coords: Vec<GpsCoord>, width: f64, height: f64) -> Vec<Point> {
    let min_lat = coords.iter().map(|c| c.lat).fold(f64::INFINITY, f64::min);
    let max_lat = coords.iter().map(|c| c.lat).fold(f64::NEG_INFINITY, f64::max);
    let min_lon = coords.iter().map(|c| c.lon).fold(f64::INFINITY, f64::min);
    let max_lon = coords.iter().map(|c| c.lon).fold(f64::NEG_INFINITY, f64::max);

    coords.iter()
        .map(|GpsCoord{ lat, lon, dist}| Point {
            x: ((*lat - min_lat) / (max_lat - min_lat)) * width,
            y: ((*lon - min_lon) / (max_lon - min_lon)) * height,
            dist: *dist,
        })
        .collect()
}

fn find_nearest_point(points: &Vec<Point>, mouse_x: f64, mouse_y: f64) -> Option<Point> {
    let mut min_dist = f64::INFINITY;
    let mut nearest_point = None;

    for point in points.iter() {
        let dx = point.x - mouse_x;
        let dy = point.y - mouse_y;
        let d = (dx * dx + dy * dy).sqrt();
        if d < min_dist {
            min_dist = d;
            nearest_point = Some(point.clone());
        }
    }

    nearest_point
}

pub async fn create_pointer_layer(id: &str, width: f64, height: f64, lat: &[f64], lon: &[f64], dist: &[f32]) -> Result<(),Error> {

    let gps_coords = gps_coord(lat, lon, dist);
    let normalized_points = normalize_coordinates(gps_coords, width, height);

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let x_margin = 0.;
    let y_margin = 0.;

    if let Some(start) = normalized_points.iter().min_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap()) {
        context.set_fill_style(&JsValue::from_str("green"));
        context.begin_path();
        context.arc(start.x + x_margin, start.y + y_margin, 8.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
        context.fill();
    }

    Ok(())
}
pub async fn create_circuit(id: &str, width: f64, height: f64, lat: &[f64], lon: &[f64], dist: &[f32]) -> Result<(),Error> {

    let gps_coords = gps_coord(lat, lon, dist);
    let normalized_points = normalize_coordinates(gps_coords, width, height);

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let x_margin = 0.;
    let y_margin = 0.;


    let my_gradient = context.create_linear_gradient(0., 0., 170., 0.);
    my_gradient.add_color_stop(0., "black");
    my_gradient.add_color_stop(0.5, "red");
    my_gradient.add_color_stop(1., "white");

    let js_gradient = my_gradient.into_js_result().unwrap();

    for point in normalized_points.iter() {

        context.set_fill_style(&js_gradient);
        context.begin_path();
        context.arc(point.x + x_margin, point.y + y_margin, 3.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
        context.fill();
    }

    Ok(())
}

fn gps_coord(lat: &[f64], lon: &[f64], dist: &[f32]) -> Vec<GpsCoord> {
    // Asegúrate de que los slices tienen la misma longitud
    assert_eq!(lat.len(), lon.len());
    assert_eq!(lat.len(), dist.len());

    // Itera sobre los slices y crea los Points
    lat.iter().zip(lon.iter()).zip(dist.iter())
        .map(|((&lat, &lon), &d)| GpsCoord::new(lat, lon, d))
        .collect()
}

pub async fn add_mouse_move_event(id: String, width: f64, height: f64, lat: &[f64], lon: &[f64], dist: &[f32]) -> Result<(),String> {
    let gps_coords = gps_coord(lat, lon, dist);
    let normalized_points = normalize_coordinates(gps_coords, width, height);

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(&id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let mouse_move_closure = {
        let canvas = canvas.clone();
         Closure::wrap(Box::new(move |event: MouseEvent| {
            let points = normalized_points.clone();

            let rect = canvas.get_bounding_client_rect();
            let mouse_x = event.client_x() as f64 - rect.left();
            let mouse_y = event.client_y() as f64 - rect.top();

            let points = points.clone();
            let nearest_point = find_nearest_point(&points, mouse_x, mouse_y);

            if let Some(point) = nearest_point {
                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap();

                let x_margin = 0.;
                let y_margin = 0.;

                context.set_fill_style(&JsValue::from_str("green"));
                context.begin_path();
                context.arc(point.x + x_margin, point.y + y_margin, 8.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
                context.fill();
            }
        }) as Box<dyn FnMut(_)>)
    };

    canvas.add_event_listener_with_callback("mousemove", mouse_move_closure.as_ref().unchecked_ref()).unwrap();
    //canvas.add_event_listener_with_callback("mousedown", mouse_down_closure.as_ref().unchecked_ref()).unwrap();
    //canvas.add_event_listener_with_callback("mouseup", mouse_up_closure.as_ref().unchecked_ref()).unwrap();
    
    Ok(())
}

#[derive(PartialEq, Eq, Debug, thiserror::Error)]
pub enum Error {
    #[error("the reference lap has been run on Circuit `{0}` while the target lap has been run on Circuit `{1}`")]
    DifferentCircuits(String, String),
}
