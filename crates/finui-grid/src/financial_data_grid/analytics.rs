#![allow(dead_code)]

use std::collections::BTreeMap;

use super::cell::GridCellValue;
use super::ids::GridColumnId;
use super::source::GridRowSource;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridAggregation {
    Count,
    Sum,
    Average,
    Min,
    Max,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridGroupSpec {
    pub column_id: GridColumnId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridAggregationSpec {
    pub column_id: GridColumnId,
    pub aggregation: GridAggregation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GridAnalyticsRow {
    pub key: String,
    pub row_count: usize,
    pub values: BTreeMap<GridColumnId, GridCellValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridFacetValue {
    pub value: String,
    pub row_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridPivotSpec {
    pub row_column_id: GridColumnId,
    pub column_column_id: GridColumnId,
    pub value: GridAggregationSpec,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GridPivotRow {
    pub row_key: String,
    pub cells: BTreeMap<String, GridCellValue>,
}

pub fn group_and_aggregate(
    source: &dyn GridRowSource,
    row_model: &[usize],
    group: &GridGroupSpec,
    aggregations: &[GridAggregationSpec],
) -> Vec<GridAnalyticsRow> {
    let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for row_index in row_model {
        let key = source
            .cell_value(*row_index, &group.column_id)
            .display(&super::column::GridFormatter::Plain);
        groups.entry(key).or_default().push(*row_index);
    }
    groups
        .into_iter()
        .map(|(key, rows)| {
            let values = aggregations
                .iter()
                .map(|spec| {
                    (
                        spec.column_id.clone(),
                        aggregate_column(source, &rows, &spec.column_id, spec.aggregation),
                    )
                })
                .collect();
            GridAnalyticsRow {
                key,
                row_count: rows.len(),
                values,
            }
        })
        .collect()
}

pub fn facet_counts(
    source: &dyn GridRowSource,
    row_model: &[usize],
    column_id: &GridColumnId,
) -> Vec<GridFacetValue> {
    let mut counts = BTreeMap::<String, usize>::new();
    for row_index in row_model {
        let value = source
            .cell_value(*row_index, column_id)
            .display(&super::column::GridFormatter::Plain);
        *counts.entry(value).or_default() += 1;
    }
    counts
        .into_iter()
        .map(|(value, row_count)| GridFacetValue { value, row_count })
        .collect()
}

pub fn pivot_aggregate(
    source: &dyn GridRowSource,
    row_model: &[usize],
    spec: &GridPivotSpec,
) -> Vec<GridPivotRow> {
    let mut buckets = BTreeMap::<String, BTreeMap<String, Vec<usize>>>::new();
    for row_index in row_model {
        let row_key = source
            .cell_value(*row_index, &spec.row_column_id)
            .display(&super::column::GridFormatter::Plain);
        let column_key = source
            .cell_value(*row_index, &spec.column_column_id)
            .display(&super::column::GridFormatter::Plain);
        buckets
            .entry(row_key)
            .or_default()
            .entry(column_key)
            .or_default()
            .push(*row_index);
    }
    buckets
        .into_iter()
        .map(|(row_key, columns)| {
            let cells = columns
                .into_iter()
                .map(|(column_key, rows)| {
                    (
                        column_key,
                        aggregate_column(
                            source,
                            &rows,
                            &spec.value.column_id,
                            spec.value.aggregation,
                        ),
                    )
                })
                .collect();
            GridPivotRow { row_key, cells }
        })
        .collect()
}

fn aggregate_column(
    source: &dyn GridRowSource,
    row_model: &[usize],
    column_id: &GridColumnId,
    aggregation: GridAggregation,
) -> GridCellValue {
    match aggregation {
        GridAggregation::Count => GridCellValue::Integer(row_model.len() as i64),
        GridAggregation::Sum => {
            GridCellValue::Decimal(numeric_values(source, row_model, column_id).iter().sum())
        }
        GridAggregation::Average => {
            let values = numeric_values(source, row_model, column_id);
            if values.is_empty() {
                GridCellValue::Empty
            } else {
                GridCellValue::Decimal(values.iter().sum::<f64>() / values.len() as f64)
            }
        }
        GridAggregation::Min => numeric_values(source, row_model, column_id)
            .into_iter()
            .reduce(f64::min)
            .map(GridCellValue::Decimal)
            .unwrap_or(GridCellValue::Empty),
        GridAggregation::Max => numeric_values(source, row_model, column_id)
            .into_iter()
            .reduce(f64::max)
            .map(GridCellValue::Decimal)
            .unwrap_or(GridCellValue::Empty),
    }
}

fn numeric_values(
    source: &dyn GridRowSource,
    row_model: &[usize],
    column_id: &GridColumnId,
) -> Vec<f64> {
    row_model
        .iter()
        .filter_map(|row_index| match source.cell_value(*row_index, column_id) {
            GridCellValue::Integer(value) => Some(value as f64),
            GridCellValue::Decimal(value) | GridCellValue::DeltaBar(value) => Some(value),
            GridCellValue::Sparkline(values) => values.last().copied(),
            _ => None,
        })
        .collect()
}
