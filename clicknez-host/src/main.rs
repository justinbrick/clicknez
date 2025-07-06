pub mod auth;
pub mod workflow;


#[derive(Clone)]
pub struct AppState {
    
}

#[tokio::main]
async fn main() {
    let state = AppState {};
    let router = axum::Router::new()
        .layer(axum::middleware::from_fn(auth::authenticate_request))
        .with_state(state);
}
