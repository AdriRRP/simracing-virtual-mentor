use plotly::layout::{Margin, RangeSlider, SpikeMode};
use shared::analysis::domain::analysis::Analysis;
use std::fmt::{Display, format, Formatter};

use log::info;
use plotly::color::{Color, NamedColor, Rgb};
use plotly::common::{AxisSide, DashType, Font, HoverInfo, Label, Line, Mode, Reference, Title};
use plotly::layout::GridPattern;
use plotly::layout::LayoutGrid;
use plotly::layout::{Axis, HoverMode};
use plotly::{Layout, Plot, Scatter};
use serde::{Deserialize, Serialize};
use shared::ibt::domain::file::var_header::Error::Name;
use shared::lap::domain::lap::metrics::Metrics;

#[derive(Clone)]
pub enum PlotType {
    Speed,
    Throttle,
    Brake,
    Gear,
    SteeringWheelAngle
}

impl Display for PlotType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Speed => write!(f, "speed"),
            Self::Throttle => write!(f, "throttle"),
            Self::Brake => write!(f, "brake"),
            Self::Gear => write!(f, "gear"),
            Self::SteeringWheelAngle => write!(f, "steering wheel angle"),
        }
    }
}
pub fn create_plot(plot_type: &PlotType, analysis: &Analysis) -> Plot {
    let mut plot = Plot::new();
    let layout = base_layout();
    traces(&mut plot, &plot_type, &analysis);
    plot.set_layout(layout);
    plot
}

fn base_layout() -> Layout {
    let bulma_background = Rgb::new(20,22,26);
    Layout::new()
        .x_axis(
            Axis::new()
                .spike_color(NamedColor::DarkGray)
                .show_tick_labels(false)
                .show_line(false)
        )
        .y_axis(
            Axis::new()
                .fixed_range(true)
                .show_spikes(false)
                .show_line(false)
        )
        .y_axis2(
            Axis::new()
                .side(AxisSide::Right)
                .fixed_range(true)
                .show_spikes(false)
                .show_line(false)
        )
        .paper_background_color(
            bulma_background
        )
        .plot_background_color(
            bulma_background
        )
        .hover_label(
            Label::new()
                .background_color(NamedColor::Black)
                .border_color(NamedColor::DarkGray)
                .font(
                    Font::new()
                        .color(NamedColor::DarkGray)
                ),
        )
        .hover_mode(HoverMode::XUnified)
        .height(150)
        .margin(
            Margin::new()
                .top(10)
                .bottom(10)
                .left(10)
                .right(10)
        )
}

fn select_metrics(plot_type: &PlotType, analysis: &Analysis) -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>, &'static str)
{
    let distance = analysis.union_distances.clone();
    match plot_type {
        PlotType::Speed => (
            distance,
            analysis.ref_lap_metrics.speed.clone(),
            analysis.target_lap_metrics.speed.clone(),
            analysis.difference_metrics.speed.clone(),
            "%{y:.1f} km/h",
        ),
        PlotType::Throttle => (
            distance,
            analysis.ref_lap_metrics.throttle.clone(),
            analysis.target_lap_metrics.throttle.clone(),
            analysis.difference_metrics.throttle.clone(),
            "%{y:.2f}",
        ),
        PlotType::Brake => (
            distance,
            analysis.ref_lap_metrics.brake.clone(),
            analysis.target_lap_metrics.brake.clone(),
            analysis.difference_metrics.brake.clone(),
            "%{y:.2f}",
        ),
        PlotType::Gear => (
            distance,
            analysis.ref_lap_metrics.gear.clone().iter().map(|&x| f32::from(x)).collect(),
            analysis.target_lap_metrics.gear.clone().iter().map(|&x| f32::from(x)).collect(),
            analysis.difference_metrics.gear.clone().iter().map(|&x| f32::from(x)).collect(),
            "Gear %{y:.0f}"
        ),
        PlotType::SteeringWheelAngle => (
            distance,
            analysis.ref_lap_metrics.steering_wheel_angle.clone(),
            analysis.target_lap_metrics.steering_wheel_angle.clone(),
            analysis.difference_metrics.steering_wheel_angle.clone(),
            "%{y:.2f} rad",
        ),
    }
}

