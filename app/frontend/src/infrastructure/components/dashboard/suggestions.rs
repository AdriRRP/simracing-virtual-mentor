use crate::utils::f64_as_usize;

use shared::analysis::domain::analysis::clusters_memberships::ClustersMemberships;
use shared::analysis::domain::analysis::tag::Base;
use shared::analysis::domain::analysis::tag::Tag;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::CustomEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub memberships: ClustersMemberships,
}

#[function_component(Suggestions)]
pub fn suggestions(props: &Props) -> Html {
    let suggestion_html = use_state(|| {
        let mut suggestions = vec![
            html! { <pre class="stay">{ "To start receiving suggestions, select a point on the map or hover over a graph." }</pre> },
        ];
        suggestions.extend((0..4).map(|_| html! { <pre class="stay">{ " " }</pre> }));
        suggestions
    });

    {
        let suggestion_html = suggestion_html.clone();
        let memberships = props.memberships.clone();

        use_effect(move || {
            let closure = Closure::<dyn FnMut(_)>::new(move |event: CustomEvent| {
                if let Some(index) = event.detail().as_f64() {
                    let index = f64_as_usize(index);

                    // Generar la lista de elementos <pre> en función de memberships y el índice
                    let suggestions = generate_suggestion(index, &memberships);
                    suggestion_html.set(suggestions); // Setea Vec<Html> en el estado
                }
            });

            let document = web_sys::window().unwrap().document().unwrap();
            document
                .add_event_listener_with_callback(
                    "suggestion-event",
                    closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            closure.forget();

            || () // Función de limpieza vacía
        });
    }

    html! {
        <div class="mt-4 ml-4">
            <div class="is-size-3 has-text-centered	">
                {"Suggestions"}
            </div>
            // Unix-style console for messages
            <div class="console">
                { for (*suggestion_html).clone() }
            </div>
        </div>
    }
}
fn generate_suggestion(index: usize, memberships: &ClustersMemberships) -> Vec<Html> {
    let mut suggestions = Vec::new();

    // Interpretar el Tag de speed
    if let Some(tag) = memberships.speed_tags.get(index) {
        let (message, css_class) = interpret_tag("speed", *tag);
        suggestions.push(html! {
            <pre class={css_class}>{ message }</pre>
        });
    }

    // Interpretar el Tag de throttle
    if let Some(tag) = memberships.throttle_tags.get(index) {
        let (message, css_class) = interpret_tag("throttle", *tag);
        suggestions.push(html! {
            <pre class={css_class}>{ message }</pre>
        });
    }

    // Interpretar el Tag de brake
    if let Some(tag) = memberships.brake_tags.get(index) {
        let (message, css_class) = interpret_tag("brake", *tag);
        suggestions.push(html! {
            <pre class={css_class}>{ message }</pre>
        });
    }

    // Interpretar el Tag de gear
    if let Some(tag) = memberships.gear_tags.get(index) {
        let (message, css_class) = interpret_tag("gear", *tag);
        suggestions.push(html! {
            <pre class={css_class}>{ message }</pre>
        });
    }

    // Interpretar el Tag de steering_wheel_angle
    if let Some(tag) = memberships.steering_wheel_angle_tags.get(index) {
        let (message, css_class) = interpret_tag("steering wheel angle", *tag);
        suggestions.push(html! {
            <pre class={css_class}>{ message }</pre>
        });
    }

    suggestions
}

fn interpret_tag(variable: &str, tag: Tag) -> (String, String) {
    match tag {
        Tag::Single(base) => match base {
            Base::Stay => (
                format!("You can maintain the current {variable}."),
                "stay".to_string(),
            ),
            Base::Increase(_) => (
                format!("You should increase the {variable}."),
                "increase".to_string(),
            ),
            Base::Reduce(_) => (
                format!("You should reduce the {variable}."),
                "reduce".to_string(),
            ),
        },
        Tag::Tendency(base1, base2) => {
            let action1 = match base1 {
                Base::Stay => format!("maintain the current {variable}"),
                Base::Increase(level) => format!(
                    "increase the {}{}",
                    variable,
                    match level {
                        0 => "",
                        1 => " significantly",
                        2 => " greatly",
                        _ => " a lot",
                    }
                ),
                Base::Reduce(level) => format!(
                    "reduce the {}{}",
                    variable,
                    match level {
                        0 => "",
                        1 => " significantly",
                        2 => " greatly",
                        _ => " a lot",
                    }
                ),
            };

            let action2 = match base2 {
                Base::Stay => "maintain it",
                Base::Increase(level) => &format!(
                    "start increasing it{}",
                    match level {
                        0 => "",
                        1 => " significantly",
                        2 => " greatly",
                        _ => " a lot",
                    }
                ),
                Base::Reduce(level) => &format!(
                    "start reducing it{}",
                    match level {
                        0 => "",
                        1 => " significantly",
                        2 => " greatly",
                        _ => " a lot",
                    }
                ),
            };

            let message = format!("You should {action1}, but it would be good to {action2}.");

            let css_class = match (base1, base2) {
                (Base::Stay, Base::Increase(_)) | (Base::Increase(_), Base::Stay) => {
                    "stay_to_increase".to_string()
                }
                (Base::Stay, Base::Reduce(_)) | (Base::Reduce(_), Base::Stay) => {
                    "stay_to_reduce".to_string()
                }
                (Base::Increase(_), Base::Increase(_)) => "increase".to_string(),
                (Base::Reduce(_), Base::Reduce(_)) => "reduce".to_string(),
                _ => "stay".to_string(),
            };

            (message, css_class)
        }
    }
}
