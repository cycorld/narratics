mod app_state;
mod web_ui;

use app_state::{AppState, ProjectStateSnapshot, SceneDto, WsMessage};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use futures::{SinkExt, StreamExt};
use narratics_engine_core::{
    container::{LoreRecord, NarrContainer, SnapshotRecord},
    text_engine::SceneEngine,
};
use serde::Deserialize;
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "narratics_backend_sync=info,tower_http=info".into()),
        )
        .init();

    let project_path = std::env::var("NARRATICS_CONTAINER_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("workspace.narr"));

    info!("Opening Narratics container at: {:?}", project_path);
    let container = NarrContainer::open(&project_path)?;
    let app_state = AppState::new(container)?;

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/", get(index_handler))
        .route("/api/state", get(state_handler))
        .route("/api/scenes/{id}", get(get_scene_handler).post(post_scene_handler))
        .route("/api/scenes/{id}/status", post(scene_status_handler))
        .route("/api/scenes/{id}/restore", post(restore_scene_handler))
        .route("/api/binder/move", post(move_node_handler))
        .route("/api/lore", post(save_lore_handler))
        .route("/api/snapshots", post(save_snapshot_handler))
        .route("/ws", get(ws_handler))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3901);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Narratics Studio Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": "0.1.0",
        "engine": "Kleppmann Tree Move CRDT + Yrs UTF-16",
        "container": "SQLite WAL (.narr)"
    }))
}

async fn index_handler() -> Html<&'static str> {
    Html(web_ui::INDEX_HTML)
}

async fn state_handler(
    State(state): State<AppState>,
) -> Result<Json<ProjectStateSnapshot>, (StatusCode, String)> {
    match state.get_snapshot().await {
        Ok(snapshot) => Ok(Json(snapshot)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to build snapshot: {e}"),
        )),
    }
}

async fn get_scene_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<SceneDto>, (StatusCode, String)> {
    let scenes = state.scenes.read().await;
    let titles = state.scene_titles.read().await;
    let statuses = state.scene_statuses.read().await;

    if let Some(engine) = scenes.get(&id) {
        let title = titles.get(&id).cloned().unwrap_or_else(|| id.clone());
        let text = engine.get_text();
        let word_count = engine.char_count();
        let status = statuses.get(&id).cloned().unwrap_or_else(|| "초고".to_string());

        Ok(Json(SceneDto {
            id,
            title,
            text,
            word_count,
            status,
            updated_at: 0,
        }))
    } else {
        Err((StatusCode::NOT_FOUND, "Scene not found".to_string()))
    }
}

#[derive(Deserialize)]
struct SceneUpdatePayload {
    text: String,
}

async fn post_scene_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<SceneUpdatePayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let word_count = payload.text.chars().count();

    // Update memory engine
    {
        let mut scenes = state.scenes.write().await;
        if let Some(engine) = scenes.get_mut(&id) {
            let current_len = engine.char_count() as u32;
            if current_len > 0 {
                engine.delete(0, current_len);
            }
            engine.insert(0, &payload.text);
        } else {
            let engine = SceneEngine::new(&id);
            engine.insert(0, &payload.text);
            scenes.insert(id.clone(), engine);
        }
    }

    // Persist to SQLite WAL
    {
        let scenes = state.scenes.read().await;
        if let Some(engine) = scenes.get(&id) {
            if let Ok(diff) = engine.encode_diff(None) {
                let conn = state.container.lock().await;
                if let Err(e) = conn.save_scene(&id, &diff, &payload.text, word_count) {
                    error!("Failed to save scene in SQLite: {:?}", e);
                }
            }
        }
    }

    // Broadcast change
    let _ = state.tx.send(WsMessage::TextUpdate {
        scene_id: id,
        text: payload.text,
        word_count,
        client_id: "rest_api".to_string(),
    });

    Ok(Json(serde_json::json!({ "success": true })))
}

#[derive(Deserialize)]
struct MoveNodePayload {
    child: String,
    parent: String,
    rank: String,
    title: String,
}

async fn move_node_handler(
    State(state): State<AppState>,
    Json(payload): Json<MoveNodePayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let op = {
        let mut tree = state.tree.write().await;
        tree.move_node(
            &payload.child,
            &payload.parent,
            &payload.rank,
            &payload.title,
        )
    };

    // Update titles map
    {
        let mut titles = state.scene_titles.write().await;
        titles.insert(payload.child.clone(), payload.title.clone());
    }

    // Save op to SQLite container
    {
        let mut conn = state.container.lock().await;
        if let Err(e) = conn.save_tree_ops(&[op.clone()]) {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to persist move op: {e}"),
            ));
        }
    }

    // Broadcast op
    let _ = state.tx.send(WsMessage::TreeMove {
        op,
        client_id: "rest_api".to_string(),
    });

    Ok(Json(serde_json::json!({ "success": true })))
}