fn traces(plot: &mut Plot, plot_type: &PlotType, analysis: &Analysis) {
    let (x, y_ref, y_target, y_diff, hover) = select_metrics(plot_type, analysis);
    trace(
        plot,
        &format!("reference {plot_type}", ),
        x.clone(),
        y_ref,
        "x",
        "y",
        NamedColor::Red,
        hover,
    );

    trace(
        plot,
        &format!("diff {plot_type}"),
        x.clone(),
        y_diff,
        "x",
        "y2",
        NamedColor::Cyan,
        hover,
    );

    trace(
        plot,
        &format!("target {plot_type}"),
        x.clone(),
        y_target,
        "x",
        "y",
        NamedColor::Green,
        hover,
    );
}

fn trace<X, Y>(
    plot: &mut Plot,
    name: &str,
    x: Vec<X>,
    y: Vec<Y>,
    x_axis: &str,
    y_axis: &str,
    color: impl Color,
    hover: &str,
) where
    X: Serialize + Clone + 'static,
    Y: Serialize + Clone + 'static,
{
    plot.add_trace(
        Scatter::new(x, y)
            .name(name)
            .line(Line::new().width(1.0).color(color))
            .x_axis(x_axis)
            .y_axis(y_axis)
            .web_gl_mode(true)
            .show_legend(false)
            .hover_template(hover),
    );
}

