use libc::c_char;
use narratics_engine_core::{
    container::{NarrContainer, SceneRecord, SnapshotRecord},
    text_engine::SceneEngine,
    tree_crdt::NodeInfo,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MobileError {
    #[error("Null pointer argument")]
    NullPointer,
    #[error("UTF-8 decoding error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Database container error: {0}")]
    Container(String),
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Engine error: {0}")]
    Engine(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MobileNodeInfo {
    pub id: String,
    pub parent_id: Option<String>,
    pub rank: String,
    pub title: String,
    pub is_folder: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MobileSceneInfo {
    pub id: String,
    pub title: String,
    pub text: String,
    pub word_count: usize,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MobileProjectState {
    pub title: String,
    pub author: String,
    pub path: String,
    pub nodes: Vec<MobileNodeInfo>,
    pub scenes: Vec<MobileSceneInfo>,
    pub lore_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MobileResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> MobileResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }
    }
}

// ==========================================
// Safe Rust High-Level APIs
// ==========================================

pub fn mobile_open_project(path: &str) -> Result<MobileProjectState, MobileError> {
    let container = NarrContainer::open(path)
        .map_err(|e| MobileError::Container(format!("컨테이너 열기 실패: {e}")))?;

    let title = container
        .get_meta("title")
        .map_err(|e| MobileError::Container(format!("메타데이터 조회 실패: {e}")))?
        .unwrap_or_else(|| "제목 없는 서사".into());

    let author = container
        .get_meta("author")
        .map_err(|e| MobileError::Container(format!("메타데이터 조회 실패: {e}")))?
        .unwrap_or_else(|| "작가".into());

    let tree = container
        .load_tree_crdt(201, "root") // 201: Mobile Client Replica ID
        .map_err(|e| MobileError::Container(format!("트리 로드 실패: {e}")))?;

    let (nodes_map, _) = tree.materialize();
    let mut raw_nodes: Vec<NodeInfo> = nodes_map.into_values().collect();
    raw_nodes.sort_by(|a, b| a.rank.cmp(&b.rank));

    let scenes_raw = container
        .list_scenes()
        .map_err(|e| MobileError::Container(format!("씬 목록 로드 실패: {e}")))?;

    let scene_map: HashMap<String, SceneRecord> =
        scenes_raw.into_iter().map(|s| (s.id.clone(), s)).collect();

    let mut mobile_nodes = Vec::new();
    let mut mobile_scenes = Vec::new();

    for node in raw_nodes {
        let is_folder = !scene_map.contains_key(&node.id) && node.id.starts_with("chap_");
        mobile_nodes.push(MobileNodeInfo {
            id: node.id.clone(),
            parent_id: node.parent.clone(),
            rank: node.rank.clone(),
            title: node.title.clone(),
            is_folder,
        });

        if let Some(scene) = scene_map.get(&node.id) {
            mobile_scenes.push(MobileSceneInfo {
                id: scene.id.clone(),
                title: node.title.clone(),
                text: scene.text_cache.clone(),
                word_count: scene.word_count,
                updated_at: scene.updated_at,
            });
        }
    }

    let lore_list = container
        .list_lore()
        .map_err(|e| MobileError::Container(format!("로어 로드 실패: {e}")))?;

    Ok(MobileProjectState {
        title,
        author,
        path: path.to_string(),
        nodes: mobile_nodes,
        scenes: mobile_scenes,
        lore_count: lore_list.len(),
    })
}

pub fn mobile_save_scene(
    path: &str,
    scene_id: &str,
    text: &str,
) -> Result<MobileSceneInfo, MobileError> {
    let container = NarrContainer::open(path)
        .map_err(|e| MobileError::Container(format!("컨테이너 열기 실패: {e}")))?;

    let engine = SceneEngine::new(scene_id);
    engine.insert(0, text);
    let diff = engine
        .encode_diff(None)
        .map_err(|e| MobileError::Engine(format!("인코딩 실패: {e}")))?;
    let char_count = engine.char_count();

    container
        .save_scene(scene_id, &diff, text, char_count)
        .map_err(|e| MobileError::Container(format!("씬 저장 실패: {e}")))?;

    Ok(MobileSceneInfo {
        id: scene_id.to_string(),
        title: scene_id.to_string(),
        text: text.to_string(),
        word_count: char_count,
        updated_at: chrono::Utc::now().timestamp(),
    })
}

pub fn mobile_move_node(
    path: &str,
    child_id: &str,
    parent_id: &str,
    rank: &str,
    title: &str,
) -> Result<Vec<MobileNodeInfo>, MobileError> {
    let mut container = NarrContainer::open(path)
        .map_err(|e| MobileError::Container(format!("컨테이너 열기 실패: {e}")))?;

    let mut tree = container
        .load_tree_crdt(201, "root")
        .map_err(|e| MobileError::Container(format!("트리 로드 실패: {e}")))?;

    let op = tree.move_node(child_id, parent_id, rank, title);
    container
        .save_tree_ops(&[op])
        .map_err(|e| MobileError::Container(format!("연산 저장 실패: {e}")))?;

    let (nodes_map, _) = tree.materialize();
    let mut raw_nodes: Vec<NodeInfo> = nodes_map.into_values().collect();
    raw_nodes.sort_by(|a, b| a.rank.cmp(&b.rank));

    let nodes = raw_nodes
        .into_iter()
        .map(|n| MobileNodeInfo {
            id: n.id,
            parent_id: n.parent,
            rank: n.rank,
            title: n.title,
            is_folder: false,
        })
        .collect();

    Ok(nodes)
}

pub fn mobile_create_snapshot(
    path: &str,
    target_id: &str,
    label: &str,
    content: &str,
) -> Result<String, MobileError> {
    let container = NarrContainer::open(path)
        .map_err(|e| MobileError::Container(format!("컨테이너 열기 실패: {e}")))?;

    let snap_id = format!("snap_mob_{}", chrono::Utc::now().timestamp_millis());
    let words = content.chars().filter(|c| !c.is_whitespace()).count();
    let record = SnapshotRecord {
        id: snap_id.clone(),
        target_id: target_id.to_string(),
        label: label.to_string(),
        content: content.to_string(),
        word_count: words,
        created_at: chrono::Utc::now().timestamp(),
    };

    container
        .save_snapshot(&record)
        .map_err(|e| MobileError::Container(format!("스냅샷 저장 실패: {e}")))?;

    Ok(snap_id)
}

// ==========================================
// C-ABI Export Symbols (for iOS Swift & Android JNI)
// ==========================================

unsafe fn c_str_to_str<'a>(ptr: *const c_char) -> Result<&'a str, MobileError> {
    if ptr.is_null() {
        return Err(MobileError::NullPointer);
    }
    CStr::from_ptr(ptr).to_str().map_err(MobileError::Utf8)
}

fn to_c_json<T: Serialize>(res: MobileResponse<T>) -> *mut c_char {
    let json_str = serde_json::to_string(&res).unwrap_or_else(|_| {
        r#"{"success":false,"data":null,"error":"JSON serialization failure"}"#.to_string()
    });
    CString::new(json_str).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn narr_mobile_open(path: *const c_char) -> *mut c_char {
    let path_str = match c_str_to_str(path) {
        Ok(s) => s,
        Err(e) => return to_c_json::<()>(MobileResponse::err(e.to_string())),
    };

    match mobile_open_project(path_str) {
        Ok(state) => to_c_json(MobileResponse::ok(state)),
        Err(e) => to_c_json::<()>(MobileResponse::err(e.to_string())),
    }
}

#[no_mangle]
pub unsafe extern "C" fn narr_mobile_save_scene(
    path: *const c_char,
    scene_id: *const c_char,
    text: *const c_char,
) -> *mut c_char {
    let path_str = match c_str_to_str(path) {
        Ok(s) => s,
        Err(e) => return to_c_json::<()>(MobileResponse::err(e.to_string())),
    };
    let scene_str = match c_str_to_str(scene_id) {
        Ok(s) => s,
        Err(e) => return to_c_json::<()>(MobileResponse::err(e.to_string())),
    };
    let text_str = match c_str_to_str(text) {
        Ok(s) => s,
        Err(e) => return to_c_json::<()>(MobileResponse::err(e.to_string())),
    };

    match mobile_save_scene(path_str, scene_str, text_str) {
        Ok(info) => to_c_json(MobileResponse::ok(info)),
        Err(e) => to_c_json::<()>(MobileResponse::err(e.to_string())),
    }
}

#[no_mangle]
pub unsafe extern "C" fn narr_mobile_move_node(
    path: *const c_char,
    child: *const c_char,
    parent: *const c_char,
    rank: *const c_char,
    title: *const c_char,
) -> *mut c_char {
    let path_str = match c_str_to_str(path) {
        Ok(s) => s,
        Err(e) => return to_c_json::<()>(MobileResponse::err(e.to_string())),
    };
    let child_str = match c_str_to_str(child) {
        Ok(s) => s,
        Err(e) => return to_c_json::<()>(MobileResponse::err(e.to_string())),
    };
    let parent_str = match c_str_to_str(parent) {
        Ok(s) => s,
        Err(e) => return to_c_json::<()>(MobileResponse::err(e.to_string())),
    };
    let rank_str = match c_str_to_str(rank) {
        Ok(s) => s,
        Err(e) => return to_c_json::<()>(MobileResponse::err(e.to_string())),
    };
    let title_str = match c_str_to_str(title) {
        Ok(s) => s,
        Err(e) => return to_c_json::<()>(MobileResponse::err(e.to_string())),
    };

    match mobile_move_node(path_str, child_str, parent_str, rank_str, title_str) {
        Ok(nodes) => to_c_json(MobileResponse::ok(nodes)),
        Err(e) => to_c_json::<()>(MobileResponse::err(e.to_string())),
    }
}

#[no_mangle]
pub unsafe extern "C" fn narr_mobile_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = CString::from_raw(ptr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobile_bridge_flow() {
        let test_path = "/tmp/narratics_mobile_test.narr";
        let _ = std::fs::remove_file(test_path);

        // Open: root node is always initialized
        let state = mobile_open_project(test_path).expect("open failed");
        assert_eq!(state.nodes.len(), 1); // root

        // Move node
        let nodes = mobile_move_node(test_path, "sc_1", "root", "10", "1화: 시작").expect("move failed");
        assert_eq!(nodes.len(), 2); // root + sc_1

        // Save scene
        let sc = mobile_save_scene(test_path, "sc_1", "모바일에서 작성한 첫 번째 문단입니다.").expect("save failed");
        assert_eq!(sc.id, "sc_1");
        assert!(sc.word_count > 0);

        // Snapshot
        let snap_id = mobile_create_snapshot(test_path, "sc_1", "모바일 스냅샷", "내용").expect("snap failed");
        assert!(snap_id.starts_with("snap_mob_"));

        // Test C-ABI
        unsafe {
            let c_path = CString::new(test_path).unwrap();
            let c_res = narr_mobile_open(c_path.as_ptr());
            assert!(!c_res.is_null());

            let res_str = CStr::from_ptr(c_res).to_str().unwrap();
            assert!(res_str.contains("\"success\":true"));

            narr_mobile_free_string(c_res);
        }

        let _ = std::fs::remove_file(test_path);
    }
}
