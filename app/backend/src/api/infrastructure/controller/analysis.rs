use crate::api::infrastructure::event::tokio_bus::TokioBus;
use crate::api::infrastructure::repository::mongo::analysis::Mongo as AnalysisRepository;
use crate::api::infrastructure::repository::mongo::lap::Mongo as LapRepository;

use shared::analysis::application::create::service::Creator;
use shared::analysis::application::delete::service::Deleter;
use shared::analysis::application::find::by_criteria::service::Finder as ByCriteriaFinder;
use shared::analysis::application::find::by_id::service::Finder as ByIdFinder;
use shared::analysis::application::find::header_by_criteria::service::Finder as ByCriteriaHeaderFinder;
use shared::analysis::application::find::header_by_id::service::Finder as ByIdHeaderFinder;
use shared::analysis::domain::analyses::Analyses;
use shared::analysis::domain::analysis::header::Header;
use shared::analysis::domain::analysis::headers::Headers;
use shared::analysis::domain::analysis::Analysis;
use shared::common::domain::criteria::Criteria;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Args {
    id: Uuid,
    name: String,
    ref_lap_id: Uuid,
    target_lap_id: Uuid,
}

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn creator(
    State(creator): State<Arc<Creator<AnalysisRepository, LapRepository, TokioBus>>>,
    Json(args): Json<Args>,
) -> Result<StatusCode, (StatusCode, String)> {
    match creator
        .create(
            args.id,
            args.name,
            Utc::now(),
            args.ref_lap_id,
            args.target_lap_id,
        )
        .await
    {
        Ok(()) => Ok(StatusCode::CREATED),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn find_by_criteria(
    State(finder): State<Arc<ByCriteriaFinder<AnalysisRepository>>>,
    Json(criteria): Json<Criteria>,
) -> Result<Json<Analyses>, (StatusCode, String)> {
    let analyses = finder.find(&criteria).await;
    match analyses {
        Ok(Some(analyses)) => Ok(Json(analyses)),
        Ok(None) => {
            let msg = if criteria.is_empty() {
                "No analyses found"
            } else {
                "No analyses found with given criteria"
            };
            Err((StatusCode::NOT_FOUND, msg.to_string()))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn find_by_id(
    State(finder): State<Arc<ByIdFinder<AnalysisRepository>>>,
    Path(analysis_id): Path<Uuid>,
) -> Result<Json<Analysis>, (StatusCode, String)> {
    let analysis = finder.find(&analysis_id).await;
    match analysis {
        Ok(Some(analysis)) => Ok(Json(analysis)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            format!("No analysis found with id `{analysis_id}`"),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn find_headers_by_criteria(
    State(finder): State<Arc<ByCriteriaHeaderFinder<AnalysisRepository>>>,
    Json(criteria): Json<Criteria>,
) -> Result<Json<Headers>, (StatusCode, String)> {
    let headers = finder.find(&criteria).await;
    match headers {
        Ok(Some(headers)) => Ok(Json(headers)),
        Ok(None) => {
            let msg = if criteria.is_empty() {
                "No analyses found"
            } else {
                "No analyses found with given criteria"
            };
            Err((StatusCode::NOT_FOUND, msg.to_string()))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn find_header_by_id(
    State(finder): State<Arc<ByIdHeaderFinder<AnalysisRepository>>>,
    Path(analysis_id): Path<Uuid>,
) -> Result<Json<Header>, (StatusCode, String)> {
    let header = finder.find(&analysis_id).await;
    match header {
        Ok(Some(header)) => Ok(Json(header)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            format!("No analysis found with id `{analysis_id}`"),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn delete(
    State(deleter): State<Arc<Deleter<AnalysisRepository>>>,
    Path(analysis_id): Path<Uuid>,
) -> Result<(), (StatusCode, String)> {
    let result = deleter.delete(&analysis_id).await;
    result.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}
