use crate::infrastructure::repository::lap::http::Http as LapsHttpRepository;
use crate::infrastructure::settings::Settings;

use log::{error, info};
use plotly::Plot;
use std::rc::Rc;
use std::time::Duration;
use web_sys::HtmlElement;

use shared::lap::domain::laps::Laps;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

#[hook]
pub fn use_laps(criteria: &str, repo: LapsHttpRepository) -> SuspensionResult<Option<Laps>> {
    let result_handle = use_state(|| None);
    let result = (*result_handle).clone();

    let suspension_hanlde = use_state(|| {
        let criteria = criteria.to_owned();
        Suspension::from_future(async move {
            match repo.find_by_criteria(&criteria).await {
                Ok(Some(found_laps)) => {
                    let selection = &found_laps[3..6];
                    let mut selected_laps = Vec::new();
                    for lap in selection {
                        selected_laps.push(lap.clone())
                    }
                    let selected_laps = Laps::from(selected_laps);
                    result_handle.set(Some(selected_laps))
                }
                Ok(None) => {
                    error!("No laps found");
                    result_handle.set(None)
                }
                Err(e) => {
                    error!("Error fetching laps: {e}");
                    result_handle.set(None)
                }
            }
        })
    });

    let suspension = (*suspension_hanlde).clone();
    if suspension.resumed() {
        return match result {
            Some(v) => Ok(Some(v)),
            None => Err(suspension),
        };
    }
    Err(suspension)
}

#[hook]
pub fn use_plotly_draw(plot: Plot, div_id: String, div_ref: NodeRef) -> SuspensionResult<()> {
    if let Some(element) = div_ref.cast::<HtmlElement>() {
        info!("{:?} exists!", element)
    }

    let suspension = Suspension::from_future(async move {
        let div_id = div_id.clone();
        let plot = plot.clone();
        info!("Ready to paint in div `{div_id}`");
        plotly::bindings::new_plot(&div_id, &plot).await
    });

    if suspension.resumed() {
        Ok(())
    } else {
        error!("Can't complete suspension");
        Err(suspension)
    }
}
