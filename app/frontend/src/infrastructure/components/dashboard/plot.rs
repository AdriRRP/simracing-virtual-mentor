use log::info;
use plotly::color::{Color, NamedColor, Rgb};
use plotly::common::{AxisSide, DashType, Font, HoverInfo, Label, Line, Mode, Reference, Title};
use plotly::layout::{Axis, HoverMode};
use plotly::{Layout, Plot, Scatter};
use shared::lap::domain::laps::Laps;

pub fn create_plot(laps: Laps) -> Plot {
    info!("Entering create_plot()");
    info!("Number of laps: {}", laps.len());

    let mut colors = vec![
        NamedColor::Orange,
        NamedColor::Red,
        NamedColor::Green,
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

    laps.iter().for_each(|lap| {
        let lap_name = format!("{} [{}]", lap.header.driver.clone(), lap.header.number);

        colors.next().iter().for_each(|c| current_color = c.clone());

        add_speed_trace(
            &mut plot,
            &lap_name,
            lap.metrics.speed.clone(),
            lap.metrics.distance.clone(),
            current_color,
        );
        add_throttle_trace(
            &mut plot,
            &lap_name,
            lap.metrics.throttle.clone(),
            lap.metrics.distance.clone(),
            current_color,
        );
        add_brake_trace(
            &mut plot,
            &lap_name,
            lap.metrics.brake.clone(),
            lap.metrics.distance.clone(),
            current_color,
        );
        add_gear_trace(
            &mut plot,
            &lap_name,
            lap.metrics.gear.clone(),
            lap.metrics.distance.clone(),
            current_color,
        );
        add_swa_trace(
            &mut plot,
            &lap_name,
            lap.metrics.steering_wheel_angle.clone(),
            lap.metrics.distance.clone(),
            current_color,
        );
        add_map_trace(
            &mut plot,
            &lap_name,
            lap.metrics.latitude.clone(),
            lap.metrics.longitude.clone(),
            lap.metrics.distance.clone(),
            current_color,
        )
    });

    let x_base = Axis::new()
        .show_spikes(true)
        .spike_dash(DashType::DashDot)
        .spike_thickness(1)
        .spike_color(secondary_color)
        .show_line(true)
        .line_color(secondary_color)
        .line_width(1);

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
    gear: Vec<u8>,
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
