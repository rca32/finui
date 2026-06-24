use super::super::{
    agent::{GridAgentBridge, GridAgentCommand},
    ids::GridColumnId,
    state::{GridSortDirection, GridState},
};

#[test]
fn split_agent_tests_module_keeps_reversible_sort_contract() {
    let mut state = GridState::default();
    let mut bridge = GridAgentBridge::default();

    bridge.apply_command(
        &mut state,
        GridAgentCommand::SetSort {
            column_id: GridColumnId::new("price"),
            direction: GridSortDirection::Desc,
            reason_ref: "test".to_owned(),
        },
    );

    assert_eq!(bridge.action_log.len(), 1);
    assert!(bridge.action_log[0].reversible);
}
