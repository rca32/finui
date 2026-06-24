use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use super::column::GridFormatter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridValueKind {
    Text,
    Symbol,
    Integer,
    Decimal,
    Price,
    Quantity,
    Percent,
    Timestamp,
    Date,
    Badge,
    Source,
    Status,
    Sparkline,
    DeltaBar,
    Link,
    Json,
    Error,
    AgentAnnotation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GridCellValue {
    Empty,
    Text(String),
    Integer(i64),
    Decimal(f64),
    Badge(String),
    Status(String),
    Source(String),
    Timestamp(String),
    Date(String),
    Link(String),
    Json(String),
    Error(String),
    AgentAnnotation(String),
    Sparkline(Vec<f64>),
    DeltaBar(f64),
}

impl GridCellValue {
    pub fn sort_key(&self) -> GridSortKey {
        match self {
            Self::Empty => GridSortKey::Empty,
            Self::Integer(value) => GridSortKey::Number(*value as f64),
            Self::Decimal(value) | Self::DeltaBar(value) => GridSortKey::Number(*value),
            Self::Sparkline(values) => values
                .last()
                .copied()
                .map(GridSortKey::Number)
                .unwrap_or(GridSortKey::Empty),
            Self::Text(value)
            | Self::Badge(value)
            | Self::Status(value)
            | Self::Source(value)
            | Self::Timestamp(value)
            | Self::Date(value)
            | Self::Link(value)
            | Self::Json(value)
            | Self::Error(value)
            | Self::AgentAnnotation(value) => GridSortKey::Text(value.to_lowercase()),
        }
    }

    pub fn display(&self, formatter: &GridFormatter) -> String {
        match (self, formatter) {
            (Self::Empty, _) => String::new(),
            (Self::Integer(value), GridFormatter::ThousandsDecimal { decimals: 0 }) => {
                format_integer_with_thousands(*value)
            }
            (Self::Integer(value), GridFormatter::ThousandsDecimal { decimals }) => {
                format_decimal_with_thousands(*value as f64, *decimals)
            }
            (Self::Decimal(value), GridFormatter::ThousandsDecimal { decimals }) => {
                format_decimal_with_thousands(*value, *decimals)
            }
            (Self::Decimal(value), GridFormatter::Percent { decimals }) => {
                format!("{value:.decimals$}%")
            }
            (Self::Integer(value), GridFormatter::CompactQuantity) => {
                format_compact_quantity(*value as f64)
            }
            (Self::Integer(value), _) => format!("{value}"),
            (Self::Decimal(value), GridFormatter::Decimal { decimals }) => {
                format!("{value:.decimals$}")
            }
            (Self::Decimal(value), GridFormatter::CompactQuantity) => {
                format_compact_quantity(*value)
            }
            (Self::Decimal(value), GridFormatter::Plain) => format!("{value}"),
            (Self::DeltaBar(value), GridFormatter::Decimal { decimals })
            | (Self::DeltaBar(value), GridFormatter::Percent { decimals }) => {
                format!("{value:.decimals$}")
            }
            (Self::DeltaBar(value), GridFormatter::ThousandsDecimal { decimals }) => {
                format_decimal_with_thousands(*value, *decimals)
            }
            (Self::DeltaBar(value), GridFormatter::CompactQuantity) => {
                format_compact_quantity(*value)
            }
            (Self::DeltaBar(value), GridFormatter::Plain) => format!("{value}"),
            (Self::Sparkline(values), _) => values
                .last()
                .map(|value| format!("{value}"))
                .unwrap_or_default(),
            (Self::Text(value), _)
            | (Self::Badge(value), _)
            | (Self::Status(value), _)
            | (Self::Source(value), _)
            | (Self::Timestamp(value), _)
            | (Self::Date(value), _)
            | (Self::Link(value), _)
            | (Self::Json(value), _)
            | (Self::Error(value), _)
            | (Self::AgentAnnotation(value), _) => value.clone(),
        }
    }

    pub fn contains(&self, needle: &str) -> bool {
        let query = needle.trim();
        if query.is_empty() {
            return true;
        }
        if let Some(result) = self.matches_numeric_query(query) {
            return result;
        }
        self.display(&GridFormatter::Plain)
            .to_lowercase()
            .contains(&query.to_lowercase())
    }

    fn numeric_value(&self) -> Option<f64> {
        match self {
            Self::Integer(value) => Some(*value as f64),
            Self::Decimal(value) | Self::DeltaBar(value) => Some(*value),
            Self::Sparkline(values) => values.last().copied(),
            _ => None,
        }
    }

    fn matches_numeric_query(&self, query: &str) -> Option<bool> {
        let value = self.numeric_value()?;
        let query = query.replace(',', "");
        if let Some((left, right)) = query.split_once("..") {
            let min = left.trim().parse::<f64>().ok()?;
            let max = right.trim().parse::<f64>().ok()?;
            return Some(value >= min && value <= max);
        }
        for (operator, predicate) in [
            (">=", NumericPredicate::GreaterOrEqual),
            ("<=", NumericPredicate::LessOrEqual),
            (">", NumericPredicate::Greater),
            ("<", NumericPredicate::Less),
            ("=", NumericPredicate::Equal),
        ] {
            if let Some(rest) = query.strip_prefix(operator) {
                let target = rest.trim().parse::<f64>().ok()?;
                return Some(predicate.matches(value, target));
            }
        }
        None
    }
}

fn format_decimal_with_thousands(value: f64, decimals: usize) -> String {
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let formatted = format!("{:.*}", decimals, value.abs());
    let (integer, fraction) = formatted
        .split_once('.')
        .map(|(integer, fraction)| (integer, Some(fraction)))
        .unwrap_or((formatted.as_str(), None));
    let mut output = format!("{sign}{}", insert_thousands(integer));
    if let Some(fraction) = fraction {
        output.push('.');
        output.push_str(fraction);
    }
    output
}

fn format_integer_with_thousands(value: i64) -> String {
    let sign = if value < 0 { "-" } else { "" };
    format!("{sign}{}", insert_thousands(&value.abs().to_string()))
}

fn insert_thousands(integer: &str) -> String {
    let mut reversed = String::with_capacity(integer.len() + integer.len() / 3);
    for (index, ch) in integer.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            reversed.push(',');
        }
        reversed.push(ch);
    }
    reversed.chars().rev().collect()
}

