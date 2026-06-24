#![allow(dead_code)]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    GridCellAlign, GridCellProvenance, GridCellValue, GridColumnDef, GridColumnId, GridColumnWidth,
    GridFormatter, GridRowId, GridValueKind, InMemoryGridRow, InMemoryGridSource,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridTablePayload {
    pub source_kind: GridPayloadSourceKind,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub tr_code: Option<String>,
    #[serde(default)]
    pub request_id: Option<String>,
    #[serde(default)]
    pub materialized_table: Option<String>,
    #[serde(default)]
    pub entity_resolution_ref: Option<String>,
    #[serde(default)]
    pub source_timestamp: Option<String>,
    #[serde(default)]
    pub received_at: Option<String>,
    pub columns: Vec<GridPayloadColumn>,
    pub rows: Vec<GridPayloadRow>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GridPayloadSourceKind {
    Endpoint,
    DuckDb,
}

impl GridPayloadSourceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Endpoint => "endpoint",
            Self::DuckDb => "duckdb",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridPayloadColumn {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub kind: Option<GridValueKind>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridPayloadRow {
    pub id: String,
    pub cells: BTreeMap<String, Value>,
    #[serde(default)]
    pub evidence_ref: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GridPayloadAdapterResult {
    pub columns: Vec<GridColumnDef>,
    pub source: InMemoryGridSource,
}

pub fn adapt_table_payload(payload: &GridTablePayload) -> GridPayloadAdapterResult {
    let columns = payload
        .columns
        .iter()
        .map(adapt_payload_column)
        .collect::<Vec<_>>();
    let rows = payload
        .rows
        .iter()
        .map(|row| adapt_payload_row(payload, row))
        .collect::<Vec<_>>();
    GridPayloadAdapterResult {
        columns,
        source: InMemoryGridSource::new(rows),
    }
}

pub fn endpoint_fixture_payload() -> GridTablePayload {
    GridTablePayload {
        source_kind: GridPayloadSourceKind::Endpoint,
        endpoint: Some("/api/market/watch".to_owned()),
        tr_code: Some("QUOTE_V1".to_owned()),
        request_id: Some("endpoint-grid-fixture".to_owned()),
        materialized_table: None,
        entity_resolution_ref: Some("resolver:item-master".to_owned()),
        source_timestamp: Some("2026-06-04T09:30:00+09:00".to_owned()),
        received_at: Some("2026-06-04T09:30:01+09:00".to_owned()),
        columns: market_payload_columns(),
        rows: market_payload_rows("evidence:endpoint"),
    }
}

pub fn duckdb_fixture_payload() -> GridTablePayload {
    GridTablePayload {
        source_kind: GridPayloadSourceKind::DuckDb,
        endpoint: None,
        tr_code: None,
        request_id: Some("duckdb-grid-fixture".to_owned()),
        materialized_table: Some("market_watch_materialized".to_owned()),
        entity_resolution_ref: Some("resolver:item-master".to_owned()),
        source_timestamp: Some("2026-06-04T09:30:00+09:00".to_owned()),
        received_at: Some("2026-06-04T09:30:02+09:00".to_owned()),
        columns: market_payload_columns(),
        rows: market_payload_rows("evidence:duckdb"),
    }
}

fn adapt_payload_column(column: &GridPayloadColumn) -> GridColumnDef {
    let kind = column
        .kind
        .unwrap_or_else(|| infer_kind_from_column_id(&column.id));
    let mut column_def = GridColumnDef::new(column.id.clone(), column.label.clone())
        .kind(kind)
        .sortable(true)
        .filterable(true)
        .width(GridColumnWidth::Fixed(default_width_for_kind(kind)));
    if matches!(
        kind,
        GridValueKind::Integer
            | GridValueKind::Decimal
            | GridValueKind::Price
            | GridValueKind::Quantity
            | GridValueKind::Percent
            | GridValueKind::DeltaBar
    ) {
        column_def = column_def.align(GridCellAlign::Right);
    }
    if matches!(kind, GridValueKind::Price) {
        column_def = column_def.formatter(GridFormatter::ThousandsDecimal { decimals: 2 });
    } else if matches!(kind, GridValueKind::Percent) {
        column_def = column_def.formatter(GridFormatter::Decimal { decimals: 2 });
    } else if matches!(kind, GridValueKind::Quantity) {
        column_def = column_def.formatter(GridFormatter::CompactQuantity);
    }
    if matches!(kind, GridValueKind::Symbol | GridValueKind::Status) {
        column_def = column_def.pinnable(true);
    }
    column_def
}

fn adapt_payload_row(payload: &GridTablePayload, row: &GridPayloadRow) -> InMemoryGridRow {
    let mut cells = BTreeMap::new();
    let mut provenance = BTreeMap::new();
    for column in &payload.columns {
        let column_id = GridColumnId::new(column.id.clone());
        let kind = column
            .kind
            .unwrap_or_else(|| infer_kind_from_column_id(&column.id));
        let value = row
            .cells
            .get(&column.id)
            .map(|value| adapt_json_value(value, kind))
            .unwrap_or(GridCellValue::Empty);
        cells.insert(column_id.clone(), value);
        provenance.insert(column_id, payload_cell_provenance(payload, row));
    }
    InMemoryGridRow {
        id: GridRowId::new(row.id.clone()),
        cells,
        provenance,
    }
}

fn payload_cell_provenance(payload: &GridTablePayload, row: &GridPayloadRow) -> GridCellProvenance {
    GridCellProvenance {
        source_kind: payload.source_kind.as_str().to_owned(),
        endpoint: payload.endpoint.clone(),
        tr_code: payload.tr_code.clone(),
        request_id: payload.request_id.clone(),
        source_timestamp: payload.source_timestamp.clone(),
        received_at: payload.received_at.clone(),
        materialized_table: payload.materialized_table.clone(),
        entity_resolution_ref: payload.entity_resolution_ref.clone(),
        stale_after_ms: Some(3_000),
        evidence_ref: row.evidence_ref.clone(),
    }
}

fn adapt_json_value(value: &Value, kind: GridValueKind) -> GridCellValue {
    match kind {
        GridValueKind::Integer | GridValueKind::Quantity => value
            .as_i64()
            .map(GridCellValue::Integer)
            .or_else(|| {
                value
                    .as_f64()
                    .map(|value| GridCellValue::Integer(value as i64))
            })
            .unwrap_or_else(|| GridCellValue::Text(value_text(value))),
        GridValueKind::Decimal | GridValueKind::Price | GridValueKind::Percent => value
            .as_f64()
            .map(GridCellValue::Decimal)
            .unwrap_or_else(|| GridCellValue::Text(value_text(value))),
        GridValueKind::Status => GridCellValue::Status(value_text(value)),
        GridValueKind::Source => GridCellValue::Source(value_text(value)),
        GridValueKind::Timestamp => GridCellValue::Timestamp(value_text(value)),
        GridValueKind::Date => GridCellValue::Date(value_text(value)),
        GridValueKind::Badge => GridCellValue::Badge(value_text(value)),
        GridValueKind::Json => GridCellValue::Json(value.to_string()),
        _ => GridCellValue::Text(value_text(value)),
    }
}

fn value_text(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::String(value) => value.clone(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        _ => value.to_string(),
    }
}

fn infer_kind_from_column_id(id: &str) -> GridValueKind {
    match id {
        "symbol" => GridValueKind::Symbol,
        "price" => GridValueKind::Price,
        "change_pct" => GridValueKind::Percent,
        "volume" => GridValueKind::Quantity,
        "source" => GridValueKind::Source,
        "status" => GridValueKind::Status,
        "source_timestamp" | "received_at" => GridValueKind::Timestamp,
        _ => GridValueKind::Text,
    }
}

fn default_width_for_kind(kind: GridValueKind) -> f32 {
    match kind {
        GridValueKind::Symbol => 82.0,
        GridValueKind::Price | GridValueKind::Quantity => 98.0,
        GridValueKind::Percent => 78.0,
        GridValueKind::Status => 78.0,
        GridValueKind::Source => 120.0,
        GridValueKind::Timestamp => 156.0,
        _ => 118.0,
    }
}

fn market_payload_columns() -> Vec<GridPayloadColumn> {
    vec![
        payload_column("symbol", "Symbol", GridValueKind::Symbol),
        payload_column("name", "Name", GridValueKind::Text),
        payload_column("price", "Price", GridValueKind::Price),
        payload_column("change_pct", "Change", GridValueKind::Percent),
        payload_column("volume", "Volume", GridValueKind::Quantity),
        payload_column("source", "Source", GridValueKind::Source),
        payload_column("status", "Status", GridValueKind::Status),
    ]
}

fn payload_column(id: &str, label: &str, kind: GridValueKind) -> GridPayloadColumn {
    GridPayloadColumn {
        id: id.to_owned(),
        label: label.to_owned(),
        kind: Some(kind),
    }
}

fn market_payload_rows(evidence_prefix: &str) -> Vec<GridPayloadRow> {
    [
        ("ACME", "Acme Holdings", 74200.0, 1.42, 18_233_422_i64),
        ("NOVA", "Nova Memory", 231500.0, 2.82, 6_334_211),
        ("ORBT", "Orbit Search", 214000.0, -0.55, 1_882_140),
    ]
    .into_iter()
    .map(|(symbol, name, price, change_pct, volume)| {
        let mut cells = BTreeMap::new();
        cells.insert("symbol".to_owned(), Value::String(symbol.to_owned()));
        cells.insert("name".to_owned(), Value::String(name.to_owned()));
        cells.insert("price".to_owned(), Value::from(price));
        cells.insert("change_pct".to_owned(), Value::from(change_pct));
        cells.insert("volume".to_owned(), Value::from(volume));
        cells.insert("source".to_owned(), Value::String("api".to_owned()));
        cells.insert("status".to_owned(), Value::String("snapshot".to_owned()));
        GridPayloadRow {
            id: symbol.to_owned(),
            cells,
            evidence_ref: Some(format!("{evidence_prefix}:{symbol}")),
        }
    })
    .collect()
}
