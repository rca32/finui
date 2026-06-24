use super::super::{
    cell::GridCellValue,
    column::GridFormatter,
    ids::{GridCellRef, GridColumnId, GridRowId},
    paint::ellipsize_cell_text,
    row::{GridRowWindow, page_row_model},
    state::GridState,
};

#[test]
fn split_state_tests_module_exercises_row_window() {
    let row_model = vec![10, 11, 12, 13, 14];
    let page = page_row_model(&row_model, 1, 2);
    let window = GridRowWindow::new(4, 10, 5);

    assert_eq!(page, &[11, 12]);
    assert_eq!(window.range(), 4..5);
    assert_eq!(window.len(), 1);
    assert!(!window.is_empty());
}

#[test]
fn long_cell_text_is_ellipsized_to_cell_width() {
    let text = "Acme Holdings ultra long analyst watchlist label";

    assert_eq!(ellipsize_cell_text(text, 40.0), "...");
    assert_eq!(ellipsize_cell_text("SK", 40.0), "SK");
    assert_eq!(ellipsize_cell_text(text, 4.0), "");
}

#[test]
fn formatter_adds_thousands_and_compact_quantity_without_changing_sort_value() {
    let price = GridCellValue::Decimal(1234567.89);
    let quantity = GridCellValue::Integer(18_233_422);

    assert_eq!(
        price.display(&GridFormatter::ThousandsDecimal { decimals: 2 }),
        "1,234,567.89"
    );
    assert_eq!(quantity.display(&GridFormatter::CompactQuantity), "18.2M");
    assert_eq!(
        price.sort_key(),
        GridCellValue::Decimal(1234567.89).sort_key()
    );
}

#[test]
fn inline_edit_state_distinguishes_commit_cancel_invalid_and_read_only() {
    let cell = GridCellRef {
        row_id: GridRowId::new("ACME"),
        column_id: GridColumnId::new("price"),
    };
    let mut state = GridState::default();

    state.begin_edit(cell.clone(), GridCellValue::Decimal(74500.0));
    let committed = state.commit_edit().expect("valid mutable edit commits");
    assert_eq!(committed.cell, cell);
    assert!(state.edit_draft.is_none());

    state.begin_edit(cell.clone(), GridCellValue::Decimal(74600.0));
    state.cancel_edit();
    assert!(state.edit_draft.is_none());

    state.begin_invalid_edit(
        cell.clone(),
        GridCellValue::Error("invalid price".to_owned()),
    );
    assert!(state.commit_edit().is_none());
    assert!(state.edit_draft.as_ref().is_some_and(|draft| !draft.valid));

    state.begin_read_only_edit(cell, GridCellValue::Decimal(74700.0));
    assert!(state.commit_edit().is_none());
    assert!(
        state
            .edit_draft
            .as_ref()
            .is_some_and(|draft| draft.read_only)
    );
}
