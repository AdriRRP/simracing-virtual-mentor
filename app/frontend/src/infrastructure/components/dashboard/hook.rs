use crate::infrastructure::repository::analysis::http::Http as AnalysisHttpRepository;
use crate::infrastructure::settings::Settings;
use symracing_virtual_mentor_shared::analysis::domain::analysis::Analysis;

use log::{error, info};
use plotly::Plot;
use std::rc::Rc;
use std::time::Duration;
use web_sys::HtmlElement;

use shared::common::domain::criteria::Criteria;
use shared::lap::domain::laps::Laps;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

#[hook]
pub fn use_analyses(
    criteria: &str,
    repo: AnalysisHttpRepository,
) -> SuspensionResult<Option<Analysis>> {
    let result_handle = use_state(|| None);
    let result = (*result_handle).clone();

    let suspension_hanlde = use_state(|| {
        let criteria = criteria.to_owned();
        Suspension::from_future(async move {
            match repo.find_by_criteria(&Criteria::default()).await {
                Ok(Some(found_analyses)) => {
                    let selection = found_analyses.first().unwrap().clone();
                    result_handle.set(Some(selection))
                }
                Ok(None) => {
                    error!("No analyses found");
                    result_handle.set(None)
                }
                Err(e) => {
                    error!("Error fetching analyses: {e}");
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
