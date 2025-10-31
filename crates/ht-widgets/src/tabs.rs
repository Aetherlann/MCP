//! Tab view widget

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabView {
    pub id: String,
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub id: String,
    pub label: String,
    pub content: String,
    pub closable: bool,
    pub disabled: bool,
}

impl TabView {
    pub fn new(id: String) -> Self {
        Self {
            id,
            tabs: Vec::new(),
            active_tab: 0,
        }
    }

    pub fn add_tab(mut self, tab: Tab) -> Self {
        self.tabs.push(tab);
        self
    }

    pub fn set_active(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab = index;
        }
    }

    pub fn close_tab(&mut self, index: usize) {
        if index < self.tabs.len() && self.tabs[index].closable {
            self.tabs.remove(index);
            if self.active_tab >= self.tabs.len() && self.active_tab > 0 {
                self.active_tab -= 1;
            }
        }
    }

    pub fn get_active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active_tab)
    }
}

impl Tab {
    pub fn new(id: String, label: String, content: String) -> Self {
        Self {
            id,
            label,
            content,
            closable: false,
            disabled: false,
        }
    }

    pub fn closable(mut self) -> Self {
        self.closable = true;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}
