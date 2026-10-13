use axum::{Router, routing::get};
use rust_web_store_rahmananin::model::templates::index_handler;
use tower_http::services::ServeDir;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
pub async fn main() {
    let subscrider = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .compact()
        .without_time()
        .finish();

    tracing::subscriber::set_global_default(subscrider).expect("");

    let app = Router::new()
        .route("/", get(index_handler))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();

    tracing::info!("Server has open on http://0.0.0.0:3030");
    axum::serve(listener, app).await.unwrap();
}
