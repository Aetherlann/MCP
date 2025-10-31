//! Tree view widget for hierarchical data

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeView {
    pub id: String,
    pub root: TreeNode,
    pub expanded: Vec<String>,
    pub selected: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeNode {
    pub id: String,
    pub label: String,
    pub children: Vec<TreeNode>,
    pub icon: Option<String>,
    pub data: Option<serde_json::Value>,
}

impl TreeView {
    pub fn new(id: String, root: TreeNode) -> Self {
        Self {
            id,
            root,
            expanded: Vec::new(),
            selected: None,
        }
    }

    pub fn expand(&mut self, node_id: String) {
        if !self.expanded.contains(&node_id) {
            self.expanded.push(node_id);
        }
    }

    pub fn collapse(&mut self, node_id: &str) {
        self.expanded.retain(|id| id != node_id);
    }

    pub fn select(&mut self, node_id: String) {
        self.selected = Some(node_id);
    }
}

impl TreeNode {
    pub fn new(id: String, label: String) -> Self {
        Self {
            id,
            label,
            children: Vec::new(),
            icon: None,
            data: None,
        }
    }

    pub fn with_children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }

    pub fn with_icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }
}
