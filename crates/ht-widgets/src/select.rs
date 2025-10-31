//! Select/dropdown widget

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Select {
    pub id: String,
    pub label: Option<String>,
    pub options: Vec<SelectOption>,
    pub multiple: bool,
    pub selected: Vec<usize>,
    pub searchable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl Select {
    pub fn new(id: String, options: Vec<SelectOption>) -> Self {
        Self {
            id,
            label: None,
            options,
            multiple: false,
            selected: Vec::new(),
            searchable: false,
        }
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn multiple(mut self) -> Self {
        self.multiple = true;
        self
    }

    pub fn searchable(mut self) -> Self {
        self.searchable = true;
        self
    }

    pub fn select(&mut self, index: usize) {
        if !self.multiple {
            self.selected.clear();
        }
        if !self.selected.contains(&index) {
            self.selected.push(index);
        }
    }

    pub fn get_selected_values(&self) -> Vec<&String> {
        self.selected
            .iter()
            .filter_map(|&i| self.options.get(i).map(|opt| &opt.value))
            .collect()
    }
}

impl SelectOption {
    pub fn new(value: String, label: String) -> Self {
        Self {
            value,
            label,
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}
