use axum::body::Body;
use axum::http::header;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use tokio::signal;
include!(concat!(env!("OUT_DIR"), "/goats.rs"));

struct Image {
    data: &'static [u8],
    mime: &'static str,
}

impl IntoResponse for &Image {
    fn into_response(self) -> Response {
        ([(header::CONTENT_TYPE, self.mime)], Body::from(self.data)).into_response()
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_goat))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Serving swiss goats from http://0.0.0.0:3000");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn health() -> &'static str {
    "OK"
}

async fn get_goat() -> Response {
    let goat = rand::random::<usize>() % GOATS.len();

    GOATS[goat].into_response()
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
