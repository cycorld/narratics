mod commands;

use commands::{create_snapshot, export_manuscript, open_project, save_scene, DesktopState};
use std::sync::Arc;

fn main() {
    println!("=== Narratics Desktop Application v0.1.0 ===");
    println!("Architecture: Native Desktop Shell + Rust Engine-Core");

    let state = Arc::new(DesktopState::new());
    let test_path =
        std::env::var("NARRATICS_WORKSPACE").unwrap_or_else(|_| "workspace.narr".to_string());

    match open_project(&state, &test_path) {
        Ok(summary) => {
            println!("✓ Project Loaded: {}", summary.title);
            println!("  Author: {}", summary.author);
            println!("  Path: {}", summary.path);
            println!("  Scenes: {}", summary.scenes_count);
            println!("  Lore Entries: {}", summary.lore_count);
            println!("  Tree Nodes: {}", summary.tree_nodes.len());

            if let Some(first_node) = summary.tree_nodes.first() {
                let save_res = save_scene(
                    &state,
                    &first_node.id,
                    "데스크톱 네이티브 셸에서 직접 기록한 서사 단락입니다. 오프라인 CRDT 엔진 동기화 검증 완료.",
                );
                match save_res {
                    Ok(sc) => println!("✓ Scene Saved: ID={} (Words: {})", sc.id, sc.word_count),
                    Err(e) => eprintln!("✗ Scene save failed: {e}"),
                }

                let snap_res = create_snapshot(
                    &state,
                    &first_node.id,
                    "데스크톱 릴리즈 검증 스냅샷",
                    "스냅샷 텍스트",
                );
                match snap_res {
                    Ok(id) => println!("✓ Snapshot Created: {id}"),
                    Err(e) => eprintln!("✗ Snapshot failed: {e}"),
                }
            }

            match export_manuscript(&state, "markdown") {
                Ok(exp) => println!("✓ Manuscript Exported: {} words", exp.total_words),
                Err(e) => eprintln!("✗ Export failed: {e}"),
            }
        }
        Err(e) => {
            eprintln!("Error opening project: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::move_node;

    #[test]
    fn test_desktop_state_and_commands() {
        let state = Arc::new(DesktopState::new());
        let test_db = "/tmp/narratics_desktop_test.narr";
        let _ = std::fs::remove_file(test_db);

        // Open/create
        let summary = open_project(&state, test_db).expect("open failed");
        assert_eq!(summary.scenes_count, 0);

        // Move node / create node
        let tree_res =
            move_node(&state, "act_1", "root", "10", "제1막: 시작").expect("move failed");
        assert_eq!(tree_res.nodes.len(), 2); // root + act_1

        // Save scene
        let sc_res = save_scene(&state, "act_1", "테스트 본문 내용").expect("save scene failed");
        assert_eq!(sc_res.word_count, 9);

        // Snapshot
        let snap_id =
            create_snapshot(&state, "act_1", "v1.0", "테스트 본문 내용").expect("snap failed");
        assert!(snap_id.starts_with("snap_"));

        // Export
        let exp = export_manuscript(&state, "md").expect("export failed");
        assert!(exp.content.contains("제1막: 시작"));

        let _ = std::fs::remove_file(test_db);
    }
}
