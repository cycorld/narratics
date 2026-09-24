pub mod app_state;
pub mod landing_ui;
pub mod web_ui;

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
    container::{LoreRecord, SnapshotRecord},
    text_engine::SceneEngine,
};
use serde::Deserialize;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::error;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/", get(landing_handler))
        .route("/app", get(index_handler))
        .route("/studio", get(index_handler))
        .route("/api/releases", get(api_releases_handler))
        .route("/releases/SHA256SUMS", get(sha256sums_handler))
        .route(
            "/releases/{platform}/{filename}",
            get(download_release_handler),
        )
        .route("/docs/{filename}", get(docs_handler))
        .route("/api/state", get(state_handler))
        .route("/api/projects", get(list_projects_handler))
        .route("/api/projects/new", post(create_project_handler))
        .route("/api/projects/switch", post(switch_project_handler))
        .route("/api/projects/delete", post(delete_project_handler))
        .route("/api/projects/meta", post(update_project_meta_handler))
        .route(
            "/api/scenes/{id}",
            get(get_scene_handler).post(post_scene_handler),
        )
        .route("/api/scenes/{id}/status", post(scene_status_handler))
        .route("/api/scenes/{id}/restore", post(restore_scene_handler))
        .route("/api/binder/move", post(move_node_handler))
        .route("/api/binder/rename", post(rename_node_handler))
        .route("/api/binder/delete", post(delete_node_handler))
        .route("/api/binder/reorder", post(reorder_node_handler))
        .route("/api/lore", post(save_lore_handler))
        .route("/api/lore/delete", post(delete_lore_handler))
        .route("/api/snapshots", post(save_snapshot_handler))
        .route("/api/export/{format}", get(export_manuscript_handler))
        .route("/ws", get(ws_handler))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": "0.1.0",
        "engine": "Kleppmann Tree Move CRDT + Yrs UTF-16",
        "container": "SQLite WAL (.narr)"
    }))
}

async fn landing_handler() -> Html<&'static str> {
    Html(landing_ui::LANDING_HTML)
}

async fn index_handler() -> Html<&'static str> {
    Html(web_ui::INDEX_HTML)
}

async fn api_releases_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "version": "0.1.0",
        "date": "2026-09-24",
        "license": "PolyForm-Noncommercial-1.0.0",
        "platforms": {
            "windows": {
                "filename": "Narratics-Setup-0.1.0.exe",
                "format": ".exe",
                "arch": ["x64", "arm64"],
                "download_url": "/releases/windows/Narratics-Setup-0.1.0.exe",
                "sha256": "dbcd8d39a2af2b369e15d82bac01c6f91858787cb377581b1a5d42c2ec466dac",
                "requirements": "Windows 10 / 11 (64-bit)"
            },
            "macos": {
                "filename": "Narratics-0.1.0-universal.dmg",
                "format": ".dmg",
                "arch": ["universal", "aarch64", "x86_64"],
                "download_url": "/releases/macos/Narratics-0.1.0-universal.dmg",
                "sha256": "91227c87305a0265f755a2a3ef7f366e11c22fc6d3dd01a9b2e0d8ff910dad72",
                "requirements": "macOS 12.0 Monterey or later"
            },
            "ios": {
                "filename": "Narratics-v0.1.0.ipa",
                "format": ".ipa",
                "arch": ["arm64"],
                "download_url": "/releases/ios/Narratics-v0.1.0.ipa",
                "sha256": "1c9f2357b441e653533b5d50d0b119a198921a670db9d52cbda9e3ce2a5d0dc2",
                "requirements": "iOS 16.0+ / iPadOS 16.0+"
            },
            "android": {
                "filename": "Narratics-v0.1.0-release.apk",
                "format": ".apk",
                "arch": ["arm64-v8a", "x86_64"],
                "download_url": "/releases/android/Narratics-v0.1.0-release.apk",
                "sha256": "f824c93f102450499d598cdcc7219f23bec16e32e55c8e81c94361146255d52f",
                "requirements": "Android 10.0 or later"
            },
            "linux": {
                "filename": "Narratics-0.1.0-x86_64.AppImage",
                "format": ".AppImage",
                "arch": ["x86_64"],
                "download_url": "/releases/linux/Narratics-0.1.0-x86_64.AppImage",
                "sha256": "8b58eacaf9ea3fb756d7a3cf43d127646461c6870bb122fe7f653bbd1871c857",
                "requirements": "glibc 2.31+"
            }
        },
        "checksums_url": "/releases/SHA256SUMS"
    }))
}

async fn sha256sums_handler() -> Result<impl IntoResponse, (StatusCode, String)> {
    match tokio::fs::read_to_string("releases/SHA256SUMS").await {
        Ok(content) => {
            let mut headers = axum::http::HeaderMap::new();
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_static("text/plain; charset=utf-8"),
            );
            Ok((headers, content))
        }
        Err(e) => Err((StatusCode::NOT_FOUND, format!("SHA256SUMS not found: {e}"))),
    }
}

async fn download_release_handler(
    Path((platform, filename)): Path<(String, String)>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if platform.contains("..")
        || filename.contains("..")
        || platform.contains('/')
        || filename.contains('/')
    {
        return Err((StatusCode::BAD_REQUEST, "Invalid path segment".into()));
    }
    let target = std::path::Path::new("releases")
        .join(&platform)
        .join(&filename);
    if !target.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Artifact not found: {platform}/{filename}"),
        ));
    }
    match tokio::fs::read(&target).await {
        Ok(bytes) => {
            let content_type = match filename.split('.').next_back() {
                Some("exe") => "application/vnd.microsoft.portable-executable",
                Some("dmg") => "application/x-apple-diskimage",
                Some("ipa") => "application/octet-stream",
                Some("apk") => "application/vnd.android.package-archive",
                Some("AppImage") => "application/x-executable",
                _ => "application/octet-stream",
            };
            let mut headers = axum::http::HeaderMap::new();
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_static(content_type),
            );
            let disp = format!("attachment; filename=\"{filename}\"");
            if let Ok(val) = axum::http::HeaderValue::from_str(&disp) {
                headers.insert(axum::http::header::CONTENT_DISPOSITION, val);
            }
            Ok((headers, bytes))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Read error: {e}"),
        )),
    }
}

