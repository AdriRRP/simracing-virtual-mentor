extern crate backend_lib;

use backend_lib::api::infrastructure::app_assembler::AppAssembler;
use backend_lib::api::infrastructure::controller::file::find_file_by_criteria;
use backend_lib::api::infrastructure::controller::upload_ibt::upload_ibt;

use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

pub struct AppState {}

#[tokio::main]
async fn main() {
    let app_assembler = AppAssembler::new("Put here config file");

    let app = Router::new()
        .route(
            "/find",
            get(find_file_by_criteria)
                .with_state(Arc::clone(&app_assembler.file.by_criteria_finder)),
        )
        .route(
            "/upload",
            post(upload_ibt).with_state((
                Arc::clone(&app_assembler.file.creator),
                Arc::clone(&app_assembler.lap.creator),
                Arc::clone(&app_assembler.file.by_id_finder),
            )),
        )
        .layer(DefaultBodyLimit::disable());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    //tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
