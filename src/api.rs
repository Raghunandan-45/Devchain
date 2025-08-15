use axum::{routing::get, Router, response::Json};
use std::sync::{Arc, Mutex};
use tower_http::cors::{CorsLayer, Any};
use crate::app_state::AppState;

// This function will be our API endpoint handler.
// It safely locks the shared state to read the data.
async fn get_chain_data(state: axum::extract::State<Arc<Mutex<AppState>>>) -> Json<serde_json::Value> {
    let state_lock = state.lock().unwrap();
    Json(serde_json::json!({
        "chain": state_lock.blockchain.chain
    }))
}

pub async fn start_api_server(app_state: Arc<Mutex<AppState>>) {
    let app = Router::new()
        .route("/api/chain", get(get_chain_data))
        .with_state(app_state)
        // This CORS layer allows the browser to make requests
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("[API] Visualizer API server listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}