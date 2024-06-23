use chrono::Utc;
use chrono::{DateTime, NaiveDate};
use crate::infrastructure::components::laps::filter::_Props::on_filter_change;

use shared::common::domain::criteria::Criteria;
use shared::common::domain::criteria::filter::condition::Condition;
use shared::common::domain::criteria::filter::field::Field;
use shared::common::domain::criteria::filter::Filter;
use shared::common::domain::criteria::filter::value::Value;
use shared::common::domain::criteria::filters::Filters;

use web_sys::{HtmlInputElement, HtmlSelectElement};
use log::{info, debug};
use yew::{function_component, html, Html, props};
use yew::prelude::*;
use chrono::NaiveDateTime;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_filter_change: Callback<Criteria>
}

#[function_component(LapFilter)]
pub fn lap_filter(props: &Props) -> Html {

    let criteria_state = use_state(|| Criteria::default());
    let parent_callback = props.on_filter_change.clone();

    let on_status_change = {
        let criteria_state = criteria_state.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            let status = select.value();

            let value = match status.as_str() {
                "Accepted" | "Success" | "Error" => Some(status),
                _ => None,
            };

            let criteria = set_criteria(criteria_state.clone(), "status", Condition::Contains, value);

            debug!("{:?}", criteria.clone());
            criteria_state.set(criteria);
        })
    };

    let on_date_change = {
        let criteria_state = criteria_state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let date = input.value();
            let input_id = input.id();

            let date: Option<DateTime<Utc>> = NaiveDate::parse_from_str(&date, "%Y-%m-%d").map(|nd| {
                let time = chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap();// TODO: revisar
                // Convert NaiveDate to NaiveDateTime at midnight
                let naive_datetime = NaiveDateTime::new(nd, time);
                // Convert NaiveDateTime to DateTime<Utc>
                DateTime::from_naive_utc_and_offset(naive_datetime, Utc)
            }).ok();

            let date = date.map(|d| d.to_rfc3339());

            let condition = match input_id.to_lowercase().as_str() {
                "before" => Condition::LowerThan,
                "after" => Condition::GreaterThan,
                _ => Condition::LowerThan,
            };

            let criteria = set_criteria(criteria_state.clone(), "created_on", condition, date);

            debug!("{:?}", criteria.clone());
            criteria_state.set(criteria);
        })
    };

    let on_name_change = {
        let criteria_state = criteria_state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = if input.value().is_empty() {None} else { Some(input.value()) };
            
            let criteria = set_criteria(criteria_state.clone(), "name", Condition::Contains, value);

            debug!("{:?}", criteria.clone());
            criteria_state.set(criteria);
        })
    };

    let on_reset = {
        let criteria_state = criteria_state.clone();
        let parent_callback = parent_callback.clone();
        Callback::from(move |_| {
            criteria_state.set(Criteria::default());
            parent_callback.emit(Criteria::default());
        })
    };

    let on_submit = {
        let criteria_state = criteria_state.clone();
        let parent_callback = parent_callback.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            parent_callback.emit((*criteria_state).clone());
        })
    };

    html! {
        <div class="box mt-4">
            <h2 class="subtitle is-5">{"Filter"}</h2>
            <form>

                <div class="has-2-cols field">
                    <label class="label">{"Name"}</label>
                    <div class="control">
                        <input
                            class="input"
                            type="text"
                            placeholder="Name contains this text..."
                            onchange={on_name_change}
                        />
                    </div>
                </div>

                <div class="grid">
                    <div class="field">
                        <label class="label">{"Status"}</label>
                        <div class="control">
                            <div class="select">
                                <select onchange={on_status_change}>
                                    <option>{"Accepted"}</option>
                                    <option>{"Success"}</option>
                                    <option>{"Error"}</option>
                                    <option selected={true}>{"..."}</option>
                                </select>
                            </div>
                        </div>
                    </div>
                    <div class="field">
                        <label class="label">{"Before"}</label>
                        <div class="control">
                            <div class="select">
                                <input
                                    type="date"
                                    class="input"
                                    id="before"
                                    onchange={on_date_change.clone()}
                                />
                            </div>
                        </div>
                    </div>
                    <div class="field">
                        <label class="label">{"After"}</label>
                        <div class="control">
                            <div class="select">
                                <input
                                    type="date"
                                    class="input"
                                    id="after"
                                    onchange={on_date_change}
                                />
                            </div>
                        </div>
                    </div>
                </div>
                <div class="field is-grouped">
                    <div class="control">
                        <button
                            class="button is-link"
                            onclick={on_submit}
                        >{"Submit"}</button>
                    </div>
                    <div class="control">
                        <input
                            type="reset"
                            class="button is-link is-light"
                            onclick={on_reset}
                            value="Clear"
                        />
                    </div>
                </div>

            </form>
        </div>
    }
}

fn set_criteria(criteria_state: UseStateHandle<Criteria>, field: &str, condition: Condition, value: Option<String>) -> Criteria {
    let mut criteria = (*criteria_state).clone();
    let mut filters = criteria.filters.clone().unwrap_or_else(Filters::default);

    // Remove existing `field` filters
    filters.retain(|f| !f.field.name().contains(field));

    if let Some(value) = value {
        // Add new filter
        let field = Field::new(field);
        let value = Value::new(value.to_lowercase().as_str());
        let filter = Filter::new(field, condition, value);
        filters.push(filter);
    }

    // Update criteria filters
    criteria.filters = if filters.is_empty() { None } else { Some(filters) };

    criteria.clone()
}
