//! Form widget for collecting user input

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Form {
    pub id: String,
    pub title: Option<String>,
    pub fields: Vec<FormField>,
    pub submit_label: String,
    pub cancel_label: Option<String>,
    pub callback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    pub name: String,
    pub label: String,
    pub field_type: FormFieldType,
    pub required: bool,
    pub default_value: Option<String>,
    pub placeholder: Option<String>,
    pub validation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FormFieldType {
    Text { multiline: bool },
    Number { min: Option<f64>, max: Option<f64>, step: Option<f64> },
    Email,
    Password,
    Checkbox,
    Radio { options: Vec<String> },
    Select { options: Vec<String>, multiple: bool },
    Date,
    Time,
    File { accept: Option<String> },
}

impl Form {
    pub fn new(id: String) -> Self {
        Self {
            id,
            title: None,
            fields: Vec::new(),
            submit_label: "Submit".to_string(),
            cancel_label: None,
            callback: None,
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn add_field(mut self, field: FormField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn with_callback(mut self, callback: String) -> Self {
        self.callback = Some(callback);
        self
    }
}

impl FormField {
    pub fn text(name: String, label: String) -> Self {
        Self {
            name,
            label,
            field_type: FormFieldType::Text { multiline: false },
            required: false,
            default_value: None,
            placeholder: None,
            validation: None,
        }
    }

    pub fn number(name: String, label: String) -> Self {
        Self {
            name,
            label,
            field_type: FormFieldType::Number { min: None, max: None, step: None },
            required: false,
            default_value: None,
            placeholder: None,
            validation: None,
        }
    }

    pub fn select(name: String, label: String, options: Vec<String>) -> Self {
        Self {
            name,
            label,
            field_type: FormFieldType::Select { options, multiple: false },
            required: false,
            default_value: None,
            placeholder: None,
            validation: None,
        }
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn with_default(mut self, value: String) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn with_placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = Some(placeholder);
        self
    }
}
