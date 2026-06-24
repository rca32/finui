use std::collections::BTreeMap;

use super::cell::{GridCellValue, GridValueKind};
use super::column::{GridCellAlign, GridColumnDef, GridColumnWidth, GridFormatter};
use super::ids::{GridColumnId, GridRowId};
use super::provenance::GridCellProvenance;
use super::source::InMemoryGridRow;

pub fn demo_columns() -> Vec<GridColumnDef> {
    vec![
        GridColumnDef::new("symbol", "Symbol")
            .kind(GridValueKind::Symbol)
            .width(GridColumnWidth::Fixed(82.0))
            .sortable(true)
            .pinnable(true),
        GridColumnDef::new("name", "Name")
            .width(GridColumnWidth::Fixed(112.0))
            .sortable(true),
        GridColumnDef::new("price", "Price")
            .kind(GridValueKind::Price)
            .align(GridCellAlign::Right)
            .width(GridColumnWidth::Fixed(86.0))
            .sortable(true)
            .pinnable(true)
            .formatter(GridFormatter::ThousandsDecimal { decimals: 0 }),
        GridColumnDef::new("change_pct", "Change")
            .kind(GridValueKind::Percent)
            .align(GridCellAlign::Right)
            .width(GridColumnWidth::Fixed(78.0))
            .sortable(true)
            .filterable(true)
            .formatter(GridFormatter::Percent { decimals: 2 }),
        GridColumnDef::new("volume", "Volume")
            .kind(GridValueKind::Quantity)
            .align(GridCellAlign::Right)
            .width(GridColumnWidth::Fixed(98.0))
            .sortable(true)
            .formatter(GridFormatter::CompactQuantity),
        GridColumnDef::new("source", "Source")
            .kind(GridValueKind::Source)
            .width(GridColumnWidth::Fixed(86.0)),
        GridColumnDef::new("status", "Status")
            .kind(GridValueKind::Status)
            .width(GridColumnWidth::Fixed(78.0)),
        GridColumnDef::new("trend", "Trend")
            .kind(GridValueKind::Sparkline)
            .width(GridColumnWidth::Fixed(96.0)),
        GridColumnDef::new("delta", "Delta")
            .kind(GridValueKind::DeltaBar)
            .width(GridColumnWidth::Fixed(74.0)),
    ]
}

pub fn demo_rows(count: usize) -> Vec<InMemoryGridRow> {
    let seeds = [
        ("ACME", "Acme Holdings", 74200.0, 1.42, 18233422_i64),
        ("NOVA", "Nova Memory", 231500.0, 2.82, 6334211),
        ("ORBT", "Orbit Search", 214000.0, -0.55, 1882140),
        ("051910", "LG Chem", 384000.0, -1.15, 902211),
    ];
    (0..count)
        .map(|index| {
            let seed = seeds[index % seeds.len()];
            let symbol = if index < seeds.len() {
                seed.0.to_owned()
            } else {
                format!("9{index:05}")
            };
            let row_id = GridRowId::new(symbol.clone());
            let mut cells = BTreeMap::new();
            cells.insert(GridColumnId::new("symbol"), GridCellValue::Text(symbol));
            cells.insert(
                GridColumnId::new("name"),
                GridCellValue::Text(seed.1.to_owned()),
            );
            cells.insert(
                GridColumnId::new("price"),
                GridCellValue::Decimal(seed.2 + index as f64),
            );
            cells.insert(
                GridColumnId::new("change_pct"),
                GridCellValue::Decimal(seed.3 - (index % 7) as f64 * 0.08),
            );
            cells.insert(
                GridColumnId::new("volume"),
                GridCellValue::Integer(seed.4 + index as i64 * 11),
            );
            cells.insert(
                GridColumnId::new("source"),
                GridCellValue::Badge("api".to_owned()),
            );
            cells.insert(
                GridColumnId::new("status"),
                GridCellValue::Status(
                    if index % 11 == 0 { "stale" } else { "snapshot" }.to_owned(),
                ),
            );
            cells.insert(
                GridColumnId::new("trend"),
                GridCellValue::Sparkline(vec![
                    seed.2 - 1200.0,
                    seed.2 - 400.0 + index as f64,
                    seed.2 + 300.0,
                    seed.2 - 150.0,
                    seed.2 + index as f64,
                ]),
            );
            cells.insert(
                GridColumnId::new("delta"),
                GridCellValue::DeltaBar(seed.3 * 18.0),
            );
            let mut provenance = BTreeMap::new();
            provenance.insert(
                GridColumnId::new("price"),
                GridCellProvenance {
                    source_kind: "api".to_owned(),
                    endpoint: Some("/api/market/watch".to_owned()),
                    tr_code: Some("QUOTE_V1".to_owned()),
                    request_id: Some(format!("grid-demo-{index}")),
                    source_timestamp: Some("2026-06-04T09:30:00+09:00".to_owned()),
                    received_at: Some("2026-06-04T09:30:01+09:00".to_owned()),
                    materialized_table: None,
                    entity_resolution_ref: Some("resolver:item-master".to_owned()),
                    stale_after_ms: Some(3_000),
                    evidence_ref: Some(format!("evidence:grid-demo:{index}")),
                },
            );
            InMemoryGridRow {
                id: row_id,
                cells,
                provenance,
            }
        })
        .collect()
}
