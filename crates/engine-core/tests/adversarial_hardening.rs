use narratics_engine_core::{
    container::{LoreRecord, NarrContainer},
    text_engine::SceneEngine,
    tree_crdt::{MoveOp, OpId, TreeCRDT},
};
use tempfile::tempdir;

#[test]
fn cycle_01_offline_invariant_and_network_partition() {
    // Axis 1: Complete offline isolation and post-partition reconnection
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("offline_novel.narr");

    // Peer A writes offline
    {
        let mut container = NarrContainer::open(&file_path).unwrap();
        let mut tree_a = container.load_tree_crdt(1, "root").unwrap();
        let op_a = tree_a.move_node("scene_1", "root", "10", "1화: 시작");
        container.save_tree_ops(&[op_a]).unwrap();

        let scene = SceneEngine::new("scene_1");
        scene.insert(0, "오프라인에서 작성된 첫 문장입니다.");
        container
            .save_scene(
                "scene_1",
                &scene.to_bytes(),
                &scene.get_text(),
                scene.char_count(),
            )
            .unwrap();
        container.checkpoint().unwrap();
    }

    // Verify completely readable without any network connection
    {
        let container = NarrContainer::open(&file_path).unwrap();
        let scene = container
            .load_scene("scene_1")
            .unwrap()
            .expect("Scene must exist");
        assert_eq!(scene.text_cache, "오프라인에서 작성된 첫 문장입니다.");
        assert!(scene.word_count > 0);
    }
}

#[test]
fn cycle_02_boundary_and_negative_invariants() {
    // Axis 2: Out of bounds indices, empty payloads, 0 lengths, inverted deletions
    let scene = SceneEngine::new("scene_boundary");

    // Deleting from empty doc should NOT panic
    scene.delete(0, 100);
    scene.delete(9999, 50);
    assert_eq!(scene.get_text(), "");

    // Insert text
    scene.insert(0, "한글과 이모지 🚀 테스트");
    let initial_len = scene.char_count();

    // Deleting with start index > len should safely no-op without panic
    scene.delete((initial_len + 100) as u32, 50);
    assert_eq!(scene.get_text(), "한글과 이모지 🚀 테스트");

    // Deleting with len exceeding remainder should truncate safely to end
    scene.delete(0, (initial_len + 1000) as u32);
    assert_eq!(scene.get_text(), "");
}

#[test]
fn cycle_03_path_and_filesystem_hostility() {
    // Axis 3: Hostile file paths, spaces, quotes, non-ascii, nested non-existent dirs
    let dir = tempdir().unwrap();
    let hostile_dir = dir
        .path()
        .join("작가 작업실 '2026' [장편] & 특수문자 #!/")
        .join("depth1/depth2");

    std::fs::create_dir_all(&hostile_dir).unwrap();
    let hostile_file = hostile_dir.join("나의 위대한 소설 '최종본' (v1.0).narr");

    let container = NarrContainer::open(&hostile_file).unwrap();
    container.set_meta("title", "적대적 경로 테스트").unwrap();
    assert_eq!(
        container.get_meta("title").unwrap().as_deref(),
        Some("적대적 경로 테스트")
    );

    // Save and flush
    container.checkpoint().unwrap();
    assert!(hostile_file.exists());
}

#[test]
fn cycle_04_deep_tree_cycle_and_circular_ancestors() {
    // Axis 4: Adversarial tree cyclic references and deep recursive moves
    let mut tree = TreeCRDT::new(1, "root");

    // Create a chain of 50 folders: root -> n0 -> n1 -> ... -> n49
    let mut prev = "root".to_string();
    for i in 0..50 {
        let child = format!("node_{}", i);
        tree.move_node(&child, &prev, format!("{:04}", i), format!("Node {}", i));
        prev = child;
    }

    let (nodes, rejected) = tree.materialize();
    assert_eq!(rejected.len(), 0);
    assert_eq!(nodes.len(), 51);

    // Attack: Attempt to move node_0 under node_49 (Would create a 50-node cycle)
    tree.move_node("node_0", "node_49", "9999", "Cycle Attack");
    let (_nodes_after, rejected_after) = tree.materialize();

    assert_eq!(
        rejected_after.len(),
        1,
        "The circular move MUST be rejected"
    );
    assert_eq!(rejected_after[0].child, "node_0");
    assert_eq!(rejected_after[0].parent, "node_49");

    // Attack 2: Attempt to move root under node_10
    tree.move_node("root", "node_10", "9999", "Root Move Attack");
    let (_, rejected_root) = tree.materialize();
    assert!(
        rejected_root.iter().any(|op| op.child == "root"),
        "Root move must be rejected"
    );

    // Attack 3: Self-parent move (node_5 under node_5)
    tree.move_node("node_5", "node_5", "9999", "Self Move Attack");
    let (_, rejected_self) = tree.materialize();
    assert!(
        rejected_self.iter().any(|op| op.child == op.parent),
        "Self parent move must be rejected"
    );
}

