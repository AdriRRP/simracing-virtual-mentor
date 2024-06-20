extern crate symracing_virtual_mentor_backend_lib as backend_lib;

use backend_lib::api::infrastructure::app_assembler::AppAssembler;
use backend_lib::api::infrastructure::controller::analysis::{
    creator as create_analysis, delete as delete_analysis,
    find_by_criteria as find_analysis_by_criteria, find_by_id as find_analysis_by_id,
    find_header_by_id as find_analysis_header_by_id,
    find_headers_by_criteria as find_analysis_headers_by_criteria,
};
use backend_lib::api::infrastructure::controller::file::{
    delete as delete_file, find_by_criteria as find_file_by_criteria, find_by_id as find_file_by_id,
};
use backend_lib::api::infrastructure::controller::ibt_extractor::{
    upload, ControllerState as UploadIbtState,
};
use backend_lib::api::infrastructure::controller::lap::{
    delete as delete_lap, find_by_criteria as find_lap_by_criteria, find_by_id as find_lap_by_id,
    find_header_by_id as find_lap_header_by_id,
    find_headers_by_criteria as find_lap_headers_by_criteria,
};
use backend_lib::api::infrastructure::settings::Settings;
use backend_lib::api::infrastructure::subscriber::on_analysis_created::do_analysis::DoAnalysis;
use backend_lib::api::infrastructure::subscriber::{
    manager::Manager as SubscriberManager, on_file_deleted::delete_laps::LapDeleter,
    on_ibt_extracted::validate_file::FileValidator,
};

use axum::extract::DefaultBodyLimit;
use axum::routing::{delete, get, post, put};
use axum::Router;
use std::sync::Arc;
use tokio::io;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let settings = Settings::load().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Error loading settings: {e}"),
        )
    })?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("symracing_virtual_mentor_backend={}", settings.log_level).into()
            }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_thread_ids(true)
                .with_thread_names(true),
        )
        .init();

    let app_assembler = AppAssembler::new(&settings);

    SubscriberManager::builder()
        .with_subscriber(Arc::new(
            FileValidator::new(&app_assembler.event_bus, &app_assembler.file.validator).await,
        ))
        .with_subscriber(Arc::new(
            LapDeleter::new(
                &app_assembler.event_bus,
                &app_assembler.lap.by_criteria_finder,
                &app_assembler.lap.deleter,
            )
            .await,
        ))
        .with_subscriber(Arc::new(
            DoAnalysis::new(
                &app_assembler.event_bus,
                &app_assembler.analysis.by_id_finder,
                &app_assembler.analysis.analyzer,
                &app_assembler.analysis.updater,
            )
            .await,
        ))
        .build()
        .run();

    let router = configure_router(&app_assembler);

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", settings.server.host, settings.server.port))
            .await?;

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await?;

    Ok(())
}

fn configure_router(app_assembler: &AppAssembler) -> Router {
    let analysis_routes = Router::new()
        .route(
            "/create",
            put(create_analysis).with_state(Arc::clone(&app_assembler.analysis.creator)),
        )
        .route(
            "/delete/:id",
            delete(delete_analysis).with_state(Arc::clone(&app_assembler.analysis.deleter)),
        )
        .route(
            "/find/:id",
            get(find_analysis_by_id).with_state(Arc::clone(&app_assembler.analysis.by_id_finder)),
        )
        .route(
            "/find",
            post(find_analysis_by_criteria)
                .with_state(Arc::clone(&app_assembler.analysis.by_criteria_finder)),
        )
        .route(
            "/find/header/:id",
            get(find_analysis_header_by_id)
                .with_state(Arc::clone(&app_assembler.analysis.by_id_header_finder)),
        )
        .route(
            "/find/header",
            post(find_analysis_headers_by_criteria).with_state(Arc::clone(
                &app_assembler.analysis.by_criteria_header_finder,
            )),
        );

    let file_routes = Router::new()
        .route(
            "/delete/:id",
            delete(delete_file).with_state(Arc::clone(&app_assembler.file.deleter)),
        )
        .route(
            "/find/:id",
            get(find_file_by_id).with_state(Arc::clone(&app_assembler.file.by_id_finder)),
        )
        .route(
            "/find",
            post(find_file_by_criteria)
                .with_state(Arc::clone(&app_assembler.file.by_criteria_finder)),
        );

    let lap_routes = Router::new()
        .route(
            "/delete/:id",
            delete(delete_lap).with_state(Arc::clone(&app_assembler.lap.deleter)),
        )
        .route(
            "/find/:id",
            get(find_lap_by_id).with_state(Arc::clone(&app_assembler.lap.by_id_finder)),
        )
        .route(
            "/find",
            post(find_lap_by_criteria)
                .with_state(Arc::clone(&app_assembler.lap.by_criteria_finder)),
        )
        .route(
            "/find/header/:id",
            get(find_lap_header_by_id)
                .with_state(Arc::clone(&app_assembler.lap.by_id_header_finder)),
        )
        .route(
            "/find/header",
            post(find_lap_headers_by_criteria)
                .with_state(Arc::clone(&app_assembler.lap.by_criteria_header_finder)),
        );

    let ibt_extractor_routes = Router::new().route(
        "/upload/:name",
        post(upload).with_state(UploadIbtState {
            ibt_parser: Arc::clone(&app_assembler.ibt.parser),
            file_finder: Arc::clone(&app_assembler.file.by_id_finder),
        }),
    );

    Router::new()
        .nest("/analysis", analysis_routes)
        .nest("/file", file_routes)
        .nest("/lap", lap_routes)
        .nest("/ibt_extractor", ibt_extractor_routes)
        .layer(DefaultBodyLimit::disable())
        .layer(CorsLayer::permissive())
}
