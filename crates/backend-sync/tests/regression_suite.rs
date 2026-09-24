use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use narratics_backend_sync::{
    app_state::{AppState, CreateProjectReq, UpdateProjectMetaReq},
    create_router,
};
use serde_json::json;
use tempfile::tempdir;
use tower::ServiceExt;

#[tokio::test]
async fn test_regression_health_and_landing() {
    let dir = tempdir().unwrap();
    let projects_dir = dir.path().join("projects");
    let initial_file = projects_dir.join("test.narr");
    let state = AppState::new(projects_dir, initial_file).unwrap();
    let app = create_router(state);

    // Health
    let req = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // Landing
    let req = Request::builder().uri("/").body(Body::empty()).unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // Studio
    let req = Request::builder().uri("/app").body(Body::empty()).unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // Releases
    let req = Request::builder()
        .uri("/api/releases")
        .body(Body::empty())
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_regression_project_lifecycle() {
    let dir = tempdir().unwrap();
    let projects_dir = dir.path().join("projects");
    let initial_file = projects_dir.join("test.narr");
    let state = AppState::new(projects_dir, initial_file).unwrap();
    let app = create_router(state);

    // 1. Create project with three_act template
    let new_proj = CreateProjectReq {
        title: "3막 구조 테스트 소설".to_string(),
        author: Some("최작가".to_string()),
        genre: Some("미스터리".to_string()),
        synopsis: Some("사건의 발단과 전개".to_string()),
        target_words: Some(70000),
        template: Some("three_act".to_string()),
    };
    let req = Request::builder()
        .method("POST")
        .uri("/api/projects/new")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&new_proj).unwrap()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .unwrap();
    let val: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let proj_id = val["project_id"].as_str().unwrap().to_string();
    assert!(proj_id.contains("3막구조테스트소설"));

    // 2. Switch project
    let switch_payload = json!({ "id": proj_id });
    let req = Request::builder()
        .method("POST")
        .uri("/api/projects/switch")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(switch_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // 3. Verify state has 3 acts from template
    let req = Request::builder()
        .uri("/api/state")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .unwrap();
    let state_val: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(state_val["title"], "3막 구조 테스트 소설");
    assert_eq!(state_val["author"], "최작가");
    assert!(state_val["binder"].as_array().unwrap().len() >= 4);

    // 4. Update metadata
    let update_req = UpdateProjectMetaReq {
        title: Some("수정된 소설 제목".to_string()),
        author: Some("최용철 작가".to_string()),
        genre: Some("스릴러".to_string()),
        synopsis: Some("수정된 시놉시스".to_string()),
        target_words: Some(85000),
    };
    let req = Request::builder()
        .method("POST")
        .uri("/api/projects/meta")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&update_req).unwrap()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // 5. Delete project
    let del_payload = json!({ "id": proj_id });
    let req = Request::builder()
        .method("POST")
        .uri("/api/projects/delete")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(del_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_regression_binder_and_scene_crud() {
    let dir = tempdir().unwrap();
    let projects_dir = dir.path().join("projects");
    let initial_file = projects_dir.join("test_crud.narr");
    let state = AppState::new(projects_dir, initial_file).unwrap();
    let app = create_router(state);

    // 1. Move node (add chapter)
    let move_payload = json!({
        "child": "chap_reg_1",
        "parent": "root",
        "rank": "10",
        "title": "제1장: 회귀의 서막"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/binder/move")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(move_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // 2. Add scene under chapter
    let move_scene_payload = json!({
        "child": "scene_reg_1",
        "parent": "chap_reg_1",
        "rank": "10",
        "title": "씬 1: 새벽의 다짐"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/binder/move")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(move_scene_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // 3. Save scene text
    let scene_text = "이곳은 회귀 테스트 원고입니다. 단 한 자의 유실도 허용하지 않습니다.";
    let scene_save_payload = json!({
        "text": scene_text
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/scenes/scene_reg_1")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(scene_save_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // 4. Retrieve scene text and verify word count
    let req = Request::builder()
        .uri("/api/scenes/scene_reg_1")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .unwrap();
    let sc_val: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(sc_val["text"], scene_text);
    assert_eq!(sc_val["word_count"], scene_text.chars().count());

    // 5. Update status
    let status_payload = json!({ "status": "퇴고완료" });
    let req = Request::builder()
        .method("POST")
        .uri("/api/scenes/scene_reg_1/status")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(status_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // 6. Rename node
    let rename_payload = json!({ "id": "chap_reg_1", "title": "제1장: 변경된 제목" });
    let req = Request::builder()
        .method("POST")
        .uri("/api/binder/rename")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(rename_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // 7. Delete node
    let delete_payload = json!({ "id": "scene_reg_1" });
    let req = Request::builder()
        .method("POST")
        .uri("/api/binder/delete")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(delete_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_regression_lore_and_snapshots() {
    let dir = tempdir().unwrap();
    let projects_dir = dir.path().join("projects");
    let initial_file = projects_dir.join("test_lore.narr");
    let state = AppState::new(projects_dir, initial_file).unwrap();
    let app = create_router(state);

    // 1. Save Lore
    let lore_payload = json!({
        "id": "char_hero",
        "category": "인물",
        "name": "에이든 카엘룸",
        "aliases": ["달빛의 기사", "방랑자"],
        "content": "과거를 기억하는 유일한 인물."
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/lore")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(lore_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // Verify in State
    let req = Request::builder()
        .uri("/api/state")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .unwrap();
    let state_val: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let lores = state_val["lore"].as_array().unwrap();
    assert!(lores.iter().any(|l| l["name"] == "에이든 카엘룸"));

    // 2. Save Snapshot
    let snap_payload = json!({
        "id": "snap_test_01",
        "scene_id": "scene_any",
        "label": "1차 퇴고본",
        "content_hash": "sha256_mock",
        "content_diff": "스냅샷 백업 텍스트",
        "created_at": 1727140000
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/snapshots")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(snap_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // 3. Delete Lore
    let del_lore = json!({ "id": "char_hero" });
    let req = Request::builder()
        .method("POST")
        .uri("/api/lore/delete")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(del_lore.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_regression_multiformat_export() {
    let dir = tempdir().unwrap();
    let projects_dir = dir.path().join("projects");
    let initial_file = projects_dir.join("test_export.narr");
    let state = AppState::new(projects_dir, initial_file).unwrap();
    let app = create_router(state);

    // Export Markdown
    let req = Request::builder()
        .uri("/api/export/markdown")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers().get(header::CONTENT_TYPE).unwrap(),
        "text/markdown; charset=utf-8"
    );

    // Export Text
    let req = Request::builder()
        .uri("/api/export/text")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers().get(header::CONTENT_TYPE).unwrap(),
        "text/plain; charset=utf-8"
    );

    // Export Typst Source
    let req = Request::builder()
        .uri("/api/export/typst")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // Export PDF (Typst Compile)
    let req = Request::builder()
        .uri("/api/export/pdf")
        .body(Body::empty())
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers().get(header::CONTENT_TYPE).unwrap(),
        "application/pdf"
    );
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .unwrap();
    assert!(bytes.len() > 1000); // Real compiled PDF bytes
    assert_eq!(&bytes[0..4], b"%PDF"); // Valid PDF magic number
}