async fn docs_handler(
    Path(filename): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if filename.contains("..") || filename.contains('/') {
        return Err((StatusCode::BAD_REQUEST, "Invalid file name".into()));
    }
    let target = std::path::Path::new("docs").join(&filename);
    if !target.exists() {
        return Err((StatusCode::NOT_FOUND, "Document file not found".into()));
    }
    match tokio::fs::read(&target).await {
        Ok(bytes) => {
            let mut headers = axum::http::HeaderMap::new();
            if filename.ends_with(".png") {
                headers.insert(
                    axum::http::header::CONTENT_TYPE,
                    axum::http::HeaderValue::from_static("image/png"),
                );
            }
            Ok((headers, bytes))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Read error: {e}"),
        )),
    }
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
        let status = statuses
            .get(&id)
            .cloned()
            .unwrap_or_else(|| "초고".to_string());

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
        if let Err(e) = conn.save_tree_ops(std::slice::from_ref(&op)) {
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

#[derive(Deserialize)]
struct SaveLoreReq {
    pub id: String,
    pub category: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub content: String,
    pub updated_at: Option<i64>,
}

async fn save_lore_handler(
    State(state): State<AppState>,
    Json(payload): Json<SaveLoreReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let lore = LoreRecord {
        id: payload.id,
        category: payload.category,
        name: payload.name,
        aliases: payload.aliases,
        content: payload.content,
        updated_at: payload.updated_at.unwrap_or(now),
    };

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

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
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
                                            let _ = conn
                                                .save_scene(&scene_id, &diff, &text, word_count);
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
                                    let _ = conn.save_tree_ops(std::slice::from_ref(&op));
                                }
                                let _ = state_clone.tx.send(WsMessage::TreeMove { op, client_id });
                            }
                            WsMessage::LoreUpdate { lore, client_id } => {
                                {
                                    let conn = state_clone.container.lock().await;
                                    let _ = conn.save_lore(&lore);
                                }
                                let _ = state_clone
                                    .tx
                                    .send(WsMessage::LoreUpdate { lore, client_id });
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

#[derive(Deserialize)]
struct SwitchProjectPayload {
    id: String,
}

#[derive(Deserialize)]
struct DeleteProjectPayload {
    id: String,
}

#[derive(Deserialize)]
struct RenameNodePayload {
    id: String,
    title: String,
}

#[derive(Deserialize)]
struct DeleteNodePayload {
    id: String,
}

#[derive(Deserialize)]
struct ReorderNodePayload {
    id: String,
    direction: String,
}

#[derive(Deserialize)]
struct DeleteLorePayload {
    id: String,
}

async fn list_projects_handler(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match state.list_projects().await {
        Ok(list) => {
            let active_id = state.active_project_id.read().await.clone();
            Ok(Json(serde_json::json!({
                "active_id": active_id,
                "projects": list
            })))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn create_project_handler(
    State(state): State<AppState>,
    Json(payload): Json<app_state::CreateProjectReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match state.create_project(payload).await {
        Ok(id) => Ok(Json(
            serde_json::json!({ "status": "ok", "project_id": id }),
        )),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn switch_project_handler(
    State(state): State<AppState>,
    Json(payload): Json<SwitchProjectPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match state.switch_project(&payload.id).await {
        Ok(_) => Ok(Json(
            serde_json::json!({ "status": "ok", "project_id": payload.id }),
        )),
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

async fn delete_project_handler(
    State(state): State<AppState>,
    Json(payload): Json<DeleteProjectPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match state.delete_project(&payload.id).await {
        Ok(_) => Ok(Json(serde_json::json!({ "status": "ok" }))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn update_project_meta_handler(
    State(state): State<AppState>,
    Json(payload): Json<app_state::UpdateProjectMetaReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match state.update_project_meta(payload).await {
        Ok(_) => Ok(Json(serde_json::json!({ "status": "ok" }))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn rename_node_handler(
    State(state): State<AppState>,
    Json(payload): Json<RenameNodePayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match state.rename_binder_node(&payload.id, &payload.title).await {
        Ok(_) => Ok(Json(serde_json::json!({ "status": "ok" }))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn delete_node_handler(
    State(state): State<AppState>,
    Json(payload): Json<DeleteNodePayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match state.delete_binder_node(&payload.id).await {
        Ok(_) => Ok(Json(serde_json::json!({ "status": "ok" }))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn reorder_node_handler(
    State(state): State<AppState>,
    Json(payload): Json<ReorderNodePayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match state
        .reorder_binder_node(&payload.id, &payload.direction)
        .await
    {
        Ok(_) => Ok(Json(serde_json::json!({ "status": "ok" }))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn delete_lore_handler(
    State(state): State<AppState>,
    Json(payload): Json<DeleteLorePayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match state.delete_lore(&payload.id).await {
        Ok(_) => Ok(Json(serde_json::json!({ "status": "ok" }))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn export_manuscript_handler(
    State(state): State<AppState>,
    Path(format): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match state.export_manuscript(&format).await {
        Ok((filename, content_type, bytes)) => {
            let headers = [
                (axum::http::header::CONTENT_TYPE, content_type),
                (
                    axum::http::header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", filename),
                ),
            ];
            Ok((headers, bytes))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
