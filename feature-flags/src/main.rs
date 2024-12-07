use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/gold-plating", get(flag))
        .fallback(handle_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("started on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn flag() -> (StatusCode, Json<Flag>) {
    let body = Flag {
        flag: rand::random(),
    };
    tracing::info!("returning \"{}\"", body.flag);

    // 200 OK
    (StatusCode::OK, Json(body))
}

#[derive(Serialize)]
struct Flag {
    flag: bool,
}

async fn handle_404() -> StatusCode {
    StatusCode::NOT_FOUND
}
