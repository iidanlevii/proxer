use std::sync::{ Arc, Mutex };

use axum::{ body::Body, extract::State, http::Request };

use crate::{ AppState, cache::get_apis };

pub async fn proxer(State(state): State<Arc<Mutex<AppState>>>, request: Request<Body>) -> String {
    let mut state = state.lock().unwrap();
    let uri = request.uri();
    let apis = get_apis(&mut state.redis_con).unwrap();
    tracing::info!("apis: {:?}", apis);

    format!("request {:?}", request)
}
