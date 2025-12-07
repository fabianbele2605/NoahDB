
use noah_protocol::grpc::noah_service_server::{NoahService, NoahServiceServer};
use noah_protocol::grpc::{
    SetRequest, SetResponse, GetRequest, GetResponse,
    DeleteRequest, DeleteResponse, ListRequest, ListResponse
};
use std::sync::Arc;

// Adapter para NanoDB engine
mod engine_adapter;
mod persistence;
use engine_adapter::NanoEngine;
use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;


#[derive(Debug)]
pub struct NoahServer {
    engine: Arc<NanoEngine>,  // ← Usando motor NanoDB
}

impl NoahServer {
    pub fn new() -> Self {
        Self {
            engine: Arc::new(NanoEngine::new()),
        }
    }
}

// HTTP API Types
#[derive(Serialize, Deserialize)]
struct SetRequestHttp {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    success: bool,
    message: String,
    data: Option<String>,
}

// gRPC Implementation
#[tonic::async_trait]
impl NoahService for NoahServer {
    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        let req = request.into_inner();
        self.engine.set(&req.key, req.value.into_bytes());
        
        Ok(Response::new(SetResponse {
            success: true,
            message: format!("Key '{}' set successfully", req.key),
        }))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let req = request.into_inner();
        
        if let Some(value_bytes) = self.engine.get(&req.key) {
            let value = String::from_utf8_lossy(&value_bytes).to_string();
            Ok(Response::new(GetResponse {
                found: true,
                value,
            }))
        } else {
            Ok(Response::new(GetResponse {
                found: false,
                value: String::new(),
            }))
        }
    }

    async fn delete(&self, request: Request<DeleteRequest>) -> Result<Response<DeleteResponse>, Status> {
        let req = request.into_inner();
        
        let existed = self.engine.delete(&req.key);
        if existed {
            Ok(Response::new(DeleteResponse {
                success: true,
                message: format!("Key '{}' deleted successfully", req.key),
            }))
        } else {
            Ok(Response::new(DeleteResponse {
                success: false,
                message: format!("Key '{}' not found", req.key),
            }))
        }
    }

    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        let req = request.into_inner();
        let keys = self.engine.list_keys(&req.prefix);

        Ok(Response::new(ListResponse { keys }))
    }
}

// HTTP Handlers
async fn http_set(
    State(engine): State<Arc<NanoEngine>>,
    Json(payload): Json<SetRequestHttp>,
) -> Json<ApiResponse> {
    engine.set(&payload.key, payload.value.into_bytes());
    Json(ApiResponse {
        success: true,
        message: format!("Key '{}' set successfully", payload.key),
        data: None,
    })
}

async fn http_get(
    State(engine): State<Arc<NanoEngine>>,
    Path(key): Path<String>,
) -> Json<ApiResponse> {
    if let Some(value_bytes) = engine.get(&key) {
        let value = String::from_utf8_lossy(&value_bytes).to_string();
        Json(ApiResponse {
            success: true,
            message: "Key found".to_string(),
            data: Some(value),
        })
    } else {
        Json(ApiResponse {
            success: false,
            message: "Key not found".to_string(),
            data: None,
        })
    }
}

async fn http_delete(
    State(engine): State<Arc<NanoEngine>>,
    Path(key): Path<String>,
) -> Json<ApiResponse> {
    let existed = engine.delete(&key);
    if existed {
        Json(ApiResponse {
            success: true,
            message: format!("Key '{}' deleted successfully", key),
            data: None,
        })
    } else {
        Json(ApiResponse {
            success: false,
            message: format!("Key '{}' not found", key),
            data: None,
        })
    }
}

async fn http_list(
    State(engine): State<Arc<NanoEngine>>,
) -> Json<Vec<String>> {
    let keys = engine.list_keys("");
    Json(keys)
}

// Test endpoint
async fn test_cors() -> &'static str {
    "CORS test successful"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    // 1. Crear PersistenceManager
    let persistence = Arc::new(persistence::PersistenceManager::new("./data"));
    
    // 2. Cargar snapshot
    info!(" Loading snapshot...");
    let snapshot_data = persistence.load_snapshot()?;
    
    // 3. Crear engine y poblar con datos
    let engine = Arc::new(NanoEngine::new());
    for (key, value) in snapshot_data {
        engine.set(&key, value);
    }
    
    // 4. Crear servidor con el engine poblado
    let noah_server = NoahServer {
        engine: engine.clone(),
    };
    
    // 5. Tarea periódica para guardar snapshots cada 60 segundos
    let persistence_clone = persistence.clone();
    let engine_clone = engine.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            info!(" Saving snapshot...");
            let data = engine_clone.get_all_data();
            if let Err(e) = persistence_clone.save_snapshot(data) {
                tracing::error!("Failed to save snapshot: {}", e);
            }
        }
    });

    // HTTP Server
    let app = Router::new()
        .route("/api/set", post(http_set))
        .route("/api/get/:key", get(http_get))
        .route("/api/delete/:key", delete(http_delete))
        .route("/api/list", get(http_list))
        .route("/test", get(test_cors))
        .layer(CorsLayer::permissive())
        .with_state(engine);

    // Start HTTP server
    let http_addr = "127.0.0.1:8080";
    info!(" NoahDB HTTP Server starting on {}", http_addr);
    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(http_addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    // Start gRPC server
    let grpc_addr = "127.0.0.1:50051".parse()?;
    info!(" NoahDB gRPC Server starting on {}", grpc_addr);

    Server::builder()
        .add_service(NoahServiceServer::new(noah_server))
        .serve(grpc_addr)
        .await?;

    Ok(())
}
