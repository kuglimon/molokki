use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Krangle {
    id: u32,
    address: String,
    corrupted: bool,
    enabled: bool,
}

#[derive(Clone)]
struct AppState {
    krangles: Arc<DashMap<u32, Krangle>>,
}

#[tokio::main]
async fn main() {
    // Set up logging
    tracing_subscriber::fmt::init();

    // Create initial state with some dummy krangles
    let state = AppState {
        krangles: Arc::new(DashMap::new()),
    };

    state.krangles.insert(
        1,
        Krangle {
            id: 1,
            address: "http://example.com/krangle/1".to_string(),
            corrupted: false,
            enabled: true,
        },
    );

    state.krangles.insert(
        2,
        Krangle {
            id: 2,
            address: "http://example.com/krangle/2".to_string(),
            corrupted: false,
            enabled: false,
        },
    );

    // Define routes
    let app = Router::new()
        .route("/krangles", get(get_all_krangles))
        .route("/krangles/:id/corrupt", post(corrupt_krangle))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum GetKranglesResponse {
    ValidKrangles(Krangle),
    ApiError { error: String },
}

async fn get_all_krangles(State(state): State<AppState>) -> impl IntoResponse {
    let krangles: Vec<Krangle> = state
        .krangles
        .iter()
        .map(|entry| entry.value().clone())
        .collect();
    Json(krangles)
}

async fn corrupt_krangle(Path(id): Path<u32>, State(state): State<AppState>) -> impl IntoResponse {
    match state.krangles.get_mut(&id) {
        Some(mut krangle) => {
            if krangle.corrupted {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(GetKranglesResponse::ApiError {
                        error: "Krangle is already corrupted".to_string(),
                    }),
                );
            }
            krangle.corrupted = true;
            info!(krangle_id = id, "Krangle corrupted");
            (
                StatusCode::OK,
                Json(GetKranglesResponse::ValidKrangles(krangle.clone())),
            )
        }
        None => {
            error!(krangle_id = id, "Krangle not found");
            (
                StatusCode::NOT_FOUND,
                Json(GetKranglesResponse::ApiError {
                    error: "Krangle not found".to_string(),
                }),
            )
        }
    }
}