pub fn create_plot2(plot_type: &PlotType, analysis: Analysis) -> Plot {
    info!("Entering create_plot()");

    let mut colors = vec![
        NamedColor::Red,
        NamedColor::Green,
        NamedColor::Orange,
        NamedColor::Olive,
        NamedColor::Lime,
        NamedColor::Aqua,
        NamedColor::Fuchsia,
        NamedColor::Navy,
        NamedColor::Blue,
        NamedColor::Black,
        NamedColor::Maroon,
    ]
    .into_iter();

    // Color Palette
    let background_color = Rgb::new(18, 18, 18);
    let surface_color = Rgb::new(53, 54, 58);
    let primary_color = Rgb::new(255, 255, 255);
    let secondary_color = Rgb::new(120, 120, 120);
    let passive_color = Rgb::new(80, 80, 80);

    // Font
    let font = Font::new().color(primary_color);

    // Plot
    let mut plot = Plot::new();

    let mut current_color = NamedColor::Red;

    colors.next().iter().for_each(|c| current_color = c.clone());
    let ref_name = format!("{} [{}]", analysis.ref_lap_driver, analysis.ref_lap_number);
    add_trace(
        &mut plot,
        ref_name,
        current_color,
        analysis.ref_lap_metrics.clone(),
    );

    colors.next().iter().for_each(|c| current_color = c.clone());
    let target_name = format!(
        "{} [{}]",
        analysis.target_lap_driver, analysis.target_lap_number
    );
    add_trace(
        &mut plot,
        target_name,
        current_color,
        analysis.target_lap_metrics.clone(),
    );

    current_color = NamedColor::Cyan;
    let name = "difference".to_string();
    add_diff_speed_trace(
        &mut plot,
        &name,
        analysis.difference_metrics.speed,
        analysis.union_distances.clone(),
        current_color,
    );
    add_diff_throttle_trace(
        &mut plot,
        &name,
        analysis.difference_metrics.throttle,
        analysis.union_distances.clone(),
        current_color,
    );

    add_map_trace(
        &mut plot,
        &name,
        analysis.ref_lap_metrics.latitude.clone(),
        analysis.ref_lap_metrics.longitude.clone(),
        analysis.union_distances,
        NamedColor::Gray,
    );

    let x_base = Axis::new()
        .show_spikes(true)
        .spike_dash(DashType::DashDot)
        .spike_thickness(1)
        .spike_color(secondary_color)
        .show_line(true)
        .line_color(secondary_color)
        .line_width(1)
        .range_slider(RangeSlider::new().visible(true));

    let y_base = Axis::new()
        .show_spikes(true)
        .spike_dash(DashType::DashDot)
        .spike_thickness(1)
        .spike_color(secondary_color)
        .show_line(false)
        .side(AxisSide::Right)
        .show_grid(true)
        .grid_width(0)
        .grid_color(passive_color)
        .fixed_range(true);

    let layout = Layout::new().grid(
        LayoutGrid::new()
            .rows(5)
            .columns(1)
            .pattern(GridPattern::Independent),
    );
    let layout = Layout::new()
        //.grid(
        //    LayoutGrid::new()
        //        .rows(5)
        //        .columns(1)
        //        .pattern(GridPattern::Independent),
        //)
        .paper_background_color(surface_color)
        .plot_background_color(surface_color)
        .font(font.clone())
        .hover_mode(HoverMode::X)
        .hover_label(
            Label::new()
                .font(font.clone())
                .background_color(background_color)
                .border_color(primary_color),
        )
        .x_axis(
            x_base
                .clone()
                .anchor("y")
                .domain(&[0.5, 1.])
                .hover_format(".2f")
                .tick_suffix(" m")
                .show_tick_labels(false)
                .title(
                    Title::new("speed")
                        .x_ref(Reference::Paper)
                        .font(font.clone()),
                )
                .side(AxisSide::Top),
        )
        .x_axis2(
            x_base
                .clone()
                .anchor("y2")
                .domain(&[0.5, 1.])
                .matches(true)
                .show_tick_labels(false)
                .hover_format(".2f")
                .tick_suffix(" m")
                .title(Title::new("throttle").font(font.clone()))
                .side(AxisSide::Top),
        )
        .x_axis3(
            x_base
                .clone()
                .anchor("y3")
                .domain(&[0.5, 1.])
                .matches(true)
                .show_tick_labels(false)
                .hover_format(".2f")
                .tick_suffix(" m")
                .title(Title::new("brake").font(font.clone()))
                .side(AxisSide::Top),
        )
        .x_axis4(
            x_base
                .clone()
                .anchor("y4")
                .domain(&[0.5, 1.])
                .matches(true)
                .show_tick_labels(false)
                .hover_format(".2f")
                .tick_suffix(" m")
                .title(Title::new("gear").font(font.clone()))
                .side(AxisSide::Top),
        )
        .x_axis5(
            x_base
                .clone()
                .anchor("y5")
                .domain(&[0.5, 1.])
                .matches(true)
                .show_tick_labels(false)
                .hover_format(".2f")
                .tick_suffix(" m")
                .title(Title::new("steering wheel angle").font(font.clone()))
                .side(AxisSide::Top),
        )
        // MAP
        .x_axis6(
            x_base
                .clone()
                .anchor("y6")
                .domain(&[0., 0.40])
                .title(Title::new("Map").font(font.clone()))
                .side(AxisSide::Top)
                .show_tick_labels(false)
                .show_line(false)
                .show_spikes(false)
                .show_grid(false),
        )
        .y_axis(
            y_base
                .clone()
                .anchor("x")
                .domain(&[0.8, 1.])
                .tick_suffix(" km/h"),
        )
        .y_axis2(
            y_base.clone().anchor("x2").domain(&[0.6, 0.75]), //.tick_suffix("%")
        )
        .y_axis3(
            y_base.clone().anchor("x3").domain(&[0.4, 0.55]), //.tick_suffix("%")
        )
        .y_axis4(y_base.clone().anchor("x4").domain(&[0.20, 0.35]))
        .y_axis5(
            y_base
                .clone()
                .anchor("x5")
                .domain(&[0., 0.15])
                .tick_suffix(" rad"),
        )
        .y_axis6(
            y_base
                .clone()
                .anchor("x6")
                .domain(&[0.3, 1.])
                .show_tick_labels(false)
                .show_line(false)
                .show_spikes(false)
                .show_grid(false),
        )
        // TODO:  Esto son las diferencias
        .y_axis7(
            y_base
                .clone()
                .anchor("x")
                .domain(&[0.8, 1.])
                .tick_suffix(" km/h")
                .side(AxisSide::Left),
        )
        .y_axis8(
            y_base
                .clone()
                .anchor("x2")
                .domain(&[0.6, 0.75])
                .side(AxisSide::Left), //
        )
        // TODO: Hasta aquí
        .height(1280)
        .width(1520);
    plot.set_layout(layout);

    info!("Plot completed");
    plot
}

fn add_speed_trace(
    plot: &mut Plot,
    lap_name: &str,
    speed: Vec<f32>,
    distance: Vec<f32>,
    color: impl Color,
) {
    plot.add_trace(
        Scatter::new(distance, speed)
            .name(lap_name)
            .line(Line::new().width(1.0).color(color))
            .web_gl_mode(true)
            .show_legend(false)
            .hover_template("%{y:.0f} km/h"),
    );
}

fn add_diff_speed_trace(
    plot: &mut Plot,
    lap_name: &str,
    speed: Vec<f32>,
    distance: Vec<f32>,
    color: impl Color,
) {
    plot.add_trace(
        Scatter::new(distance, speed)
            .name(lap_name)
            .line(Line::new().width(1.0).color(color))
            .y_axis("y7")
            .web_gl_mode(true)
            .show_legend(false)
            .hover_template("%{y:.0f} km/h"),
    );
}

