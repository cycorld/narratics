use narratics_engine_core::{
    container::{LoreRecord, NarrContainer},
    text_engine::SceneEngine,
    tree_crdt::{MoveOp, TreeCRDT},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex, RwLock};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    #[serde(rename = "init")]
    Init {
        state: ProjectStateSnapshot,
    },
    #[serde(rename = "text_update")]
    TextUpdate {
        scene_id: String,
        text: String,
        word_count: usize,
        client_id: String,
    },
    #[serde(rename = "tree_move")]
    TreeMove {
        op: MoveOp,
        client_id: String,
    },
    #[serde(rename = "lore_update")]
    LoreUpdate {
        lore: LoreRecord,
        client_id: String,
    },
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "pong")]
    Pong,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BinderItemDto {
    pub id: String,
    pub parent: Option<String>,
    pub rank: String,
    pub title: String,
    pub is_folder: bool,
    pub word_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SceneDto {
    pub id: String,
    pub title: String,
    pub text: String,
    pub word_count: usize,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectStateSnapshot {
    pub title: String,
    pub author: String,
    pub binder: Vec<BinderItemDto>,
    pub active_scene_id: String,
    pub scenes: HashMap<String, SceneDto>,
    pub lore: Vec<LoreRecord>,
}

#[derive(Clone)]
pub struct AppState {
    pub container: Arc<Mutex<NarrContainer>>,
    pub tree: Arc<RwLock<TreeCRDT>>,
    pub scenes: Arc<RwLock<HashMap<String, SceneEngine>>>,
    pub scene_titles: Arc<RwLock<HashMap<String, String>>>,
    pub tx: broadcast::Sender<WsMessage>,
}

impl AppState {
    pub fn new(mut container: NarrContainer) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let (tx, _) = broadcast::channel(100);

        // Load or initialize TreeCRDT
        let mut tree = container.load_tree_crdt(1, "root")?;
        let (nodes, _) = tree.materialize();

        let mut scene_engines: HashMap<String, SceneEngine> = HashMap::new();
        let mut scene_titles: HashMap<String, String> = HashMap::new();

        // Seed initial project if empty
        if nodes.len() <= 1 {
            let op1 = tree.move_node("chap_1", "root", "10", "제1장: 안개 낀 항구");
            let op2 = tree.move_node("scene_1", "chap_1", "10", "1화: 밤안개를 가르는 돛");
            let op3 = tree.move_node("scene_2", "chap_1", "20", "2화: 밀회의 기록");
            let op4 = tree.move_node("chap_2", "root", "20", "제2장: 그림자 상단");
            let op5 = tree.move_node("scene_3", "chap_2", "10", "3화: 밀수선 바르바로사");

            container.save_tree_ops(&[op1, op2, op3, op4, op5])?;

            // Seed initial scenes
            let initial_text_1 = "자정이 가까워진 시각, 항구의 낡은 등대는 회전등을 깜빡이며 짙은 바다 안개를 희미하게 가르고 있었다.\n\n카일은 외투 깃을 바짝 올린 채 젖은 부두 끝자락에 멈춰 섰다. 차가운 바닷바람이 귓가를 스칠 때마다 등 뒤에 매어둔 단검 자루가 묵직하게 흔들렸다.\n\n\"약속 시각은 정각이었을 텐데.\"\n\n어둠 속에서 나직한 목소리가 들려왔다. 안개 너머로 모습을 드러낸 것은 그림자 상단의 연락책이었다.";
            let engine1 = SceneEngine::new("scene_1");
            engine1.insert(0, initial_text_1);
            let diff1 = engine1.encode_diff(None)?;
            container.save_scene("scene_1", &diff1, initial_text_1, engine1.char_count())?;
            scene_engines.insert("scene_1".to_string(), engine1);
            scene_titles.insert("scene_1".to_string(), "1화: 밤안개를 가르는 돛".to_string());

            let initial_text_2 = "탁자 위에 놓인 양초가 불규칙하게 흔들리며 벽면에 기괴한 그림자를 드리웠다.\n\n밀실에 모인 세 명의 남자는 아무도 입을 열지 않았다. 카일이 품에서 꺼낸 양피지에는 제국 수도의 경비 배치도와 에테르 결정의 운송 경로가 정밀하게 기록되어 있었다.";
            let engine2 = SceneEngine::new("scene_2");
            engine2.insert(0, initial_text_2);
            let diff2 = engine2.encode_diff(None)?;
            container.save_scene("scene_2", &diff2, initial_text_2, engine2.char_count())?;
            scene_engines.insert("scene_2".to_string(), engine2);
            scene_titles.insert("scene_2".to_string(), "2화: 밀회의 기록".to_string());

            let initial_text_3 = "새벽 세 시, 안개를 뚫고 거대한 칠흑빛 범선이 항만에 소리 없이 접안했다. 돛대 끝에 걸린 문양은 악명 높은 바르바로사의 표식이었다.";
            let engine3 = SceneEngine::new("scene_3");
            engine3.insert(0, initial_text_3);
            let diff3 = engine3.encode_diff(None)?;
            container.save_scene("scene_3", &diff3, initial_text_3, engine3.char_count())?;
            scene_engines.insert("scene_3".to_string(), engine3);
            scene_titles.insert("scene_3".to_string(), "3화: 밀수선 바르바로사".to_string());

            // Seed lore
            container.save_lore(&LoreRecord {
                id: "lore_1".to_string(),
                category: "등장인물".to_string(),
                name: "카일".to_string(),
                aliases: vec!["주인공".to_string(), "그림자검".to_string()],
                content: "몰락한 제국 귀족 가문 출신의 정보 밀매업자. 냉철한 판단력과 뛰어난 단검술을 보유하고 있으며, 붉은 달의 밤에 벌어진 비밀을 추적하고 있다.".to_string(),
                updated_at: 0,
            })?;

            container.save_lore(&LoreRecord {
                id: "lore_2".to_string(),
                category: "장소 및 세력".to_string(),
                name: "바르바로사".to_string(),
                aliases: vec!["바르바로사 호".to_string(), "밀수선".to_string()],
                content: "제국 해군의 순찰망을 무력화하며 암흑 항로를 독점하는 고속 쾌속선이자 독립 상단.".to_string(),
                updated_at: 0,
            })?;

            container.save_lore(&LoreRecord {
                id: "lore_3".to_string(),
                category: "세계관 설정".to_string(),
                name: "에테르 결정".to_string(),
                aliases: vec!["마력석".to_string(), "동력원".to_string()],
                content: "고대 유적에서 채굴되는 고농도 마력 집합체. 비공정과 고위 마법 무기의 필수 추진 연료로 사용된다.".to_string(),
                updated_at: 0,
            })?;

            container.set_meta("title", "달빛 아래의 크로니클")?;
            container.set_meta("author", "최용철")?;
            container.checkpoint()?;
        } else {
            // Load existing scenes
            let saved_scenes = container.list_scenes()?;
            for s in saved_scenes {
                let engine = SceneEngine::from_bytes(&s.id, &s.ydoc_state)?;
                scene_engines.insert(s.id.clone(), engine);
            }
            for (_id, node) in &nodes {
                scene_titles.insert(node.id.clone(), node.title.clone());
            }
        }

        Ok(Self {
            container: Arc::new(Mutex::new(container)),
            tree: Arc::new(RwLock::new(tree)),
            scenes: Arc::new(RwLock::new(scene_engines)),
            scene_titles: Arc::new(RwLock::new(scene_titles)),
            tx,
        })
    }

    pub async fn get_snapshot(&self) -> Result<ProjectStateSnapshot, Box<dyn std::error::Error + Send + Sync>> {
        let conn = self.container.lock().await;
        let tree = self.tree.read().await;
        let (nodes, _) = tree.materialize();
        let scenes = self.scenes.read().await;
        let titles = self.scene_titles.read().await;

        let title = conn.get_meta("title")?.unwrap_or_else(|| "Narratics Novel".to_string());
        let author = conn.get_meta("author")?.unwrap_or_else(|| "Author".to_string());

        let mut binder: Vec<BinderItemDto> = Vec::new();
        let mut scene_dtos: HashMap<String, SceneDto> = HashMap::new();

        for (id, node) in &nodes {
            if id == "root" {
                continue;
            }
            let is_folder = id.starts_with("chap_") || id.starts_with("folder_");
            let word_count = if let Some(engine) = scenes.get(id) {
                engine.char_count()
            } else {
                0
            };

            binder.push(BinderItemDto {
                id: id.clone(),
                parent: node.parent.clone(),
                rank: node.rank.clone(),
                title: node.title.clone(),
                is_folder,
                word_count,
            });
        }

        // Sort binder items by rank & title
        binder.sort_by(|a, b| a.rank.cmp(&b.rank).then_with(|| a.title.cmp(&b.title)));

        for (id, engine) in scenes.iter() {
            let title = titles.get(id).cloned().unwrap_or_else(|| id.clone());
            let text = engine.get_text();
            let count = engine.char_count();
            scene_dtos.insert(
                id.clone(),
                SceneDto {
                    id: id.clone(),
                    title,
                    text,
                    word_count: count,
                    updated_at: 0,
                },
            );
        }

        let lore = conn.list_lore()?;
        let active_scene_id = binder
            .iter()
            .find(|b| !b.is_folder)
            .map(|b| b.id.clone())
            .unwrap_or_else(|| "scene_1".to_string());

        Ok(ProjectStateSnapshot {
            title,
            author,
            binder,
            active_scene_id,
            scenes: scene_dtos,
            lore,
        })
    }
}
