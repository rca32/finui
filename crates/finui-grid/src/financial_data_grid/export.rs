#![allow(dead_code)]

use super::cell::GridCellValue;
use super::column::GridColumnDef;
use super::ids::{GridColumnId, GridRowId};
use super::source::GridRowSource;
use super::state::GridState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridExportFormat {
    Tsv,
    Csv,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridExportValueMode {
    Display,
    Raw,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridExportOptions {
    pub format: GridExportFormat,
    pub value_mode: GridExportValueMode,
    pub include_hidden_columns: bool,
    pub include_header: bool,
}

impl Default for GridExportOptions {
    fn default() -> Self {
        Self {
            format: GridExportFormat::Tsv,
            value_mode: GridExportValueMode::Display,
            include_hidden_columns: false,
            include_header: true,
        }
    }
}

pub fn export_rows(
    source: &dyn GridRowSource,
    columns: &[GridColumnDef],
    state: &GridState,
    row_model: &[usize],
    options: &GridExportOptions,
) -> String {
    let export_columns = export_column_ids(columns, state, options.include_hidden_columns);
    match options.format {
        GridExportFormat::Tsv => {
            export_delimited(source, columns, row_model, &export_columns, options, '\t')
        }
        GridExportFormat::Csv => {
            export_delimited(source, columns, row_model, &export_columns, options, ',')
        }
        GridExportFormat::Json => export_json(source, columns, row_model, &export_columns, options),
    }
}

pub fn export_selected_row(
    source: &dyn GridRowSource,
    columns: &[GridColumnDef],
    state: &GridState,
    row_model: &[usize],
    options: &GridExportOptions,
) -> Option<String> {
    let selected = state.selection.selected_row.as_ref()?;
    let row_index = row_model
        .iter()
        .copied()
        .find(|row| source.row_id(*row) == *selected)?;
    Some(export_rows(source, columns, state, &[row_index], options))
}

fn export_column_ids(
    columns: &[GridColumnDef],
    state: &GridState,
    include_hidden: bool,
) -> Vec<GridColumnId> {
    let mut ids = if include_hidden {
        state.column_order.clone()
    } else {
        state.visible_column_ids()
    };
    if ids.is_empty() {
        ids = columns.iter().map(|column| column.id.clone()).collect();
    }
    ids.retain(|id| columns.iter().any(|column| column.id == *id));
    ids
}

fn export_delimited(
    source: &dyn GridRowSource,
    columns: &[GridColumnDef],
    row_model: &[usize],
    column_ids: &[GridColumnId],
    options: &GridExportOptions,
    delimiter: char,
) -> String {
    let mut lines = Vec::new();
    if options.include_header {
        lines.push(
            column_ids
                .iter()
                .filter_map(|id| columns.iter().find(|column| column.id == *id))
                .map(|column| escape_delimited(&column.label, delimiter))
                .collect::<Vec<_>>()
                .join(&delimiter.to_string()),
        );
    }
    for row_index in row_model {
        let values = column_ids
            .iter()
            .map(|column_id| {
                let column = columns.iter().find(|column| column.id == *column_id);
                let value = source.cell_value(*row_index, column_id);
                let text = match (options.value_mode, column) {
                    (GridExportValueMode::Display, Some(column)) => {
                        value.display(&column.formatter)
                    }
                    _ => raw_value_text(&value),
                };
                escape_delimited(&text, delimiter)
            })
            .collect::<Vec<_>>();
        lines.push(values.join(&delimiter.to_string()));
    }
    lines.join("\n")
}

fn export_json(
    source: &dyn GridRowSource,
    columns: &[GridColumnDef],
    row_model: &[usize],
    column_ids: &[GridColumnId],
    options: &GridExportOptions,
) -> String {
    let rows = row_model
        .iter()
        .map(|row_index| {
            let row_id = source.row_id(*row_index);
            let cells = column_ids
                .iter()
                .map(|column_id| {
                    let column = columns.iter().find(|column| column.id == *column_id);
                    let value = source.cell_value(*row_index, column_id);
                    let text = match (options.value_mode, column) {
                        (GridExportValueMode::Display, Some(column)) => {
                            value.display(&column.formatter)
                        }
                        _ => raw_value_text(&value),
                    };
                    format!(
                        "\"{}\":\"{}\"",
                        json_escape(&column_id.0),
                        json_escape(&text)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "{{\"row_id\":\"{}\",\"cells\":{{{}}}}}",
                json_escape(&row_id.0),
                cells
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rows.join(","))
}

fn raw_value_text(value: &GridCellValue) -> String {
    match value {
        GridCellValue::Empty => String::new(),
        GridCellValue::Text(value)
        | GridCellValue::Badge(value)
        | GridCellValue::Status(value)
        | GridCellValue::Source(value)
        | GridCellValue::Timestamp(value)
        | GridCellValue::Date(value)
        | GridCellValue::Link(value)
        | GridCellValue::Json(value)
        | GridCellValue::Error(value)
        | GridCellValue::AgentAnnotation(value) => value.clone(),
        GridCellValue::Integer(value) => value.to_string(),
        GridCellValue::Decimal(value) | GridCellValue::DeltaBar(value) => value.to_string(),
        GridCellValue::Sparkline(values) => values
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(","),
    }
}

fn escape_delimited(value: &str, delimiter: char) -> String {
    if value.contains(delimiter) || value.contains('\n') || value.contains('"') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

#[allow(dead_code)]
fn _keep_row_id_type(_: &GridRowId) {}
