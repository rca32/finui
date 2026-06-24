use super::super::{
    demo_data::demo_columns,
    ids::GridColumnId,
    state::GridState,
    viewport::{GridColumnRegion, build_column_layout},
};

#[test]
fn split_viewport_tests_module_keeps_right_pin_region() {
    let columns = demo_columns();
    let mut state = GridState::default();
    state.normalize_columns(&columns);
    state.pinned_right = vec![GridColumnId::new("status")];

    let layout = build_column_layout(
        0.0,
        500.0,
        &state.visible_column_ids(),
        &[],
        &[GridColumnId::new("status")],
        &columns,
        &state,
        0.0,
    );

    assert!(
        layout.iter().any(|column| column.column_id.0 == "status"
            && column.region == GridColumnRegion::PinnedRight)
    );
}
