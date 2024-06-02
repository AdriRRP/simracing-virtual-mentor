use plotly::{Plot, Scatter};
use plotly::color::Rgb;
use plotly::common::{Anchor, AxisSide, DashType, Font, Label, Orientation, Title};
use plotly::layout::{Axis, HoverMode, Legend};
use plotly::layout::update_menu::{Button, ButtonMethod, UpdateMenu, UpdateMenuDirection};
use yew::prelude::*;
use shared::lap::domain::laps::Laps;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or(None)]
    pub session: Option<Laps>
}

#[function_component(ScatterPlot)]
pub fn plot_component(props: &Props) -> Html {
    let Props { session } = props.clone();
    let Some(session) = session  else {
        return html! { <div class="lds-ring"><div></div><div></div><div></div><div></div></div> }
    };

    let f = yew_hooks::use_async::<_, _, ()>({
        let session = session.clone();
        async move {
            let id = session.id.clone();
            let plot = create_plot(session);
            plotly::bindings::new_plot(id.as_ref(), &plot).await;
            Ok(())
        }
    });

    use_effect_with(
        (),
        move |_| {
            f.run();
            || ()
        },
    );
    html! { <div id={session.id}></div> }
}

fn create_plot(laps: Laps) -> Plot {
    // Color Palette
    let background_color = Rgb::new(18, 18, 18);
    let surface_color = Rgb::new(53, 54, 58);
    let primary_color = Rgb::new(255, 255, 255);
    let secondary_color = Rgb::new(120, 120, 120);
    let passive_color = Rgb::new(80, 80, 80);

    // Font
    let font = Font::new()
        .color(primary_color);

    // Plot
    let mut plot = Plot::new();

    // Hover template
    let hover_template = "<b>%{y}km/h</b>";

    let lap_idx = 0;

    session.players.iter().for_each(|player| {
        let lap = player.laps.get(lap_idx).unwrap();
        let x = lap.distances.clone();
        let y = lap.velocity.clone();
        plot.add_trace(
            Scatter::new(x, y)
                .name(player.name.clone())
                .show_legend(true)
                .text_font(font.clone())
                .hover_template(hover_template)
                .web_gl_mode(true)
        );
    });

    // Layout
    let layout =
        plotly::Layout::new()
            .title(
                Title::new("velocity")
                    .x(0.05)
                    .font(font.clone().size(35))
            )
            .paper_background_color(surface_color)
            .plot_background_color(surface_color)
            .font(font.clone())
            .x_axis(
                Axis::new()
                    .title(
                        Title::new("Km")
                            .font(font.clone())
                    )
                    .show_spikes(true)
                    .spike_dash(DashType::DashDot)
                    .spike_thickness(1)
                    .spike_color(secondary_color)
                    .show_line(true)
                    .line_color(secondary_color)
                    .line_width(1)
            )
            .y_axis(
                Axis::new()
                    .title(
                        Title::new("Km/h")
                            .font(font.clone())
                    )
                    .show_spikes(true)
                    .spike_dash(DashType::DashDot)
                    .spike_thickness(1)
                    .spike_color(secondary_color)
                    .show_line(false)
                    .side(AxisSide::Right)
                    .show_grid(true)
                    .grid_width(0)
                    .grid_color(passive_color)
            )
            .legend(
                Legend::new()
                    .title(Title::new("Players"))
                    .x(0.0)
                    .x_anchor(Anchor::Bottom)
                    .y_anchor(Anchor::Left)
                    .y(-0.3)
                    .orientation(Orientation::Horizontal)
            )
            .hover_mode(HoverMode::XUnified)
            .hover_label(
                Label::new()
                    .font(font.clone())
                    .background_color(background_color)
                    .border_color(primary_color)
            )
            .show_legend(true)
            .update_menus(
                vec![
                    UpdateMenu::new()
                        .direction(UpdateMenuDirection::Left).buttons(
                        vec![
                            Button::new()
                                .name("Test Button")
                                .label("Test Label")
                                .method(ButtonMethod::Restyle)
                        ]
                    )
                ]
            );
    plot.set_layout(layout);


    plot
}