use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

pub type ReplicaId = u32;
pub type LamportTime = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct OpId {
    pub lamport: LamportTime,
    pub replica_id: ReplicaId,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MoveOp {
    pub id: OpId,
    pub child: String,
    pub parent: String,
    pub rank: String,
    pub title: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id: String,
    pub parent: Option<String>,
    pub rank: String,
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct TreeCRDT {
    pub replica_id: ReplicaId,
    pub lamport: LamportTime,
    pub ops: BTreeSet<MoveOp>,
    pub root_id: String,
}

impl TreeCRDT {
    pub fn new(replica_id: ReplicaId, root_id: impl Into<String>) -> Self {
        Self {
            replica_id,
            lamport: 0,
            ops: BTreeSet::new(),
            root_id: root_id.into(),
        }
    }

    /// Generates and applies a move operation locally
    pub fn move_node(
        &mut self,
        child: impl Into<String>,
        parent: impl Into<String>,
        rank: impl Into<String>,
        title: impl Into<String>,
    ) -> MoveOp {
        self.lamport += 1;
        let op = MoveOp {
            id: OpId {
                lamport: self.lamport,
                replica_id: self.replica_id,
            },
            child: child.into(),
            parent: parent.into(),
            rank: rank.into(),
            title: title.into(),
        };
        self.ops.insert(op.clone());
        op
    }

    /// Integrates remote operations and updates local Lamport clock
    pub fn integrate(&mut self, op: MoveOp) {
        if op.id.lamport > self.lamport {
            self.lamport = op.id.lamport;
        }
        self.ops.insert(op);
    }

    /// Materializes the tree by replaying operations in deterministic total order
    /// with cycle detection (Kleppmann Tree Move CRDT algorithm).
    /// Prevents infinite cycles and enforces acyclic tree invariant.
    pub fn materialize(&self) -> (HashMap<String, NodeInfo>, Vec<MoveOp>) {
        let mut nodes: HashMap<String, NodeInfo> = HashMap::new();
        let mut parent_map: HashMap<String, String> = HashMap::new();
        let mut rejected_ops: Vec<MoveOp> = Vec::new();

        // Root node
        nodes.insert(
            self.root_id.clone(),
            NodeInfo {
                id: self.root_id.clone(),
                parent: None,
                rank: "0".to_string(),
                title: "Root".to_string(),
            },
        );

        for op in &self.ops {
            // Rule 1: Root cannot be moved
            if op.child == self.root_id {
                rejected_ops.push(op.clone());
                continue;
            }

            // Rule 2: Cannot be own parent
            if op.child == op.parent {
                rejected_ops.push(op.clone());
                continue;
            }

            // Rule 3: Cycle detection
            // Check if `child` is an ancestor of `parent` in the current materialized tree
            if self.is_ancestor(&parent_map, &op.child, &op.parent) {
                // Cycle detected! Reject this move to maintain acyclic tree invariant
                rejected_ops.push(op.clone());
                continue;
            }

            // Valid move: apply it
            parent_map.insert(op.child.clone(), op.parent.clone());
            nodes.insert(
                op.child.clone(),
                NodeInfo {
                    id: op.child.clone(),
                    parent: Some(op.parent.clone()),
                    rank: op.rank.clone(),
                    title: op.title.clone(),
                },
            );
        }

        (nodes, rejected_ops)
    }

    /// Checks if a node reaches root without encountering 'trash'
    pub fn is_under_root(&self, node_id: &str, parent_map: &HashMap<String, String>) -> bool {
        let mut curr = node_id;
        let mut visited = std::collections::HashSet::new();
        while let Some(parent) = parent_map.get(curr) {
            if parent == "trash" || parent.is_empty() {
                return false;
            }
            if parent == &self.root_id {
                return true;
            }
            if !visited.insert(parent.as_str()) {
                return false;
            }
            curr = parent;
        }
        false
    }

    /// Materializes only active nodes under root, excluding trashed subtrees
    pub fn materialize_active(&self) -> (HashMap<String, NodeInfo>, Vec<MoveOp>) {
        let mut parent_map: HashMap<String, String> = HashMap::new();
        let mut all_nodes = HashMap::new();
        let mut rejected_ops = Vec::new();

        all_nodes.insert(
            self.root_id.clone(),
            NodeInfo {
                id: self.root_id.clone(),
                parent: None,
                rank: "0".to_string(),
                title: "Root".to_string(),
            },
        );

        for op in &self.ops {
            if op.child == self.root_id || op.child == op.parent {
                rejected_ops.push(op.clone());
                continue;
            }
            if self.is_ancestor(&parent_map, &op.child, &op.parent) {
                rejected_ops.push(op.clone());
                continue;
            }
            parent_map.insert(op.child.clone(), op.parent.clone());
            all_nodes.insert(
                op.child.clone(),
                NodeInfo {
                    id: op.child.clone(),
                    parent: Some(op.parent.clone()),
                    rank: op.rank.clone(),
                    title: op.title.clone(),
                },
            );
        }

        let mut active_nodes = HashMap::new();
        for (id, node) in all_nodes {
            if id == self.root_id || self.is_under_root(&id, &parent_map) {
                active_nodes.insert(id, node);
            }
        }

        (active_nodes, rejected_ops)
    }

    fn is_ancestor(
        &self,
        parent_map: &HashMap<String, String>,
        ancestor: &str,
        start: &str,
    ) -> bool {
        if ancestor == start {
            return true;
        }
        let mut curr = start;
        let mut visited = std::collections::HashSet::new();
        visited.insert(start);

        // Bounded depth limit prevents infinite loops even on poisoned inputs
        let mut steps = 0;
        const MAX_STEPS: usize = 10_000;

        while let Some(parent) = parent_map.get(curr) {
            steps += 1;
            if steps > MAX_STEPS || !visited.insert(parent.as_str()) {
                // Cycle detected in existing parent map or max depth exceeded
                return true;
            }
            if parent == ancestor {
                return true;
            }
            curr = parent;
        }
        false
    }

    /// Formats the tree as a human-readable ASCII outline
    pub fn format_tree(&self) -> String {
        let (nodes, _) = self.materialize();
        let mut children_map: HashMap<Option<String>, Vec<NodeInfo>> = HashMap::new();

        for node in nodes.values() {
            children_map
                .entry(node.parent.clone())
                .or_default()
                .push(node.clone());
        }

        // Sort children by rank
        for list in children_map.values_mut() {
            list.sort_by(|a, b| a.rank.cmp(&b.rank));
        }

        let mut out = String::new();
        self.render_node(&mut out, &children_map, &self.root_id, 0);
        out
    }

    fn render_node(
        &self,
        out: &mut String,
        children_map: &HashMap<Option<String>, Vec<NodeInfo>>,
        node_id: &str,
        depth: usize,
    ) {
        let indent = "  ".repeat(depth);
        out.push_str(&format!("{}- {}\n", indent, node_id));

        if let Some(children) = children_map.get(&Some(node_id.to_string())) {
            for child in children {
                self.render_node(out, children_map, &child.id, depth + 1);
            }
        }
    }
}