async fn save_lore_handler(
    State(state): State<AppState>,
    Json(lore): Json<LoreRecord>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    {
        let conn = state.container.lock().await;
        if let Err(e) = conn.save_lore(&lore) {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to save lore: {e}"),
            ));
        }
    }

    let _ = state.tx.send(WsMessage::LoreUpdate {
        lore,
        client_id: "rest_api".to_string(),
    });

    Ok(Json(serde_json::json!({ "success": true })))
}

#[derive(Deserialize)]
struct StatusPayload {
    status: String,
}

async fn scene_status_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<StatusPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    {
        let mut statuses = state.scene_statuses.write().await;
        statuses.insert(id.clone(), payload.status.clone());
    }

    let _ = state.tx.send(WsMessage::SceneStatusUpdated {
        scene_id: id,
        status: payload.status,
    });

    Ok(Json(serde_json::json!({ "success": true })))
}

async fn save_snapshot_handler(
    State(state): State<AppState>,
    Json(snapshot): Json<SnapshotRecord>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    {
        let conn = state.container.lock().await;
        if let Err(e) = conn.save_snapshot(&snapshot) {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to save snapshot: {e}"),
            ));
        }
    }

    let _ = state.tx.send(WsMessage::SnapshotCreated {
        snapshot,
        client_id: "rest_api".to_string(),
    });

    Ok(Json(serde_json::json!({ "success": true })))
}

#[derive(Deserialize)]
struct RestorePayload {
    #[serde(alias = "content")]
    text: String,
}

async fn restore_scene_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<RestorePayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let word_count = payload.text.chars().count();

    // Update memory engine
    {
        let mut scenes = state.scenes.write().await;
        if let Some(engine) = scenes.get_mut(&id) {
            let current_len = engine.char_count() as u32;
            if current_len > 0 {
                engine.delete(0, current_len);
            }
            engine.insert(0, &payload.text);
        } else {
            let engine = SceneEngine::new(&id);
            engine.insert(0, &payload.text);
            scenes.insert(id.clone(), engine);
        }
    }

    // Persist to SQLite WAL
    {
        let scenes = state.scenes.read().await;
        if let Some(engine) = scenes.get(&id) {
            if let Ok(diff) = engine.encode_diff(None) {
                let conn = state.container.lock().await;
                let _ = conn.save_scene(&id, &diff, &payload.text, word_count);
            }
        }
    }

    // Broadcast change
    let _ = state.tx.send(WsMessage::TextUpdate {
        scene_id: id,
        text: payload.text,
        word_count,
        client_id: "restore".to_string(),
    });

    Ok(Json(serde_json::json!({ "success": true })))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    // Send initial state
    if let Ok(snapshot) = state.get_snapshot().await {
        let init_msg = WsMessage::Init { state: snapshot };
        if let Ok(json_str) = serde_json::to_string(&init_msg) {
            let _ = sender.send(Message::Text(json_str.into())).await;
        }
    }

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(json_str) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json_str.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    let state_clone = state.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                        match ws_msg {
                            WsMessage::TextUpdate {
                                scene_id,
                                text,
                                word_count,
                                client_id,
                            } => {
                                // Apply update
                                {
                                    let mut scenes = state_clone.scenes.write().await;
                                    if let Some(engine) = scenes.get_mut(&scene_id) {
                                        let current_len = engine.char_count() as u32;
                                        if current_len > 0 {
                                            engine.delete(0, current_len);
                                        }
                                        engine.insert(0, &text);
                                    } else {
                                        let engine = SceneEngine::new(&scene_id);
                                        engine.insert(0, &text);
                                        scenes.insert(scene_id.clone(), engine);
                                    }
                                }

                                // Persist
                                {
                                    let scenes = state_clone.scenes.read().await;
                                    if let Some(engine) = scenes.get(&scene_id) {
                                        if let Ok(diff) = engine.encode_diff(None) {
                                            let conn = state_clone.container.lock().await;
                                            let _ = conn.save_scene(
                                                &scene_id,
                                                &diff,
                                                &text,
                                                word_count,
                                            );
                                        }
                                    }
                                }

                                // Broadcast to other clients
                                let _ = state_clone.tx.send(WsMessage::TextUpdate {
                                    scene_id,
                                    text,
                                    word_count,
                                    client_id,
                                });
                            }
                            WsMessage::TreeMove { op, client_id } => {
                                {
                                    let mut tree = state_clone.tree.write().await;
                                    tree.integrate(op.clone());
                                }
                                {
                                    let mut conn = state_clone.container.lock().await;
                                    let _ = conn.save_tree_ops(&[op.clone()]);
                                }
                                let _ = state_clone.tx.send(WsMessage::TreeMove {
                                    op,
                                    client_id,
                                });
                            }
                            WsMessage::LoreUpdate { lore, client_id } => {
                                {
                                    let conn = state_clone.container.lock().await;
                                    let _ = conn.save_lore(&lore);
                                }
                                let _ = state_clone.tx.send(WsMessage::LoreUpdate {
                                    lore,
                                    client_id,
                                });
                            }
                            WsMessage::Ping => {
                                let _ = state_clone.tx.send(WsMessage::Pong);
                            }
                            _ => {}
                        }
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}
