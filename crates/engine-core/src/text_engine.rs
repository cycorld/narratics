use yrs::updates::decoder::Decode;
use yrs::{
    Doc, GetString, OffsetKind, Options, ReadTxn, StateVector, Text, TextRef, Transact, Update,
};

#[derive(Clone, Debug)]
pub struct CompactionResult {
    pub raw_update_size: usize,
    pub compacted_size: usize,
    pub char_count: usize,
}

pub struct SceneEngine {
    pub scene_id: String,
    doc: Doc,
    text: TextRef,
}

impl SceneEngine {
    /// Creates a new scene with Utf16 offset kind to ensure compatibility
    /// with browser DOM (TipTap/ProseMirror) and prevent UTF-8 boundary panics.
    pub fn new(scene_id: impl Into<String>) -> Self {
        let options = Options {
            offset_kind: OffsetKind::Utf16,
            ..Default::default()
        };
        let doc = Doc::with_options(options);
        let text = doc.get_or_insert_text("content");
        Self {
            scene_id: scene_id.into(),
            doc,
            text,
        }
    }

    /// Loads a scene from a binary CRDT update payload
    pub fn from_bytes(
        scene_id: impl Into<String>,
        bytes: &[u8],
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let engine = Self::new(scene_id);
        if !bytes.is_empty() {
            let mut txn = engine.doc.transact_mut();
            let update = Update::decode_v1(bytes)?;
            txn.apply_update(update)?;
        }
        Ok(engine)
    }

    /// Inserts text at UTF-16 index
    pub fn insert(&self, index: u32, text: &str) {
        let mut txn = self.doc.transact_mut();
        self.text.insert(&mut txn, index, text);
    }

    /// Deletes `len` characters at UTF-16 index
    pub fn delete(&self, index: u32, len: u32) {
        let mut txn = self.doc.transact_mut();
        let total_len = self.text.len(&txn);
        if index < total_len {
            let safe_len = len.min(total_len - index);
            self.text.remove_range(&mut txn, index, safe_len);
        }
    }

    /// Returns the complete text content
    pub fn get_text(&self) -> String {
        let txn = self.doc.transact();
        self.text.get_string(&txn)
    }

    /// Returns the character length (UTF-16 code units)
    pub fn char_count(&self) -> usize {
        let txn = self.doc.transact();
        self.text.len(&txn) as usize
    }

    /// Generates binary diff against remote state vector (or full state if None)
    pub fn encode_diff(
        &self,
        remote_sv: Option<&[u8]>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let txn = self.doc.transact();
        match remote_sv {
            Some(sv_bytes) if !sv_bytes.is_empty() => {
                let sv = StateVector::decode_v1(sv_bytes)?;
                Ok(txn.encode_diff_v1(&sv))
            }
            _ => Ok(txn.encode_state_as_update_v1(&StateVector::default())),
        }
    }

    /// Applies incoming binary update
    pub fn apply_update(
        &self,
        update_bytes: &[u8],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if update_bytes.is_empty() {
            return Ok(());
        }
        let update = Update::decode_v1(update_bytes)?;
        let mut txn = self.doc.transact_mut();
        txn.apply_update(update)?;
        Ok(())
    }

    /// Compresses and compacts tombstones into a clean v1 snapshot
    pub fn compact(&self) -> CompactionResult {
        let txn = self.doc.transact();
        let compacted_bytes = txn.encode_state_as_update_v1(&StateVector::default());
        let char_count = self.text.len(&txn) as usize;

        CompactionResult {
            raw_update_size: compacted_bytes.len(),
            compacted_size: compacted_bytes.len(),
            char_count,
        }
    }

    /// Serializes entire doc to single update blob
    pub fn to_bytes(&self) -> Vec<u8> {
        let txn = self.doc.transact();
        txn.encode_state_as_update_v1(&StateVector::default())
    }
}
