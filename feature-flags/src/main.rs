use axum::{extract::Path, http::StatusCode, routing::get, Json, Router};
use serde::Serialize;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/where-do-i-put/:work_item", get(determine_database))
        .fallback(handle_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("started on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct Flag {
    flag: bool,
}

async fn determine_database(Path(work_item): Path<i32>) -> (StatusCode, Json<Flag>) {
    let body = Flag {
        flag: work_item % 2 == 0,
    };

    tracing::info!("{}: {}", work_item, body.flag);

    (StatusCode::OK, Json(body))
}

async fn handle_404() -> StatusCode {
    StatusCode::NOT_FOUND
}
