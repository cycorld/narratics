use narratics_engine_core::{
    container::{LoreRecord, NarrContainer, SnapshotRecord},
    text_engine::SceneEngine,
    tree_crdt::{MoveOp, TreeCRDT},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex, RwLock};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    #[serde(rename = "init")]
    Init { state: ProjectStateSnapshot },
    #[serde(rename = "text_update")]
    TextUpdate {
        scene_id: String,
        text: String,
        word_count: usize,
        client_id: String,
    },
    #[serde(rename = "tree_move")]
    TreeMove { op: MoveOp, client_id: String },
    #[serde(rename = "lore_update")]
    LoreUpdate { lore: LoreRecord, client_id: String },
    #[serde(rename = "snapshot_created")]
    SnapshotCreated {
        snapshot: SnapshotRecord,
        client_id: String,
    },
    #[serde(rename = "scene_status_updated")]
    SceneStatusUpdated { scene_id: String, status: String },
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
    pub status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SceneDto {
    pub id: String,
    pub title: String,
    pub text: String,
    pub word_count: usize,
    pub status: String,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectInfoDto {
    pub id: String,
    pub file_name: String,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub synopsis: String,
    pub target_words: usize,
    pub total_words: usize,
    pub scenes_count: usize,
    pub chapters_count: usize,
    pub updated_at: i64,
    pub is_active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectStateSnapshot {
    pub project_id: String,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub synopsis: String,
    pub target_words: usize,
    pub total_words: usize,
    pub binder: Vec<BinderItemDto>,
    pub active_scene_id: String,
    pub scenes: HashMap<String, SceneDto>,
    pub lore: Vec<LoreRecord>,
    pub snapshots: Vec<SnapshotRecord>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateProjectReq {
    pub title: String,
    pub author: Option<String>,
    pub genre: Option<String>,
    pub synopsis: Option<String>,
    pub target_words: Option<usize>,
    pub template: Option<String>, // "blank" | "three_act" | "webnovel"
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateProjectMetaReq {
    pub title: Option<String>,
    pub author: Option<String>,
    pub genre: Option<String>,
    pub synopsis: Option<String>,
    pub target_words: Option<usize>,
}

#[derive(Clone)]
pub struct AppState {
    pub projects_dir: PathBuf,
    pub active_project_id: Arc<RwLock<String>>,
    pub container: Arc<Mutex<NarrContainer>>,
    pub tree: Arc<RwLock<TreeCRDT>>,
    pub scenes: Arc<RwLock<HashMap<String, SceneEngine>>>,
    pub scene_titles: Arc<RwLock<HashMap<String, String>>>,
    pub scene_statuses: Arc<RwLock<HashMap<String, String>>>,
    pub tx: broadcast::Sender<WsMessage>,
}

impl AppState {
    pub fn new(
        projects_dir: PathBuf,
        initial_file: PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let (tx, _) = broadcast::channel(100);

        std::fs::create_dir_all(&projects_dir)?;

        // Ensure at least one project exists
        let active_file = if initial_file.exists() {
            initial_file
        } else {
            projects_dir.join("moonlight_chronicles.narr")
        };

        let active_id = active_file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("moonlight_chronicles")
            .to_string();

        let mut container = NarrContainer::open(&active_file)?;

        // Load or initialize TreeCRDT
        let mut tree = container.load_tree_crdt(1, "root")?;
        let (nodes, _) = tree.materialize_active();

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
            container.set_meta("genre", "다크 판타지")?;
            container.set_meta(
                "synopsis",
                "제국의 암운 속에서 붉은 달의 비밀을 쫓는 밀매업자 카일의 서사시.",
            )?;
            container.set_meta("target_words", "100000")?;
            container.checkpoint()?;
        } else {
            // Load existing scenes
            let saved_scenes = container.list_scenes()?;
            for s in saved_scenes {
                let engine = SceneEngine::from_bytes(&s.id, &s.ydoc_state)?;
                scene_engines.insert(s.id.clone(), engine);
            }
            for node in nodes.values() {
                scene_titles.insert(node.id.clone(), node.title.clone());
            }
        }

        Ok(Self {
            projects_dir,
            active_project_id: Arc::new(RwLock::new(active_id)),
            container: Arc::new(Mutex::new(container)),
            tree: Arc::new(RwLock::new(tree)),
            scenes: Arc::new(RwLock::new(scene_engines)),
            scene_titles: Arc::new(RwLock::new(scene_titles)),
            scene_statuses: Arc::new(RwLock::new(HashMap::new())),
            tx,
        })
    }

    pub async fn list_projects(
        &self,
    ) -> Result<Vec<ProjectInfoDto>, Box<dyn std::error::Error + Send + Sync>> {
        let active_id = self.active_project_id.read().await.clone();
        let mut list = Vec::new();

        if !self.projects_dir.exists() {
            return Ok(list);
        }

        for entry in std::fs::read_dir(&self.projects_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("narr") {
                let stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                let is_active = stem == active_id;

                if is_active {
                    // Read live state
                    let snap = self.get_snapshot().await?;
                    let chapters = snap.binder.iter().filter(|b| b.is_folder).count();
                    let scenes = snap.binder.iter().filter(|b| !b.is_folder).count();
                    let updated_at = std::fs::metadata(&path)
                        .and_then(|m| m.modified())
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d| d.as_secs() as i64)
                        .unwrap_or(0);

                    list.push(ProjectInfoDto {
                        id: stem.clone(),
                        file_name: path
                            .file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or("")
                            .to_string(),
                        title: snap.title,
                        author: snap.author,
                        genre: snap.genre,
                        synopsis: snap.synopsis,
                        target_words: snap.target_words,
                        total_words: snap.total_words,
                        scenes_count: scenes,
                        chapters_count: chapters,
                        updated_at,
                        is_active: true,
                    });
                } else {
                    // Read file metadata via isolated container
                    if let Ok(c) = NarrContainer::open(&path) {
                        let title = c.get_meta("title")?.unwrap_or_else(|| stem.clone());
                        let author = c
                            .get_meta("author")?
                            .unwrap_or_else(|| "작가 미상".to_string());
                        let genre = c.get_meta("genre")?.unwrap_or_else(|| "일반".to_string());
                        let synopsis = c.get_meta("synopsis")?.unwrap_or_default();
                        let target_words = c
                            .get_meta("target_words")?
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(100000);
                        let scenes = c.list_scenes().unwrap_or_default();
                        let scenes_count = scenes.len();
                        let total_words: usize = scenes.iter().map(|s| s.word_count).sum();
                        let updated_at = std::fs::metadata(&path)
                            .and_then(|m| m.modified())
                            .ok()
                            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                            .map(|d| d.as_secs() as i64)
                            .unwrap_or(0);

                        list.push(ProjectInfoDto {
                            id: stem.clone(),
                            file_name: path
                                .file_name()
                                .and_then(|s| s.to_str())
                                .unwrap_or("")
                                .to_string(),
                            title,
                            author,
                            genre,
                            synopsis,
                            target_words,
                            total_words,
                            scenes_count,
                            chapters_count: 1,
                            updated_at,
                            is_active: false,
                        });
                    }
                }
            }
        }

        list.sort_by(|a, b| {
            b.is_active
                .cmp(&a.is_active)
                .then_with(|| b.updated_at.cmp(&a.updated_at))
        });
        Ok(list)
    }

    pub async fn create_project(
        &self,
        req: CreateProjectReq,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let clean_title = req.title.trim();
        if clean_title.is_empty() {
            return Err("작품 제목은 필수입니다.".into());
        }

        let now_ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        // Create slug
        let slug_base: String = clean_title
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
            .collect();
        let slug = if slug_base.is_empty() {
            format!("book_{}", now_ts)
        } else {
            format!(
                "{}_{}",
                slug_base,
                &now_ts.to_string()[now_ts.to_string().len().saturating_sub(4)..]
            )
        };

        let file_path = self.projects_dir.join(format!("{}.narr", slug));
        let mut new_container = NarrContainer::open(&file_path)?;

        let author = req.author.as_deref().unwrap_or("최용철");
        let genre = req.genre.as_deref().unwrap_or("장편소설");
        let synopsis = req.synopsis.as_deref().unwrap_or("");
        let target_words = req.target_words.unwrap_or(100000);

        new_container.set_meta("title", clean_title)?;
        new_container.set_meta("author", author)?;
        new_container.set_meta("genre", genre)?;
        new_container.set_meta("synopsis", synopsis)?;
        new_container.set_meta("target_words", &target_words.to_string())?;

        let template = req.template.as_deref().unwrap_or("three_act");
        let mut tree = new_container.load_tree_crdt(1, "root")?;

        match template {
            "blank" => {
                let op1 = tree.move_node("chap_1", "root", "10", "제1장: 시작");
                let op2 = tree.move_node("scene_1", "chap_1", "10", "1화: 첫 번째 장면");
                new_container.save_tree_ops(&[op1, op2])?;

                let engine = SceneEngine::new("scene_1");
                let diff = engine.encode_diff(None)?;
                new_container.save_scene("scene_1", &diff, "", 0)?;
            }
            "webnovel" => {
                let op0 = tree.move_node("chap_0", "root", "05", "프롤로그");
                let op0_1 = tree.move_node("scene_0", "chap_0", "10", "프롤로그: 각성의 순간");
                let op1 = tree.move_node("chap_1", "root", "10", "제1권: 귀환자");
                let op1_1 = tree.move_node("scene_1", "chap_1", "10", "1화: F급 헌터의 최후");
                let op1_2 = tree.move_node("scene_2", "chap_1", "20", "2화: 시간을 거스른 자");
                let op1_3 = tree.move_node("scene_3", "chap_1", "30", "3화: 히든 퀘스트 개방");
                new_container.save_tree_ops(&[op0, op0_1, op1, op1_1, op1_2, op1_3])?;

                for (sc_id, _sc_title, sc_intro) in &[
                    ("scene_0", "프롤로그: 각성의 순간", "그것은 모든 것이 끝났다고 생각했던 순간에 찾아왔다.\n\n눈앞이 암전되고 전신을 찢는 듯한 고통이 몰려왔을 때, 귓가에 낯익은 기계음이 울려 퍼졌다.\n\n[축하합니다. 히든 칭호 '마지막 생존자'를 획득하셨습니다.]"),
                    ("scene_1", "1화: F급 헌터의 최후", "피비린내 나는 던전 지하 7층.\n\n부러진 칼자루를 쥔 손이 잘게 떨렸다. 카일은 가쁜 숨을 몰아쉬며 붉은 눈의 마수들을 노려보았다."),
                    ("scene_2", "2화: 시간을 거스른 자", "눈을 떴을 때, 천장에 걸린 낡은 형광등이 깜빡이고 있었다.\n\n달력에 적힌 날짜는 정확히 10년 전, 게이트가 처음 열렸던 바로 그날이었다."),
                    ("scene_3", "3화: 히든 퀘스트 개방", "상태창을 호출하자 반투명한 푸른빛 홀로그램이 허공에 떠올랐다.")
                ] {
                    let engine = SceneEngine::new(*sc_id);
                    engine.insert(0, sc_intro);
                    let diff = engine.encode_diff(None)?;
                    new_container.save_scene(sc_id, &diff, sc_intro, engine.char_count())?;
                }
            }
            _ /* three_act */ => {
                let op1 = tree.move_node("chap_1", "root", "10", "제1막: 발단과 일상의 균열");
                let op1_1 = tree.move_node("scene_1", "chap_1", "10", "1화: 평온한 일상의 균열");
                let op1_2 = tree.move_node("scene_2", "chap_1", "20", "2화: 거부할 수 없는 부름");
                let op2 = tree.move_node("chap_2", "root", "20", "제2막: 시련과 중간점의 반전");
                let op2_1 = tree.move_node("scene_3", "chap_2", "10", "3화: 새로운 세계로의 진입");
                let op2_2 = tree.move_node("scene_4", "chap_2", "20", "4화: 중간점의 대반전");
                let op3 = tree.move_node("chap_3", "root", "30", "제3막: 절정과 새로운 결말");
                let op3_1 = tree.move_node("scene_5", "chap_3", "10", "5화: 최후의 결전");
                let op3_2 = tree.move_node("scene_6", "chap_3", "20", "6화: 새로운 시작");
                new_container.save_tree_ops(&[op1, op1_1, op1_2, op2, op2_1, op2_2, op3, op3_1, op3_2])?;

                for (sc_id, _sc_title, sc_intro) in &[
                    ("scene_1", "1화: 평온한 일상의 균열", "시작은 지극히 사소한 불협화음이었다. 언제나처럼 평화롭던 아침, 우체통에 도착한 검은 밀랍 인장의 편지가 모든 것을 뒤흔들었다."),
                    ("scene_2", "2화: 거부할 수 없는 부름", "더 이상 물러설 곳은 없었다. 과거를 뒤로하고 길을 떠나야만 했다."),
                    ("scene_3", "3화: 새로운 세계로의 진입", "안개 너머로 모습을 드러낸 미지의 대륙은 규율도 상식도 통하지 않는 야만의 땅이었다."),
                    ("scene_4", "4화: 중간점의 대반전", "가장 믿었던 동료의 단검이 등 뒤를 겨누었을 때, 모든 진실이 차갑게 드러났다."),
                    ("scene_5", "5화: 최후의 결전", "폭풍우 치는 요새의 정상에서, 두 개의 운명이 마침내 정면으로 충돌했다."),
                    ("scene_6", "6화: 새로운 시작", "전쟁의 먼지가 가라앉고 태양이 다시 솟아올랐다. 폐허 위에서 새로운 역사가 쓰이기 시작했다.")
                ] {
                    let engine = SceneEngine::new(*sc_id);
                    engine.insert(0, sc_intro);
                    let diff = engine.encode_diff(None)?;
                    new_container.save_scene(sc_id, &diff, sc_intro, engine.char_count())?;
                }
            }
        }

        new_container.checkpoint()?;
        drop(new_container);

        // Switch to the newly created project
        self.switch_project(&slug).await?;
        Ok(slug)
    }

    pub async fn switch_project(
        &self,
        project_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let file_path = self.projects_dir.join(format!("{}.narr", project_id));
        if !file_path.exists() {
            return Err(format!("프로젝트 파일을 찾을 수 없습니다: {:?}", file_path).into());
        }

        // Checkpoint old container
        {
            let conn = self.container.lock().await;
            let _ = conn.checkpoint();
        }

        // Open new container
        let new_container = NarrContainer::open(&file_path)?;
        let new_tree = new_container.load_tree_crdt(1, "root")?;
        let (nodes, _) = new_tree.materialize_active();

        let mut new_scenes = HashMap::new();
        let mut new_titles = HashMap::new();

        let saved = new_container.list_scenes()?;
        for s in saved {
            let engine = SceneEngine::from_bytes(&s.id, &s.ydoc_state)?;
            new_scenes.insert(s.id.clone(), engine);
        }
        for node in nodes.values() {
            new_titles.insert(node.id.clone(), node.title.clone());
        }

        // Apply new state
        {
            let mut c = self.container.lock().await;
            *c = new_container;
        }
        {
            let mut t = self.tree.write().await;
            *t = new_tree;
        }
        {
            let mut s = self.scenes.write().await;
            *s = new_scenes;
        }
        {
            let mut st = self.scene_titles.write().await;
            *st = new_titles;
        }
        {
            let mut stat = self.scene_statuses.write().await;
            stat.clear();
        }
        {
            let mut pid = self.active_project_id.write().await;
            *pid = project_id.to_string();
        }

        // Broadcast Init to all clients
        let snap = self.get_snapshot().await?;
        let _ = self.tx.send(WsMessage::Init { state: snap });
        Ok(())
    }

    pub async fn delete_project(
        &self,
        project_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let active_id = self.active_project_id.read().await.clone();
        if active_id == project_id {
            // Cannot delete active project without switching
            let projects = self.list_projects().await?;
            let other = projects.iter().find(|p| p.id != project_id);
            if let Some(other_proj) = other {
                self.switch_project(&other_proj.id).await?;
            } else {
                return Err("마지막 남은 프로젝트는 삭제할 수 없습니다.".into());
            }
        }

        let base_path = self.projects_dir.join(format!("{}.narr", project_id));
        let _ = std::fs::remove_file(&base_path);
        let _ = std::fs::remove_file(format!("{}-wal", base_path.display()));
        let _ = std::fs::remove_file(format!("{}-shm", base_path.display()));
        Ok(())
    }

    pub async fn update_project_meta(
        &self,
        req: UpdateProjectMetaReq,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let conn = self.container.lock().await;
        if let Some(t) = req.title {
            if !t.trim().is_empty() {
                conn.set_meta("title", t.trim())?;
            }
        }
        if let Some(a) = req.author {
            conn.set_meta("author", a.trim())?;
        }
        if let Some(g) = req.genre {
            conn.set_meta("genre", g.trim())?;
        }
        if let Some(s) = req.synopsis {
            conn.set_meta("synopsis", s.trim())?;
        }
        if let Some(w) = req.target_words {
            conn.set_meta("target_words", &w.to_string())?;
        }
        conn.checkpoint()?;
        drop(conn);

        let snap = self.get_snapshot().await?;
        let _ = self.tx.send(WsMessage::Init { state: snap });
        Ok(())
    }

    pub async fn rename_binder_node(
        &self,
        node_id: &str,
        new_title: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let clean_title = new_title.trim();
        if clean_title.is_empty() {
            return Err("노드 제목은 비워둘 수 없습니다.".into());
        }

        let op = {
            let mut tree = self.tree.write().await;
            let (nodes, _) = tree.materialize_active();
            if let Some(node) = nodes.get(node_id) {
                let parent = node.parent.clone().unwrap_or_else(|| "root".to_string());
                let op = tree.move_node(node_id, &parent, &node.rank, clean_title);
                Some(op)
            } else {
                None
            }
        };

        if let Some(op) = op {
            let mut conn = self.container.lock().await;
            conn.save_tree_ops(std::slice::from_ref(&op))?;
            drop(conn);

            let mut titles = self.scene_titles.write().await;
            titles.insert(node_id.to_string(), clean_title.to_string());
            drop(titles);

            let _ = self.tx.send(WsMessage::TreeMove {
                op,
                client_id: "server".to_string(),
            });
        }
        Ok(())
    }

    pub async fn delete_binder_node(
        &self,
        node_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let op = {
            let mut tree = self.tree.write().await;
            tree.move_node(node_id, "trash", "0", "deleted")
        };

        let mut conn = self.container.lock().await;
        conn.save_tree_ops(&[op])?;
        let _ = conn.delete_scene(node_id);
        drop(conn);

        {
            let mut scenes = self.scenes.write().await;
            scenes.remove(node_id);
        }
        {
            let mut titles = self.scene_titles.write().await;
            titles.remove(node_id);
        }
        {
            let mut statuses = self.scene_statuses.write().await;
            statuses.remove(node_id);
        }

        let snap = self.get_snapshot().await?;
        let _ = self.tx.send(WsMessage::Init { state: snap });
        Ok(())
    }

    pub async fn reorder_binder_node(
        &self,
        node_id: &str,
        direction: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut tree = self.tree.write().await;
        let (nodes, _) = tree.materialize_active();
        let target_node = match nodes.get(node_id) {
            Some(n) => n.clone(),
            None => return Ok(()),
        };

        let parent_id = target_node
            .parent
            .clone()
            .unwrap_or_else(|| "root".to_string());
        let mut siblings: Vec<_> = nodes
            .values()
            .filter(|n| n.parent.as_deref().unwrap_or("root") == parent_id.as_str())
            .cloned()
            .collect();

        siblings.sort_by(|a, b| a.rank.cmp(&b.rank).then_with(|| a.title.cmp(&b.title)));

        let idx = match siblings.iter().position(|n| n.id == node_id) {
            Some(i) => i,
            None => return Ok(()),
        };

        let swap_idx = if direction == "up" {
            if idx == 0 {
                return Ok(());
            }
            idx - 1
        } else {
            if idx + 1 >= siblings.len() {
                return Ok(());
            }
            idx + 1
        };

        let current_rank = siblings[idx].rank.clone();
        let other_rank = siblings[swap_idx].rank.clone();

        // If ranks are equal, re-space ranks
        let (new_target_rank, new_other_rank) = if current_rank == other_rank {
            if direction == "up" {
                (
                    format!("{:04}", (swap_idx * 10)),
                    format!("{:04}", (idx * 10 + 5)),
                )
            } else {
                (
                    format!("{:04}", (swap_idx * 10 + 5)),
                    format!("{:04}", (idx * 10)),
                )
            }
        } else {
            (other_rank, current_rank)
        };

        let op1 = tree.move_node(
            &siblings[idx].id,
            &parent_id,
            &new_target_rank,
            &siblings[idx].title,
        );
        let op2 = tree.move_node(
            &siblings[swap_idx].id,
            &parent_id,
            &new_other_rank,
            &siblings[swap_idx].title,
        );

        let mut conn = self.container.lock().await;
        conn.save_tree_ops(&[op1, op2])?;
        drop(conn);
        drop(tree);

        let snap = self.get_snapshot().await?;
        let _ = self.tx.send(WsMessage::Init { state: snap });
        Ok(())
    }

    pub async fn delete_lore(
        &self,
        lore_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let conn = self.container.lock().await;
        conn.delete_lore(lore_id)?;
        drop(conn);

        let snap = self.get_snapshot().await?;
        let _ = self.tx.send(WsMessage::Init { state: snap });
        Ok(())
    }

    pub async fn get_snapshot(
        &self,
    ) -> Result<ProjectStateSnapshot, Box<dyn std::error::Error + Send + Sync>> {
        let conn = self.container.lock().await;
        let tree = self.tree.read().await;
        let (nodes, _) = tree.materialize_active();
        let scenes = self.scenes.read().await;
        let titles = self.scene_titles.read().await;
        let statuses = self.scene_statuses.read().await;
        let project_id = self.active_project_id.read().await.clone();

        let title = conn
            .get_meta("title")?
            .unwrap_or_else(|| "Narratics Novel".to_string());
        let author = conn
            .get_meta("author")?
            .unwrap_or_else(|| "최용철".to_string());
        let genre = conn
            .get_meta("genre")?
            .unwrap_or_else(|| "장편소설".to_string());
        let synopsis = conn.get_meta("synopsis")?.unwrap_or_default();
        let target_words = conn
            .get_meta("target_words")?
            .and_then(|s| s.parse().ok())
            .unwrap_or(100000);

        let mut binder: Vec<BinderItemDto> = Vec::new();
        let mut scene_dtos: HashMap<String, SceneDto> = HashMap::new();
        let mut total_words = 0;

        for (id, node) in &nodes {
            if id == "root" {
                continue;
            }
            let is_folder = id.starts_with("chap_") || id.starts_with("folder_");
            let word_count = if let Some(engine) = scenes.get(id) {
                let cnt = engine.char_count();
                if !is_folder {
                    total_words += cnt;
                }
                cnt
            } else {
                0
            };
            let status = statuses
                .get(id)
                .cloned()
                .unwrap_or_else(|| "초고".to_string());

            binder.push(BinderItemDto {
                id: id.clone(),
                parent: node.parent.clone(),
                rank: node.rank.clone(),
                title: node.title.clone(),
                is_folder,
                word_count,
                status,
            });
        }

        // Sort binder items by rank & title
        binder.sort_by(|a, b| a.rank.cmp(&b.rank).then_with(|| a.title.cmp(&b.title)));

        for (id, engine) in scenes.iter() {
            let title = titles.get(id).cloned().unwrap_or_else(|| id.clone());
            let text = engine.get_text();
            let count = engine.char_count();
            let status = statuses
                .get(id)
                .cloned()
                .unwrap_or_else(|| "초고".to_string());
            scene_dtos.insert(
                id.clone(),
                SceneDto {
                    id: id.clone(),
                    title,
                    text,
                    word_count: count,
                    status,
                    updated_at: 0,
                },
            );
        }

        let lore = conn.list_lore()?;
        let snapshots = conn.list_snapshots(None)?;
        let active_scene_id = binder
            .iter()
            .find(|b| !b.is_folder)
            .map(|b| b.id.clone())
            .unwrap_or_else(|| "scene_1".to_string());

        Ok(ProjectStateSnapshot {
            project_id,
            title,
            author,
            genre,
            synopsis,
            target_words,
            total_words,
            binder,
            active_scene_id,
            scenes: scene_dtos,
            lore,
            snapshots,
        })
    }

    pub async fn export_manuscript(
        &self,
        format: &str,
    ) -> Result<(String, String, Vec<u8>), Box<dyn std::error::Error + Send + Sync>> {
        let snap = self.get_snapshot().await?;
        let safe_title = snap.title.replace(['/', '\\'], "_");

        let mut md_content = String::new();
        let mut txt_content = String::new();
        let mut typ_body = String::new();

        md_content.push_str(&format!(
            "# {}\n\n> 저자: {}\n> 장르: {}\n\n",
            snap.title, snap.author, snap.genre
        ));
        if !snap.synopsis.is_empty() {
            md_content.push_str(&format!("**시놉시스**\n{}\n\n---\n\n", snap.synopsis));
        }

        txt_content.push_str(&format!("{}\n저자: {}\n\n", snap.title, snap.author));

        for item in &snap.binder {
            if item.is_folder {
                md_content.push_str(&format!("## {}\n\n", item.title));
                txt_content.push_str(&format!("\n\n[ {} ]\n\n", item.title));
                typ_body.push_str(&format!("= {}\n\n", escape_typst_content(&item.title)));
            } else {
                md_content.push_str(&format!("### {}\n\n", item.title));
                txt_content.push_str(&format!("== {} ==\n\n", item.title));
                typ_body.push_str(&format!("== {}\n\n", escape_typst_content(&item.title)));

                if let Some(sc) = snap.scenes.get(&item.id) {
                    md_content.push_str(&sc.text);
                    md_content.push_str("\n\n");

                    txt_content.push_str(&sc.text);
                    txt_content.push_str("\n\n");

                    typ_body.push_str(&escape_typst_content(&sc.text));
                    typ_body.push_str("\n\n");
                }
            }
        }

        match format {
            "txt" | "text" => {
                let filename = format!("{}.txt", safe_title);
                Ok((filename, "text/plain; charset=utf-8".to_string(), txt_content.into_bytes()))
            }
            "typst" | "typ" => {
                let full_typ = render_typst_document(&snap.title, &snap.author, &snap.synopsis, &typ_body);
                let filename = format!("{}.typ", safe_title);
                Ok((filename, "text/plain; charset=utf-8".to_string(), full_typ.into_bytes()))
            }
            "pdf" => {
                let full_typ = render_typst_document(&snap.title, &snap.author, &snap.synopsis, &typ_body);
                let id = format!("{:x}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_nanos());
                let in_path = format!("/tmp/narratics_export_{}.typ", id);
                let out_path = format!("/tmp/narratics_export_{}.pdf", id);

                std::fs::write(&in_path, full_typ)?;

                let typst_bin = std::env::var("TYPST_BIN").unwrap_or_else(|_| "typst".to_string());
                let status = tokio::process::Command::new(typst_bin)
                    .arg("compile")
                    .arg(&in_path)
                    .arg(&out_path)
                    .status()
                    .await?;

                if !status.success() {
                    let _ = std::fs::remove_file(&in_path);
                    return Err("Typst PDF 조판 컴파일에 실패했습니다.".into());
                }

                let pdf_bytes = std::fs::read(&out_path)?;
                let _ = std::fs::remove_file(&in_path);
                let _ = std::fs::remove_file(&out_path);

                let filename = format!("{}.pdf", safe_title);
                Ok((filename, "application/pdf".to_string(), pdf_bytes))
            }
            _ /* markdown */ => {
                let filename = format!("{}.md", safe_title);
                Ok((filename, "text/markdown; charset=utf-8".to_string(), md_content.into_bytes()))
            }
        }
    }
}

fn escape_typst_str(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn escape_typst_content(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('#', "\\#")
        .replace('$', "\\$")
        .replace('@', "\\@")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('*', "\\*")
        .replace('_', "\\_")
        .replace('`', "\\`")
        .replace('<', "\\<")
        .replace('>', "\\>")
}

fn render_typst_document(title: &str, author: &str, synopsis: &str, body: &str) -> String {
    let syn_block = if synopsis.trim().is_empty() {
        String::new()
    } else {
        format!(
            r#"
#v(2em)
#align(center)[#text(size: 13pt, weight: "bold")[작품 개요]]
#v(1.5em)
#align(left)[#text(size: 9.5pt, fill: luma(80))[{}]]
#pagebreak()
"#,
            escape_typst_content(synopsis)
        )
    };

    format!(
        r#"// Narratics 서사 집필 스튜디오 - Typst 조판 규격서
#set document(title: "{}", author: "{}")
#set page(
  paper: "a5",
  margin: (inside: 20mm, outside: 15mm, top: 22mm, bottom: 20mm),
  header: context {{
    if counter(page).get().first() > 2 [
      #set text(size: 8pt, fill: luma(120), font: "Pretendard")
      #grid(
        columns: (1fr, 1fr),
        align(left)[{}],
        align(right)[#counter(page).display()]
      )
      #line(length: 100%, stroke: 0.4pt + luma(200))
    ]
  }}
)

#set text(font: ("Pretendard", "Noto Sans CJK KR"), size: 10pt, lang: "ko")
#set par(justify: true, leading: 0.95em, first-line-indent: 1.2em)

#show heading.where(level: 1): it => {{
  pagebreak(weak: true)
  v(3em)
  text(weight: "bold", size: 16pt)[#it.body]
  v(1.5em)
}}

#show heading.where(level: 2): it => {{
  v(1.5em)
  text(weight: "bold", size: 12pt, fill: luma(60))[#it.body]
  v(0.8em)
}}

// 표지 (Cover)
#align(center + horizon)[
  #text(size: 26pt, weight: "bold")[{}]
  #v(1.5em)
  #text(size: 13pt, fill: luma(70))[{}]
  #v(3em)
  #text(size: 9pt, fill: luma(140))[Narratics Publication Engine]
]
#pagebreak()

{}

// 본문 원고
{}
"#,
        escape_typst_str(title),
        escape_typst_str(author),
        escape_typst_content(title),
        escape_typst_content(title),
        escape_typst_content(author),
        syn_block,
        body
    )
}
