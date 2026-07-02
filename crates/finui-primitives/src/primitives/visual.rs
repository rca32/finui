#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrimitiveVisualSnapshotBaseline {
    pub primitive: &'static str,
    pub state: &'static str,
    pub png_path: &'static str,
    pub diff_threshold: f32,
}

pub fn primitive_visual_snapshot_baselines() -> &'static [PrimitiveVisualSnapshotBaseline] {
    &[
        PrimitiveVisualSnapshotBaseline {
            primitive: "primitives_lab",
            state: "light-open-disabled-long-text",
            png_path: "docs/images/primitives-lab-light.png",
            diff_threshold: 0.01,
        },
        PrimitiveVisualSnapshotBaseline {
            primitive: "primitives_lab",
            state: "dark-rtl-disabled",
            png_path: "docs/images/primitives-lab-dark.png",
            diff_threshold: 0.01,
        },
        PrimitiveVisualSnapshotBaseline {
            primitive: "primitives_lab",
            state: "layer-edge-placement",
            png_path: "docs/images/primitives-lab-layers.png",
            diff_threshold: 0.015,
        },
    ]
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn visual_snapshot_baselines_cover_representative_primitive_states() {
        let baselines = primitive_visual_snapshot_baselines();

        assert!(
            baselines
                .iter()
                .any(|baseline| baseline.state.contains("light"))
        );
        assert!(
            baselines
                .iter()
                .any(|baseline| baseline.state.contains("dark"))
        );
        assert!(
            baselines
                .iter()
                .any(|baseline| baseline.state.contains("disabled"))
        );
        assert!(
            baselines
                .iter()
                .any(|baseline| baseline.state.contains("long-text"))
        );
        assert!(
            baselines
                .iter()
                .any(|baseline| baseline.state.contains("edge-placement"))
        );
        assert!(
            baselines
                .iter()
                .all(|baseline| baseline.diff_threshold > 0.0 && baseline.diff_threshold <= 0.02)
        );
    }

    #[test]
    fn visual_snapshot_baseline_png_files_exist_and_have_png_signature() {
        for baseline in primitive_visual_snapshot_baselines() {
            let path = repo_root().join(baseline.png_path);
            let bytes = std::fs::read(&path).expect("baseline png");

            assert!(
                bytes.starts_with(b"\x89PNG\r\n\x1a\n"),
                "{} is not a PNG baseline",
                baseline.png_path
            );
            assert!(
                bytes.len() > 256,
                "{} should not be an empty placeholder",
                baseline.png_path
            );
        }
    }

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|path| path.parent())
            .expect("workspace root")
            .to_path_buf()
    }
}
