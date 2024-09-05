use shared::analysis::domain::analysis::reference_lap::ReferenceLap;
use shared::analysis::domain::analysis::Analysis;
use shared::lap::domain::lap::variables::Variables;

use plotly::color::{Color, NamedColor, Rgb};
use plotly::common::{AxisSide, Font, Label, Line};
use plotly::layout::Margin;
use plotly::layout::{Axis, HoverMode};
use plotly::{Layout, Plot, Scatter};
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub enum Type {
    Speed,
    Throttle,
    Brake,
    Gear,
    SteeringWheelAngle,
}

impl Display for Type {
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
pub fn create(plot_type: &Type, analysis: &Analysis) -> Result<Plot, String> {
    let reference = analysis
        .reference
        .clone()
        .ok_or_else(|| "No reference found".to_string())?;
    let target = analysis
        .target
        .clone()
        .ok_or_else(|| "No target found".to_string())?;
    let differences = analysis
        .differences
        .clone()
        .ok_or_else(|| "No differences found".to_string())?;
    let distances = analysis.union_distances.clone();

    let mut plot = Plot::new();
    let layout = base_layout();
    traces(
        &mut plot,
        plot_type,
        &reference,
        &target,
        differences,
        distances,
    );
    plot.set_layout(layout);
    Ok(plot)
}

fn base_layout() -> Layout {
    let bulma_background = Rgb::new(20, 22, 26);
    Layout::new()
        .x_axis(
            Axis::new()
                .spike_color(NamedColor::DarkGray)
                .show_tick_labels(false)
                .show_line(false),
        )
        .y_axis(
            Axis::new()
                .fixed_range(true)
                .show_spikes(false)
                .show_line(false),
        )
        .y_axis2(
            Axis::new()
                .side(AxisSide::Right)
                .fixed_range(true)
                .show_spikes(false)
                .show_line(false),
        )
        .paper_background_color(bulma_background)
        .plot_background_color(bulma_background)
        .hover_label(
            Label::new()
                .background_color(NamedColor::Black)
                .border_color(NamedColor::DarkGray)
                .font(Font::new().color(NamedColor::DarkGray)),
        )
        .hover_mode(HoverMode::XUnified)
        .height(150)
        .margin(Margin::new().top(10).bottom(10).left(10).right(10))
}

fn select_metrics(
    plot_type: &Type,
    reference: &ReferenceLap,
    target: &ReferenceLap,
    difference: Variables,
    distances: Vec<f32>,
) -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>, &'static str) {
    match plot_type {
        Type::Speed => (
            distances,
            reference.variables.speed.clone(),
            target.variables.speed.clone(),
            difference.speed,
            "%{y:.1f} km/h",
        ),
        Type::Throttle => (
            distances,
            reference.variables.throttle.clone(),
            target.variables.throttle.clone(),
            difference.throttle,
            "%{y:.2f}",
        ),
        Type::Brake => (
            distances,
            reference.variables.brake.clone(),
            target.variables.brake.clone(),
            difference.brake,
            "%{y:.2f}",
        ),
        Type::Gear => (
            distances,
            reference
                .variables
                .gear
                .clone()
                .iter()
                .map(|&x| f32::from(x))
                .collect(),
            target
                .variables
                .gear
                .clone()
                .iter()
                .map(|&x| f32::from(x))
                .collect(),
            difference.gear.iter().map(|&x| f32::from(x)).collect(),
            "Gear %{y:.0f}",
        ),
        Type::SteeringWheelAngle => (
            distances,
            reference.variables.steering_wheel_angle.clone(),
            target.variables.steering_wheel_angle.clone(),
            difference.steering_wheel_angle,
            "%{y:.2f} rad",
        ),
    }
}

fn traces(
    plot: &mut Plot,
    plot_type: &Type,
    reference: &ReferenceLap,
    target: &ReferenceLap,
    difference: Variables,
    distances: Vec<f32>,
) {
    let (x, y_ref, y_target, y_diff, hover) =
        select_metrics(plot_type, reference, target, difference, distances);
    trace(
        plot,
        &format!("reference {plot_type}",),
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
        x,
        y_target,
        "x",
        "y",
        NamedColor::Green,
        hover,
    );
}

#[allow(clippy::too_many_arguments)]
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