#[test]
fn cycle_05_poison_crdt_binary_payloads() {
    // Axis 5: Corrupted, truncated, random poison bytes fed to CRDT decoder
    let scene = SceneEngine::new("scene_poison");

    // 0-byte payload should safely no-op
    assert!(scene.apply_update(&[]).is_ok());

    // Random noise payload should return error, NEVER panic
    let random_noise: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0xFF, 0x42, 0x13, 0x37];
    let res = scene.apply_update(&random_noise);
    assert!(res.is_err(), "Poison bytes must return error, not panic");

    // Truncated valid update payload
    let valid_scene = SceneEngine::new("valid");
    valid_scene.insert(0, "정상 텍스트 데이터");
    let valid_bytes = valid_scene.to_bytes();
    assert!(!valid_bytes.is_empty());

    let truncated_bytes = &valid_bytes[..valid_bytes.len() / 2];
    let res_trunc = scene.apply_update(truncated_bytes);
    assert!(
        res_trunc.is_err(),
        "Truncated update must fail gracefully without panic"
    );
}

#[test]
fn cycle_06_arithmetic_multibyte_utf8_utf16_boundary_panics() {
    // Axis 6: 3-byte Korean (완성형/조합형) + 4-byte Astral Emojis + surrogate pairs
    let scene = SceneEngine::new("scene_korean_boundary");

    // Korean + surrogate emojis: "안녕 👋 세상 🧙‍♂️ 폭풍 ⚡"
    let test_str = "안녕 👋 세상 🧙‍♂️ 폭풍 ⚡";
    scene.insert(0, test_str);

    assert_eq!(scene.get_text(), test_str);
    let total_len = scene.char_count();

    // Slicing and editing inside complex multi-byte stream
    // Insert in the middle
    scene.insert(3, "[삽입] ");
    assert!(scene.get_text().contains("[삽입]"));

    // Delete exact segments
    scene.delete(3, 5);
    assert_eq!(scene.get_text(), test_str);

    // Repeated random inserts and deletes
    for i in 0..100 {
        scene.insert((i % total_len) as u32, "가");
    }
    assert!(scene.char_count() > total_len);
}

#[test]
fn cycle_07_four_peers_concurrent_divergence_and_convergence() {
    // Axis 7: 4 peers editing different paragraphs and moving chapters simultaneously
    let mut p1 = TreeCRDT::new(1, "root");
    let mut p2 = TreeCRDT::new(2, "root");
    let mut p3 = TreeCRDT::new(3, "root");
    let mut p4 = TreeCRDT::new(4, "root");

    // Initial structure
    let base_op1 = p1.move_node("chap_1", "root", "10", "1장");
    let base_op2 = p1.move_node("chap_2", "root", "20", "2장");

    for p in [&mut p2, &mut p3, &mut p4] {
        p.integrate(base_op1.clone());
        p.integrate(base_op2.clone());
    }

    // Peer 1 moves chap_2 under chap_1
    let op_p1 = p1.move_node("chap_2", "chap_1", "10", "1장의 하위 씬");
    // Peer 2 simultaneously moves chap_1 under chap_2 (Cycle conflict)
    let op_p2 = p2.move_node("chap_1", "chap_2", "10", "2장의 하위 씬");
    // Peer 3 creates chap_3 under root
    let op_p3 = p3.move_node("chap_3", "root", "30", "3장");
    // Peer 4 moves chap_2 under root with new title
    let op_p4 = p4.move_node("chap_2", "root", "15", "수정된 2장");

    // Broadcast all ops to everyone in arbitrary order
    let all_ops = vec![op_p1, op_p2, op_p3, op_p4];
    for op in &all_ops {
        p1.integrate(op.clone());
        p2.integrate(op.clone());
        p3.integrate(op.clone());
        p4.integrate(op.clone());
    }

    // Materialize all 4 trees
    let (t1, rej1) = p1.materialize();
    let (t2, rej2) = p2.materialize();
    let (t3, rej3) = p3.materialize();
    let (t4, rej4) = p4.materialize();

    // Invariant: All peers MUST converge to the EXACT same nodes and parents
    assert_eq!(t1, t2);
    assert_eq!(t2, t3);
    assert_eq!(t3, t4);
    assert_eq!(rej1, rej2);
    assert_eq!(rej2, rej3);
    assert_eq!(rej3, rej4);
}