fn format_compact_quantity(value: f64) -> String {
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let abs = value.abs();
    for (suffix, unit) in [("B", 1_000_000_000.0), ("M", 1_000_000.0), ("K", 1_000.0)] {
        if abs >= unit {
            let scaled = abs / unit;
            let decimals = if scaled >= 100.0 { 0 } else { 1 };
            return format!("{sign}{scaled:.decimals$}{suffix}");
        }
    }
    format_decimal_with_thousands(value, 0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NumericPredicate {
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Equal,
}

impl NumericPredicate {
    fn matches(self, value: f64, target: f64) -> bool {
        match self {
            Self::Greater => value > target,
            Self::GreaterOrEqual => value >= target,
            Self::Less => value < target,
            Self::LessOrEqual => value <= target,
            Self::Equal => (value - target).abs() < f64::EPSILON,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GridSortKey {
    Empty,
    Number(f64),
    Text(String),
}

impl PartialOrd for GridSortKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Self::Empty, Self::Empty) => Ordering::Equal,
            (Self::Empty, _) => Ordering::Less,
            (_, Self::Empty) => Ordering::Greater,
            (Self::Number(left), Self::Number(right)) => {
                left.partial_cmp(right).unwrap_or(Ordering::Equal)
            }
            (Self::Text(left), Self::Text(right)) => left.cmp(right),
            (Self::Number(_), Self::Text(_)) => Ordering::Less,
            (Self::Text(_), Self::Number(_)) => Ordering::Greater,
        })
    }
}
