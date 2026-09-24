use narratics_engine_core::{
    container::{NarrContainer, SceneRecord, SnapshotRecord},
    text_engine::SceneEngine,
    tree_crdt::{NodeInfo, ReplicaId, TreeCRDT},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectSummary {
    pub path: String,
    pub title: String,
    pub author: String,
    pub tree_nodes: Vec<NodeInfo>,
    pub scenes_count: usize,
    pub lore_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SceneSummary {
    pub id: String,
    pub word_count: usize,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct TreeSummary {
    pub ops_count: usize,
    pub nodes: Vec<NodeInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExportSummary {
    pub format: String,
    pub total_words: usize,
    pub content: String,
}

pub struct DesktopState {
    pub current_path: Mutex<Option<String>>,
    pub container: Mutex<Option<NarrContainer>>,
    pub tree: Mutex<TreeCRDT>,
    pub replica_id: ReplicaId,
}

impl DesktopState {
    pub fn new() -> Self {
        Self {
            current_path: Mutex::new(None),
            container: Mutex::new(None),
            tree: Mutex::new(TreeCRDT::new(101, "root")),
            replica_id: 101, // Desktop client replica ID
        }
    }
}

pub fn open_project(state: &Arc<DesktopState>, path_str: &str) -> Result<ProjectSummary, String> {
    let container =
        NarrContainer::open(path_str).map_err(|e| format!("컨테이너 열기 실패: {e}"))?;

    let title = container
        .get_meta("title")
        .map_err(|e| format!("메타데이터 조회 실패: {e}"))?
        .unwrap_or_else(|| "제목 없는 서사".into());
    let author = container
        .get_meta("author")
        .map_err(|e| format!("메타데이터 조회 실패: {e}"))?
        .unwrap_or_else(|| "작가".into());

    let tree = container
        .load_tree_crdt(state.replica_id, "root")
        .map_err(|e| format!("트리 CRDT 로드 실패: {e}"))?;

    let (nodes_map, _) = tree.materialize();
    let mut tree_nodes: Vec<NodeInfo> = nodes_map.into_values().collect();
    tree_nodes.sort_by(|a, b| a.rank.cmp(&b.rank));

    let scenes = container
        .list_scenes()
        .map_err(|e| format!("씬 목록 조회 실패: {e}"))?;
    let lore_list = container
        .list_lore()
        .map_err(|e| format!("로어 로드 실패: {e}"))?;

    *state.tree.lock().unwrap() = tree;
    *state.current_path.lock().unwrap() = Some(path_str.to_string());
    *state.container.lock().unwrap() = Some(container);

    Ok(ProjectSummary {
        path: path_str.to_string(),
        title,
        author,
        tree_nodes,
        scenes_count: scenes.len(),
        lore_count: lore_list.len(),
    })
}

pub fn save_scene(
    state: &Arc<DesktopState>,
    scene_id: &str,
    text: &str,
) -> Result<SceneSummary, String> {
    let mut conn_guard = state.container.lock().unwrap();
    let container = conn_guard
        .as_mut()
        .ok_or_else(|| "열려있는 프로젝트가 없습니다.".to_string())?;

    let engine = SceneEngine::new(scene_id);
    engine.insert(0, text);
    let diff = engine
        .encode_diff(None)
        .map_err(|e| format!("인코딩 실패: {e}"))?;
    let char_count = engine.char_count();

    container
        .save_scene(scene_id, &diff, text, char_count)
        .map_err(|e| format!("씬 저장 실패: {e}"))?;

    Ok(SceneSummary {
        id: scene_id.to_string(),
        word_count: char_count,
        updated_at: chrono::Utc::now().timestamp(),
    })
}

#[allow(dead_code)]
pub fn move_node(
    state: &Arc<DesktopState>,
    child_id: &str,
    parent_id: &str,
    rank: &str,
    title: &str,
) -> Result<TreeSummary, String> {
    let op = {
        let mut tree = state.tree.lock().unwrap();
        tree.move_node(child_id, parent_id, rank, title)
    };

    {
        let mut conn_guard = state.container.lock().unwrap();
        if let Some(container) = conn_guard.as_mut() {
            let _ = container.save_tree_ops(&[op]);
        }
    }

    let nodes = {
        let tree = state.tree.lock().unwrap();
        let (nodes_map, _) = tree.materialize();
        let mut nodes: Vec<NodeInfo> = nodes_map.into_values().collect();
        nodes.sort_by(|a, b| a.rank.cmp(&b.rank));
        nodes
    };

    Ok(TreeSummary {
        ops_count: 1,
        nodes,
    })
}

pub fn create_snapshot(
    state: &Arc<DesktopState>,
    target_id: &str,
    label: &str,
    content: &str,
) -> Result<String, String> {
    let mut conn_guard = state.container.lock().unwrap();
    let container = conn_guard
        .as_mut()
        .ok_or_else(|| "열려있는 프로젝트가 없습니다.".to_string())?;

    let snap_id = format!("snap_{}", chrono::Utc::now().timestamp_millis());
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
        .map_err(|e| format!("스냅샷 저장 실패: {e}"))?;

    Ok(snap_id)
}

pub fn export_manuscript(
    state: &Arc<DesktopState>,
    format_type: &str,
) -> Result<ExportSummary, String> {
    let (title, author, scenes) = {
        let mut conn_guard = state.container.lock().unwrap();
        let container = conn_guard
            .as_mut()
            .ok_or_else(|| "열려있는 프로젝트가 없습니다.".to_string())?;

        let title = container
            .get_meta("title")
            .ok()
            .flatten()
            .unwrap_or_else(|| "내러틱스 원고".into());
        let author = container
            .get_meta("author")
            .ok()
            .flatten()
            .unwrap_or_else(|| "작가".into());
        let scenes = container.list_scenes().unwrap_or_default();
        (title, author, scenes)
    };

    let nodes = {
        let tree = state.tree.lock().unwrap();
        let (nodes_map, _) = tree.materialize();
        let mut nodes: Vec<NodeInfo> = nodes_map.into_values().collect();
        nodes.sort_by(|a, b| a.rank.cmp(&b.rank));
        nodes
    };

    let scene_map: HashMap<String, SceneRecord> =
        scenes.into_iter().map(|s| (s.id.clone(), s)).collect();

    let mut combined_text = String::new();
    combined_text.push_str(&format!("# {}\n\n저자: {}\n\n---\n\n", title, author));

    let mut total_words = 0;
    for node in nodes {
        if let Some(scene) = scene_map.get(&node.id) {
            combined_text.push_str(&format!("## {}\n\n{}\n\n", node.title, scene.text_cache));
            total_words += scene.word_count;
        }
    }

    Ok(ExportSummary {
        format: format_type.to_string(),
        total_words,
        content: combined_text,
    })
}