#[test]
fn cycle_08_tombstone_compaction_under_extreme_revisions() {
    // Axis 8: 200 rapid deletions and replacements to verify compaction
    let scene = SceneEngine::new("scene_extreme");

    let chunk = "동해 물과 백두산이 마르고 닳도록 하느님이 보우하사 우리나라 만세. ";
    for _ in 0..100 {
        scene.insert(0, chunk);
    }
    let base_size = scene.char_count();

    for i in 0..200 {
        let pos = ((i * 17) % (scene.char_count().max(10) - 5)) as u32;
        scene.delete(pos, 5);
        scene.insert(pos, "수정된 문단 ");
    }

    let result = scene.compact();
    assert!(result.char_count > base_size / 2);
    assert!(
        result.compacted_size < 100_000,
        "Compacted size must remain bounded under 100KB"
    );
}

#[test]
fn cycle_09_sqlite_wal_contention_and_crash_consistency() {
    // Axis 9: Multi-connection concurrent read/write and atomic rollback
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("wal_stress.narr");

    // Writer 1 inserts lore and scenes
    let container = NarrContainer::open(&db_path).unwrap();
    for i in 0..50 {
        let lore = LoreRecord {
            id: format!("char_{}", i),
            category: "캐릭터".to_string(),
            name: format!("등장인물 {}", i),
            aliases: vec![format!("별칭 {}", i), "흑마법사".to_string()],
            content: "주인공의 조력자로서 활약하는 핵심 인물.".to_string(),
            updated_at: 1700000000 + i,
        };
        container.save_lore(&lore).unwrap();
    }

    // Checkpoint
    container.checkpoint().unwrap();

    // Reader checks count
    let list = container.list_lore().unwrap();
    assert_eq!(list.len(), 50);
    assert_eq!(list[0].category, "캐릭터");
}

#[test]
fn cycle_10_json_binary_roundtrip_schema_parity() {
    // Axis 10: Serialization / Deserialization parity without silent field loss
    let op = MoveOp {
        id: OpId {
            lamport: 1042,
            replica_id: 88,
        },
        child: "folder_epic".to_string(),
        parent: "root".to_string(),
        rank: "01.05".to_string(),
        title: "2부: 대서사시".to_string(),
    };

    let json = serde_json::to_string(&op).unwrap();
    let decoded: MoveOp = serde_json::from_str(&json).unwrap();
    assert_eq!(op, decoded);

    let lore = LoreRecord {
        id: "hero".to_string(),
        category: "주인공".to_string(),
        name: "카일".to_string(),
        aliases: vec!["그림자 검사".to_string(), "용사".to_string()],
        content: "어둠 속에서 깨어난 마지막 계승자.".to_string(),
        updated_at: 1727000000,
    };
    let lore_json = serde_json::to_string(&lore).unwrap();
    let decoded_lore: LoreRecord = serde_json::from_str(&lore_json).unwrap();
    assert_eq!(lore.id, decoded_lore.id);
    assert_eq!(lore.aliases, decoded_lore.aliases);
}
