use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridCellProvenance {
    pub source_kind: String,
    pub endpoint: Option<String>,
    pub tr_code: Option<String>,
    pub request_id: Option<String>,
    pub source_timestamp: Option<String>,
    pub received_at: Option<String>,
    pub materialized_table: Option<String>,
    pub entity_resolution_ref: Option<String>,
    pub stale_after_ms: Option<u64>,
    pub evidence_ref: Option<String>,
}

impl GridCellProvenance {
    #[allow(dead_code)]
    pub fn is_stale(&self, now_ms: u64, received_at_ms: Option<u64>) -> bool {
        match (self.stale_after_ms, received_at_ms) {
            (Some(stale_after_ms), Some(received_at_ms)) => {
                now_ms.saturating_sub(received_at_ms) > stale_after_ms
            }
            _ => false,
        }
    }
}
