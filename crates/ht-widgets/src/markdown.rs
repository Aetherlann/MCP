//! Markdown renderer widget

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownRenderer {
    pub id: String,
    pub content: String,
    pub theme: MarkdownTheme,
    pub syntax_highlighting: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarkdownTheme {
    Light,
    Dark,
    GitHub,
    Nord,
}

impl MarkdownRenderer {
    pub fn new(id: String, content: String) -> Self {
        Self {
            id,
            content,
            theme: MarkdownTheme::Dark,
            syntax_highlighting: true,
        }
    }

    pub fn with_theme(mut self, theme: MarkdownTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn disable_syntax_highlighting(mut self) -> Self {
        self.syntax_highlighting = false;
        self
    }
}
