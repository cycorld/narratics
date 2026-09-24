use crate::tree_crdt::{MoveOp, OpId, TreeCRDT};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContainerError {
    #[error("SQLite database error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid container file format")]
    InvalidFormat,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectMeta {
    pub title: String,
    pub author: String,
    pub schema_version: u32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SceneRecord {
    pub id: String,
    pub ydoc_state: Vec<u8>,
    pub text_cache: String,
    pub word_count: usize,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoreRecord {
    pub id: String,
    pub category: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub content: String,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SnapshotRecord {
    pub id: String,
    pub target_id: String,
    pub label: String,
    pub content: String,
    pub word_count: usize,
    pub created_at: i64,
}

pub struct NarrContainer {
    conn: Connection,
}

impl NarrContainer {
    /// Opens or creates a `.narr` SQLite container with WAL mode
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, ContainerError> {
        let conn = Connection::open(path)?;
        Self::init_connection(conn)
    }

    /// Opens an in-memory `.narr` container (useful for tests and temporary caches)
    pub fn open_in_memory() -> Result<Self, ContainerError> {
        let conn = Connection::open_in_memory()?;
        Self::init_connection(conn)
    }

    fn init_connection(conn: Connection) -> Result<Self, ContainerError> {
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.pragma_update(None, "busy_timeout", 5000)?;

        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS binder_ops (
                op_id TEXT PRIMARY KEY,
                lamport INTEGER NOT NULL,
                replica_id INTEGER NOT NULL,
                child TEXT NOT NULL,
                parent TEXT NOT NULL,
                rank TEXT NOT NULL,
                title TEXT NOT NULL,
                created_at INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_binder_ops_lamport ON binder_ops(lamport, replica_id);

            CREATE TABLE IF NOT EXISTS scenes (
                id TEXT PRIMARY KEY,
                ydoc_state BLOB NOT NULL,
                text_cache TEXT NOT NULL,
                word_count INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS lore (
                id TEXT PRIMARY KEY,
                category TEXT NOT NULL,
                name TEXT NOT NULL,
                aliases TEXT NOT NULL,
                content TEXT NOT NULL,
                updated_at INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS snapshots (
                id TEXT PRIMARY KEY,
                target_id TEXT NOT NULL,
                label TEXT NOT NULL,
                blob BLOB NOT NULL,
                created_at INTEGER NOT NULL
            );
            "#,
        )?;

        // Initialize default schema version if missing
        let exists: Option<String> = conn
            .query_row(
                "SELECT value FROM meta WHERE key = 'schema_version'",
                [],
                |row| row.get(0),
            )
            .optional()?;

        if exists.is_none() {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;

            conn.execute(
                "INSERT INTO meta (key, value) VALUES ('schema_version', '1')",
                [],
            )?;
            conn.execute(
                "INSERT INTO meta (key, value) VALUES ('created_at', ?1)",
                params![now],
            )?;
            conn.execute(
                "INSERT INTO meta (key, value) VALUES ('title', 'Untitled Novel')",
                [],
            )?;
        }

        Ok(Self { conn })
    }

    pub fn set_meta(&self, key: &str, value: &str) -> Result<(), ContainerError> {
        self.conn.execute(
            "INSERT INTO meta (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get_meta(&self, key: &str) -> Result<Option<String>, ContainerError> {
        let val: Option<String> = self
            .conn
            .query_row(
                "SELECT value FROM meta WHERE key = ?1",
                params![key],
                |row| row.get(0),
            )
            .optional()?;
        Ok(val)
    }

    pub fn save_tree_ops(&mut self, ops: &[MoveOp]) -> Result<(), ContainerError> {
        let tx = self.conn.transaction()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        {
            let mut stmt = tx.prepare_cached(
                r#"
                INSERT INTO binder_ops (op_id, lamport, replica_id, child, parent, rank, title, created_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                ON CONFLICT(op_id) DO NOTHING
                "#,
            )?;

            for op in ops {
                let op_id_str = format!("{}:{}", op.id.lamport, op.id.replica_id);
                stmt.execute(params![
                    op_id_str,
                    op.id.lamport as i64,
                    op.id.replica_id as i64,
                    op.child,
                    op.parent,
                    op.rank,
                    op.title,
                    now
                ])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn load_tree_crdt(
        &self,
        replica_id: u32,
        root_id: impl Into<String>,
    ) -> Result<TreeCRDT, ContainerError> {
        let root = root_id.into();
        let mut tree = TreeCRDT::new(replica_id, &root);

        let mut stmt = self.conn.prepare(
            "SELECT lamport, replica_id, child, parent, rank, title FROM binder_ops ORDER BY lamport ASC, replica_id ASC",
        )?;

        let rows = stmt.query_map([], |row| {
            let lamport: i64 = row.get(0)?;
            let replica_id: i64 = row.get(1)?;
            let child: String = row.get(2)?;
            let parent: String = row.get(3)?;
            let rank: String = row.get(4)?;
            let title: String = row.get(5)?;

            Ok(MoveOp {
                id: OpId {
                    lamport: lamport as u64,
                    replica_id: replica_id as u32,
                },
                child,
                parent,
                rank,
                title,
            })
        })?;

        for op in rows {
            tree.integrate(op?);
        }

        Ok(tree)
    }

    pub fn save_scene(
        &self,
        scene_id: &str,
        ydoc_state: &[u8],
        text_cache: &str,
        word_count: usize,
    ) -> Result<(), ContainerError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        self.conn.execute(
            r#"
            INSERT INTO scenes (id, ydoc_state, text_cache, word_count, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(id) DO UPDATE SET
                ydoc_state = ?2,
                text_cache = ?3,
                word_count = ?4,
                updated_at = ?5
            "#,
            params![scene_id, ydoc_state, text_cache, word_count as i64, now],
        )?;
        Ok(())
    }

    pub fn load_scene(&self, scene_id: &str) -> Result<Option<SceneRecord>, ContainerError> {
        let record: Option<SceneRecord> = self
            .conn
            .query_row(
                "SELECT id, ydoc_state, text_cache, word_count, updated_at FROM scenes WHERE id = ?1",
                params![scene_id],
                |row| {
                    let word_count: i64 = row.get(3)?;
                    Ok(SceneRecord {
                        id: row.get(0)?,
                        ydoc_state: row.get(1)?,
                        text_cache: row.get(2)?,
                        word_count: word_count as usize,
                        updated_at: row.get(4)?,
                    })
                },
            )
            .optional()?;
        Ok(record)
    }

    pub fn list_scenes(&self) -> Result<Vec<SceneRecord>, ContainerError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, ydoc_state, text_cache, word_count, updated_at FROM scenes ORDER BY id ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            let word_count: i64 = row.get(3)?;
            Ok(SceneRecord {
                id: row.get(0)?,
                ydoc_state: row.get(1)?,
                text_cache: row.get(2)?,
                word_count: word_count as usize,
                updated_at: row.get(4)?,
            })
        })?;

        let mut scenes = Vec::new();
        for r in rows {
            scenes.push(r?);
        }
        Ok(scenes)
    }

    pub fn save_lore(&self, lore: &LoreRecord) -> Result<(), ContainerError> {
        let aliases_json = serde_json::to_string(&lore.aliases)?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        self.conn.execute(
            r#"
            INSERT INTO lore (id, category, name, aliases, content, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ON CONFLICT(id) DO UPDATE SET
                category = ?2,
                name = ?3,
                aliases = ?4,
                content = ?5,
                updated_at = ?6
            "#,
            params![
                lore.id,
                lore.category,
                lore.name,
                aliases_json,
                lore.content,
                now
            ],
        )?;
        Ok(())
    }

    pub fn list_lore(&self) -> Result<Vec<LoreRecord>, ContainerError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, category, name, aliases, content, updated_at FROM lore ORDER BY category ASC, name ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            let aliases_json: String = row.get(3)?;
            let aliases: Vec<String> = serde_json::from_str(&aliases_json).unwrap_or_default();
            Ok(LoreRecord {
                id: row.get(0)?,
                category: row.get(1)?,
                name: row.get(2)?,
                aliases,
                content: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn delete_lore(&self, id: &str) -> Result<(), ContainerError> {
        self.conn.execute("DELETE FROM lore WHERE id = ?1", rusqlite::params![id])?;
        Ok(())
    }

    pub fn delete_scene(&self, id: &str) -> Result<(), ContainerError> {
        self.conn.execute("DELETE FROM scenes WHERE id = ?1", rusqlite::params![id])?;
        self.conn.execute("DELETE FROM snapshots WHERE target_id = ?1", rusqlite::params![id])?;
        Ok(())
    }

    pub fn save_snapshot(&self, snapshot: &SnapshotRecord) -> Result<(), ContainerError> {
        let now = if snapshot.created_at > 0 {
            snapshot.created_at
        } else {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64
        };

        self.conn.execute(
            r#"
            INSERT INTO snapshots (id, target_id, label, blob, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(id) DO UPDATE SET
                label = ?3,
                blob = ?4,
                created_at = ?5
            "#,
            params![
                snapshot.id,
                snapshot.target_id,
                snapshot.label,
                snapshot.content.as_bytes(),
                now
            ],
        )?;
        Ok(())
    }

    pub fn list_snapshots(&self, target_id: Option<&str>) -> Result<Vec<SnapshotRecord>, ContainerError> {
        let mut list = Vec::new();
        if let Some(target) = target_id {
            let mut stmt = self.conn.prepare(
                "SELECT id, target_id, label, blob, created_at FROM snapshots WHERE target_id = ?1 ORDER BY created_at DESC",
            )?;
            let rows = stmt.query_map(params![target], |row| {
                let bytes: Vec<u8> = row.get(3)?;
                let content = String::from_utf8_lossy(&bytes).to_string();
                let word_count = content.chars().count();
                Ok(SnapshotRecord {
                    id: row.get(0)?,
                    target_id: row.get(1)?,
                    label: row.get(2)?,
                    content,
                    word_count,
                    created_at: row.get(4)?,
                })
            })?;
            for r in rows {
                list.push(r?);
            }
        } else {
            let mut stmt = self.conn.prepare(
                "SELECT id, target_id, label, blob, created_at FROM snapshots ORDER BY created_at DESC",
            )?;
            let rows = stmt.query_map([], |row| {
                let bytes: Vec<u8> = row.get(3)?;
                let content = String::from_utf8_lossy(&bytes).to_string();
                let word_count = content.chars().count();
                Ok(SnapshotRecord {
                    id: row.get(0)?,
                    target_id: row.get(1)?,
                    label: row.get(2)?,
                    content,
                    word_count,
                    created_at: row.get(4)?,
                })
            })?;
            for r in rows {
                list.push(r?);
            }
        }
        Ok(list)
    }

    /// Explicit WAL checkpoint to ensure all data is flushed into the main `.narr` database file
    pub fn checkpoint(&self) -> Result<(), ContainerError> {
        self.conn.pragma_update(None, "wal_checkpoint", "PASSIVE")?;
        Ok(())
    }
}
