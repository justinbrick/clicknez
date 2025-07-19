use tower_http::trace::TraceLayer;
use tracing_subscriber::filter;

pub mod auth;
pub mod workflow;


#[derive(Clone)]
pub struct AppState {
    
}

#[tokio::main]
async fn main() {
    
    tracing_subscriber::fmt()
        .with_max_level(filter::LevelFilter::INFO)
        .init();

    let state = AppState {};
    let router = axum::Router::new()
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router)
        .await
        .unwrap();
}
