#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstrumentBrowserSurface {
    Tree,
    DataGrid,
    MatrixGrid,
    OptionChain,
    BondQueryBuilder,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstrumentBrowserColumn {
    pub key: &'static str,
    pub label: &'static str,
    pub min_width: u16,
}

impl InstrumentBrowserColumn {
    pub const fn new(key: &'static str, label: &'static str, min_width: u16) -> Self {
        Self {
            key,
            label,
            min_width,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstrumentBrowserSurfaceSpec {
    pub surface: InstrumentBrowserSurface,
    pub columns: Vec<InstrumentBrowserColumn>,
    pub keyboard_focusable: bool,
    pub scrollable: bool,
}

pub fn primitive_instrument_browser_surface_spec(
    surface: InstrumentBrowserSurface,
) -> InstrumentBrowserSurfaceSpec {
    let columns = match surface {
        InstrumentBrowserSurface::Tree => vec![
            InstrumentBrowserColumn::new("label", "분류", 160),
            InstrumentBrowserColumn::new("count", "항목", 72),
        ],
        InstrumentBrowserSurface::DataGrid => vec![
            InstrumentBrowserColumn::new("code", "코드", 88),
            InstrumentBrowserColumn::new("name", "이름", 180),
            InstrumentBrowserColumn::new("market", "시장", 84),
            InstrumentBrowserColumn::new("route", "차트", 96),
        ],
        InstrumentBrowserSurface::MatrixGrid => vec![
            InstrumentBrowserColumn::new("row", "행사", 88),
            InstrumentBrowserColumn::new("near", "최근월", 96),
            InstrumentBrowserColumn::new("next", "차근월", 96),
            InstrumentBrowserColumn::new("far", "원월", 96),
        ],
        InstrumentBrowserSurface::OptionChain => vec![
            InstrumentBrowserColumn::new("call", "콜", 96),
            InstrumentBrowserColumn::new("strike", "행사가", 88),
            InstrumentBrowserColumn::new("put", "풋", 96),
        ],
        InstrumentBrowserSurface::BondQueryBuilder => vec![
            InstrumentBrowserColumn::new("issuer", "발행기관", 132),
            InstrumentBrowserColumn::new("maturity", "만기", 88),
            InstrumentBrowserColumn::new("coupon", "이표", 72),
            InstrumentBrowserColumn::new("provider", "제공처", 92),
        ],
    };
    InstrumentBrowserSurfaceSpec {
        surface,
        columns,
        keyboard_focusable: true,
        scrollable: surface != InstrumentBrowserSurface::BondQueryBuilder,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screenshot_surfaces_have_focusable_non_empty_columns() {
        for surface in [
            InstrumentBrowserSurface::Tree,
            InstrumentBrowserSurface::DataGrid,
            InstrumentBrowserSurface::MatrixGrid,
            InstrumentBrowserSurface::OptionChain,
            InstrumentBrowserSurface::BondQueryBuilder,
        ] {
            let spec = primitive_instrument_browser_surface_spec(surface);
            assert!(spec.keyboard_focusable);
            assert!(!spec.columns.is_empty());
            assert!(spec.columns.iter().all(|column| column.min_width >= 72));
        }
    }
}
