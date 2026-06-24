#![allow(dead_code)]

use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridRowWindow {
    pub start: usize,
    pub end: usize,
    pub total: usize,
}

impl GridRowWindow {
    pub fn new(start: usize, len: usize, total: usize) -> Self {
        let start = start.min(total);
        let end = start.saturating_add(len).min(total);
        Self { start, end, total }
    }

    pub fn range(self) -> Range<usize> {
        self.start..self.end
    }

    pub fn len(self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }
}

pub fn page_row_model(row_model: &[usize], start: usize, len: usize) -> &[usize] {
    let window = GridRowWindow::new(start, len, row_model.len());
    &row_model[window.range()]
}
