//! Progress bar widget

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressBar {
    pub id: String,
    pub value: f32, // 0.0 to 1.0
    pub label: Option<String>,
    pub show_percentage: bool,
    pub indeterminate: bool,
    pub style: ProgressStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgressStyle {
    Bar,
    Circle,
    Spinner,
}

impl ProgressBar {
    pub fn new(id: String) -> Self {
        Self {
            id,
            value: 0.0,
            label: None,
            show_percentage: true,
            indeterminate: false,
            style: ProgressStyle::Bar,
        }
    }

    pub fn with_value(mut self, value: f32) -> Self {
        self.value = value.clamp(0.0, 1.0);
        self
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn indeterminate(mut self) -> Self {
        self.indeterminate = true;
        self
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(0.0, 1.0);
    }

    pub fn set_label(&mut self, label: String) {
        self.label = Some(label);
    }

    pub fn percentage(&self) -> f32 {
        self.value * 100.0
    }
}
