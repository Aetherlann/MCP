//! Button widget

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    pub id: String,
    pub label: String,
    pub style: ButtonStyle,
    pub enabled: bool,
    pub callback: Option<String>, // Callback command or URL
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
}

impl Button {
    pub fn new(id: String, label: String) -> Self {
        Self {
            id,
            label,
            style: ButtonStyle::Primary,
            enabled: true,
            callback: None,
        }
    }

    pub fn with_style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_callback(mut self, callback: String) -> Self {
        self.callback = Some(callback);
        self
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }
}
