extern crate backend_lib;

use backend_lib::api::infrastructure::app_assembler::AppAssembler;
use backend_lib::api::infrastructure::controller::file::{
    delete as delete_file, find_by_criteria as find_file_by_criteria, find_by_id as find_file_by_id,
};
use backend_lib::api::infrastructure::controller::ibt_extractor::upload;
use backend_lib::api::infrastructure::controller::ibt_extractor::ControllerState as UploadIbtState;
use backend_lib::api::infrastructure::controller::lap::{
    delete as delete_lap, find_by_criteria as find_lap_by_criteria, find_by_id as find_lap_by_id,
    find_header_by_id as find_lap_header_by_id,
    find_headers_by_criteria as find_lap_headers_by_criteria,
};
use backend_lib::api::infrastructure::settings::Settings;
use backend_lib::api::infrastructure::subscriber::manager::Manager as SubscriberManager;
use backend_lib::api::infrastructure::subscriber::on_ibt_extracted::validate_file::FileValidator;

use axum::extract::DefaultBodyLimit;
use axum::routing::{delete, get, post};
use axum::Router;
use backend_lib::api::infrastructure::subscriber::on_file_deleted::delete_laps::LapDeleter;
use std::sync::Arc;
use tokio::io;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct AppState {}

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
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("backend={}", settings.log_level).into()),
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
        .build()
        .run();

    //let analysis_routes = Router::new().route("/:id", get(|| async {}));
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
            get(find_file_by_criteria)
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
            get(find_lap_by_criteria).with_state(Arc::clone(&app_assembler.lap.by_criteria_finder)),
        )
        .route(
            "/find/header/:id",
            get(find_lap_header_by_id)
                .with_state(Arc::clone(&app_assembler.lap.by_id_header_finder)),
        )
        .route(
            "/find/header",
            get(find_lap_headers_by_criteria)
                .with_state(Arc::clone(&app_assembler.lap.by_criteria_header_finder)),
        );

    let ibt_extractor_routes = Router::new().route(
        "/upload/:name",
        post(upload).with_state(UploadIbtState {
            ibt_parser: Arc::clone(&app_assembler.ibt.parser),
            file_finder: Arc::clone(&app_assembler.file.by_id_finder),
        }),
    );

    let app = Router::new()
        .nest("/file", file_routes)
        .nest("/lap", lap_routes)
        .nest("/ibt_extractor", ibt_extractor_routes)
        .layer(DefaultBodyLimit::disable());

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", settings.server.host, settings.server.port))
            .await?;

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await?;

    Ok(())
}
