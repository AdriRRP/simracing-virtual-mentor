extern crate backend_lib;

use backend_lib::api::infrastructure::app_assembler::AppAssembler;
use backend_lib::api::infrastructure::controller::file::find_file_by_criteria;
use backend_lib::api::infrastructure::controller::upload_ibt::upload_ibt;
use backend_lib::api::infrastructure::controller::upload_ibt::ControllerState as UploadIbtState;
use backend_lib::api::infrastructure::settings::Settings;
use backend_lib::api::infrastructure::subscriber::manager::Manager as SubscriberManager;
use backend_lib::api::infrastructure::subscriber::on_ibt_extracted::validate_file::FileValidator;

use axum::extract::DefaultBodyLimit;
use axum::routing::{delete, get, post};
use axum::Router;
use std::sync::Arc;
use tokio::io;

pub struct AppState {}

#[tokio::main]
async fn main() -> io::Result<()> {
    let settings = Settings::load().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Error loading settings: {e}"),
        )
    })?;

    let app_assembler = AppAssembler::new(&settings);

    SubscriberManager::builder()
        .with_subscriber(Arc::new(
            FileValidator::new(&app_assembler.event_bus, &app_assembler.file.validator).await,
        ))
        .build()
        .run();

    //let analysis_routes = Router::new().route("/:id", get(|| async {}));
    let file_routes = Router::new()
        .route("/delete/:id", delete(|| async {}))
        .route("/find/:id", get(|| async {}))
        .route(
            "/find",
            get(find_file_by_criteria)
                .with_state(Arc::clone(&app_assembler.file.by_criteria_finder)),
        );
    //let ibt_extractor_routes = Router::new().route("/:id", get(|| async {}));
    //let lap_routes = Router::new().route("/:id", get(|| async {}));

    let app = Router::new()
        .nest("/file", file_routes)
        .route(
            "/upload",
            post(upload_ibt).with_state(UploadIbtState {
                ibt_parser: Arc::clone(&app_assembler.ibt.parser),
                file_finder: Arc::clone(&app_assembler.file.by_id_finder),
            }),
        )
        .layer(DefaultBodyLimit::disable());

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", settings.server.host, settings.server.port))
            .await?;

    println!("listening on {}", listener.local_addr().unwrap());

    //tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await?;

    Ok(())
}