fn add_throttle_trace(
    plot: &mut Plot,
    lap_name: &str,
    throttle: Vec<f32>,
    distance: Vec<f32>,
    color: impl Color,
) {
    plot.add_trace(
        Scatter::new(distance, throttle.clone())
            .name(lap_name)
            .custom_data(throttle.iter().map(|t| *t * 100.).collect())
            .line(Line::new().width(1.0).color(color))
            .x_axis("x2")
            .y_axis("y2")
            .web_gl_mode(true)
            .show_legend(false)
            .hover_template("Throttle %{customdata:.1f}%"),
    );
}

fn add_diff_throttle_trace(
    plot: &mut Plot,
    lap_name: &str,
    throttle: Vec<f32>,
    distance: Vec<f32>,
    color: impl Color,
) {
    plot.add_trace(
        Scatter::new(distance, throttle.clone())
            .name(lap_name)
            .custom_data(throttle.iter().map(|t| *t * 100.).collect())
            .line(Line::new().width(1.0).color(color))
            .x_axis("x2")
            .y_axis("y8")
            .web_gl_mode(true)
            .show_legend(false)
            .hover_template("Throttle %{customdata:.1f}%"),
    );
}
fn add_brake_trace(
    plot: &mut Plot,
    lap_name: &str,
    brake: Vec<f32>,
    distance: Vec<f32>,
    color: impl Color,
) {
    plot.add_trace(
        Scatter::new(distance, brake.clone())
            .name(lap_name)
            .custom_data(brake.iter().map(|t| *t * 100.).collect())
            .line(Line::new().width(1.0).color(color))
            .x_axis("x3")
            .y_axis("y3")
            .web_gl_mode(true)
            .show_legend(false)
            .hover_template("Brake %{customdata:.1f}%"),
    );
}
fn add_gear_trace(
    plot: &mut Plot,
    lap_name: &str,
    gear: Vec<i8>,
    distance: Vec<f32>,
    color: impl Color,
) {
    plot.add_trace(
        Scatter::new(distance, gear)
            .name(lap_name)
            .line(Line::new().width(1.0).color(color))
            .x_axis("x4")
            .y_axis("y4")
            .web_gl_mode(true)
            .show_legend(false)
            .hover_template("Gear %{y}"),
    );
}
fn add_swa_trace(
    plot: &mut Plot,
    lap_name: &str,
    swa: Vec<f32>,
    distance: Vec<f32>,
    color: impl Color,
) {
    plot.add_trace(
        Scatter::new(distance, swa)
            .name(lap_name)
            .line(Line::new().width(1.0).color(color))
            .x_axis("x5")
            .y_axis("y5")
            .web_gl_mode(true)
            .show_legend(false)
            .hover_template("%{y:.2f} rad"),
    );
}

fn add_map_trace(
    plot: &mut Plot,
    lap_name: &str,
    latitude: Vec<f64>,
    longitude: Vec<f64>,
    distance: Vec<f32>,
    color: impl Color,
) {
    plot.add_trace(
        Scatter::new(latitude, longitude)
            .name(lap_name)
            .custom_data(distance)
            .line(Line::new().width(5.0).color(color))
            .x_axis("x6")
            .y_axis("y6")
            .web_gl_mode(true)
            .show_legend(false)
            //.hover_info(HoverInfo::Skip)
            //.hover_template("%{customdata:.0f}m ")
            .mode(Mode::LinesMarkers),
    );
}

fn add_trace(plot: &mut Plot, name: String, current_color: NamedColor, metrics: Metrics) {
    add_speed_trace(
        plot,
        &name,
        metrics.speed.clone(),
        metrics.distance.clone(),
        current_color,
    );
    add_throttle_trace(
        plot,
        &name,
        metrics.throttle.clone(),
        metrics.distance.clone(),
        current_color,
    );
    add_brake_trace(
        plot,
        &name,
        metrics.brake.clone(),
        metrics.distance.clone(),
        current_color,
    );
    add_gear_trace(
        plot,
        &name,
        metrics.gear.clone(),
        metrics.distance.clone(),
        current_color,
    );
    add_swa_trace(
        plot,
        &name,
        metrics.steering_wheel_angle.clone(),
        metrics.distance.clone(),
        current_color,
    );
    //add_map_trace(
    //    plot,
    //    &name,
    //    metrics.latitude.clone(),
    //    metrics.longitude.clone(),
    //    metrics.distance.clone(),
    //    current_color,
    //)
}
