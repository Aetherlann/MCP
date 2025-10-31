//! Data table widget with sorting and filtering

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTable {
    pub id: String,
    pub columns: Vec<TableColumn>,
    pub rows: Vec<Vec<String>>,
    pub sortable: bool,
    pub filterable: bool,
    pub paginated: bool,
    pub page_size: usize,
    pub current_page: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableColumn {
    pub name: String,
    pub label: String,
    pub width: Option<f32>,
    pub align: TextAlign,
    pub sortable: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl DataTable {
    pub fn new(id: String, columns: Vec<TableColumn>) -> Self {
        Self {
            id,
            columns,
            rows: Vec::new(),
            sortable: true,
            filterable: true,
            paginated: false,
            page_size: 20,
            current_page: 0,
        }
    }

    pub fn with_rows(mut self, rows: Vec<Vec<String>>) -> Self {
        self.rows = rows;
        self
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    pub fn set_rows(&mut self, rows: Vec<Vec<String>>) {
        self.rows = rows;
    }

    pub fn paginated(mut self, page_size: usize) -> Self {
        self.paginated = true;
        self.page_size = page_size;
        self
    }

    pub fn total_pages(&self) -> usize {
        if self.page_size == 0 {
            return 1;
        }
        (self.rows.len() + self.page_size - 1) / self.page_size
    }
}

impl TableColumn {
    pub fn new(name: String, label: String) -> Self {
        Self {
            name,
            label,
            width: None,
            align: TextAlign::Left,
            sortable: true,
        }
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }
}
