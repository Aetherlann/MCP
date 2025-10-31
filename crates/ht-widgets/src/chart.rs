//! Chart widget for data visualization

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    pub id: String,
    pub title: Option<String>,
    pub chart_type: ChartType,
    pub data: ChartData,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType {
    Line,
    Bar,
    Pie,
    Scatter,
    Area,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub datasets: Vec<Dataset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    pub label: String,
    pub data: Vec<f64>,
    pub color: Option<String>,
}

impl Chart {
    pub fn new(id: String, chart_type: ChartType) -> Self {
        Self {
            id,
            title: None,
            chart_type,
            data: ChartData {
                labels: Vec::new(),
                datasets: Vec::new(),
            },
            width: None,
            height: None,
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_data(mut self, data: ChartData) -> Self {
        self.data = data;
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
}
