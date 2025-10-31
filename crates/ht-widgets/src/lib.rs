//! Widget System for Interactive Terminal Components
//!
//! This crate provides a comprehensive widget system for creating
//! interactive terminal interfaces. LLMs can use these widgets to
//! create forms, buttons, tables, and other interactive elements.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub mod button;
pub mod form;
pub mod table;
pub mod chart;
pub mod progress;
pub mod select;
pub mod tree;
pub mod tabs;
pub mod markdown;

pub use button::Button;
pub use form::{Form, FormField, FormFieldType};
pub use table::{DataTable, TableColumn};
pub use chart::{Chart, ChartType, ChartData};
pub use progress::ProgressBar;
pub use select::{Select, SelectOption};
pub use tree::{TreeView, TreeNode};
pub use tabs::{TabView, Tab};
pub use markdown::MarkdownRenderer;

/// Widget manager that handles all interactive widgets
#[derive(Debug)]
pub struct WidgetManager {
    widgets: HashMap<String, Widget>,
    event_handlers: HashMap<String, EventHandler>,
}

impl WidgetManager {
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            event_handlers: HashMap::new(),
        }
    }

    /// Register a new widget
    pub fn register_widget(&mut self, widget: Widget) -> String {
        let id = widget.id().to_string();
        self.widgets.insert(id.clone(), widget);
        id
    }

    /// Get a widget by ID
    pub fn get_widget(&self, id: &str) -> Option<&Widget> {
        self.widgets.get(id)
    }

    /// Get a mutable widget by ID
    pub fn get_widget_mut(&mut self, id: &str) -> Option<&mut Widget> {
        self.widgets.get_mut(id)
    }

    /// Remove a widget
    pub fn remove_widget(&mut self, id: &str) -> Option<Widget> {
        self.widgets.remove(id)
    }

    /// Register an event handler
    pub fn register_handler(&mut self, widget_id: String, handler: EventHandler) {
        self.event_handlers.insert(widget_id, handler);
    }

    /// Handle a widget event
    pub fn handle_event(&mut self, widget_id: &str, event: WidgetEvent) -> Result<Option<EventResult>> {
        if let Some(handler) = self.event_handlers.get(widget_id) {
            Ok(Some((handler.callback)(event)?))
        } else {
            Ok(None)
        }
    }

    /// Update widget state
    pub fn update_widget(&mut self, id: &str, update: WidgetUpdate) -> Result<()> {
        let widget = self.get_widget_mut(id)
            .ok_or_else(|| anyhow::anyhow!("Widget not found: {}", id))?;

        widget.apply_update(update)?;
        Ok(())
    }

    /// Get all widgets
    pub fn widgets(&self) -> impl Iterator<Item = &Widget> {
        self.widgets.values()
    }
}

/// Base widget trait
pub trait WidgetTrait {
    fn id(&self) -> &str;
    fn widget_type(&self) -> WidgetType;
    fn render(&self) -> String;
    fn handle_input(&mut self, input: &str) -> Result<Option<WidgetEvent>>;
}

/// Widget types enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Widget {
    Button(Button),
    Form(Form),
    Table(DataTable),
    Chart(Chart),
    Progress(ProgressBar),
    Select(Select),
    Tree(TreeView),
    Tabs(TabView),
    Markdown(MarkdownRenderer),
}

impl Widget {
    pub fn id(&self) -> &str {
        match self {
            Widget::Button(w) => &w.id,
            Widget::Form(w) => &w.id,
            Widget::Table(w) => &w.id,
            Widget::Chart(w) => &w.id,
            Widget::Progress(w) => &w.id,
            Widget::Select(w) => &w.id,
            Widget::Tree(w) => &w.id,
            Widget::Tabs(w) => &w.id,
            Widget::Markdown(w) => &w.id,
        }
    }

    pub fn widget_type(&self) -> WidgetType {
        match self {
            Widget::Button(_) => WidgetType::Button,
            Widget::Form(_) => WidgetType::Form,
            Widget::Table(_) => WidgetType::Table,
            Widget::Chart(_) => WidgetType::Chart,
            Widget::Progress(_) => WidgetType::Progress,
            Widget::Select(_) => WidgetType::Select,
            Widget::Tree(_) => WidgetType::Tree,
            Widget::Tabs(_) => WidgetType::Tabs,
            Widget::Markdown(_) => WidgetType::Markdown,
        }
    }

    pub fn apply_update(&mut self, update: WidgetUpdate) -> Result<()> {
        match (self, update) {
            (Widget::Progress(w), WidgetUpdate::SetProgress { value, label }) => {
                w.set_value(value);
                if let Some(label) = label {
                    w.set_label(label);
                }
            }
            (Widget::Table(w), WidgetUpdate::SetTableData { rows }) => {
                w.set_rows(rows);
            }
            (Widget::Markdown(w), WidgetUpdate::SetContent { content }) => {
                w.set_content(content);
            }
            _ => return Err(anyhow::anyhow!("Invalid update for widget type")),
        }
        Ok(())
    }
}

/// Widget type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WidgetType {
    Button,
    Form,
    Table,
    Chart,
    Progress,
    Select,
    Tree,
    Tabs,
    Markdown,
}

/// Widget events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum WidgetEvent {
    Click { widget_id: String, data: Option<serde_json::Value> },
    Submit { widget_id: String, data: serde_json::Value },
    Change { widget_id: String, value: serde_json::Value },
    Select { widget_id: String, selected: String },
    Custom { widget_id: String, event_type: String, data: serde_json::Value },
}

/// Result of handling an event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventResult {
    pub command: Option<String>,
    pub output: Option<String>,
    pub data: Option<serde_json::Value>,
}

/// Event handler
pub struct EventHandler {
    pub callback: Box<dyn Fn(WidgetEvent) -> Result<EventResult> + Send>,
}

impl std::fmt::Debug for EventHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventHandler").finish()
    }
}

/// Widget update commands
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "update")]
pub enum WidgetUpdate {
    SetProgress { value: f32, label: Option<String> },
    SetTableData { rows: Vec<Vec<String>> },
    SetContent { content: String },
    SetChartData { data: Vec<(String, f64)> },
}

/// Callback URL for bidirectional communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackUrl {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
}

impl CallbackUrl {
    pub fn new(url: String) -> Self {
        Self {
            url,
            method: "POST".to_string(),
            headers: HashMap::new(),
        }
    }

    pub fn with_method(mut self, method: String) -> Self {
        self.method = method;
        self
    }

    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_manager() {
        let mut manager = WidgetManager::new();

        let button = Button::new("test-button".to_string(), "Click Me".to_string());
        let id = manager.register_widget(Widget::Button(button));

        assert!(manager.get_widget(&id).is_some());
        assert_eq!(manager.widgets().count(), 1);
    }
}
