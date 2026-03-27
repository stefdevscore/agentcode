use crate::budgeter::{apply_budget, Budget, BudgetSummary};
use crate::indexer::index_file;
use crate::models::{ContextMap, FileMap};
use crate::walker::walk_project;
use anyhow::{Context, Result};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct MapOutput {
    pub full_map: ContextMap,
    pub pruned_map: ContextMap,
    pub summary: BudgetSummary,
    pub scanned_files: usize,
}

pub fn build_context_map(root: &Path, budget: Budget) -> Result<MapOutput> {
    let canonical_root = root
        .canonicalize()
        .with_context(|| format!("failed to resolve root path: {}", root.display()))?;
    let files = walk_project(&canonical_root);
    let mut indexed_files = Vec::new();

    for file_path in &files {
        let symbols = index_file(file_path);
        if symbols.is_empty() {
            continue;
        }

        let relative = file_path
            .strip_prefix(&canonical_root)
            .unwrap_or(file_path)
            .to_string_lossy()
            .replace('\\', "/");
        indexed_files.push(FileMap {
            path: relative,
            symbols,
        });
    }

    indexed_files.sort_by(|left, right| left.path.cmp(&right.path));
    let full_map = ContextMap {
        root: canonical_root.to_string_lossy().to_string(),
        files: indexed_files,
    };
    let (pruned_map, summary) = apply_budget(&full_map, budget);

    Ok(MapOutput {
        full_map,
        pruned_map,
        summary,
        scanned_files: files.len(),
    })
}

pub fn serialize_map(map: &ContextMap) -> Result<String> {
    serde_json::to_string(map).context("failed to serialize context map")
}

#[cfg(test)]
mod tests {
    use super::{build_context_map, serialize_map};
    use crate::budgeter::Budget;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn normalizes_relative_paths() {
        let fixture_root = make_temp_dir("paths");
        fs::create_dir_all(fixture_root.join("src")).unwrap();
        fs::write(fixture_root.join("src/lib.rs"), "pub fn alpha() {}\n").unwrap();

        let output = build_context_map(&fixture_root, Budget::parse("8k").unwrap()).unwrap();
        assert_eq!(output.pruned_map.files[0].path, "src/lib.rs");
        assert!(serialize_map(&output.pruned_map)
            .unwrap()
            .contains("\"src/lib.rs\""));
    }

    fn make_temp_dir(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("agentcode-{label}-{unique}"));
        fs::create_dir_all(&path).unwrap();
        path
    }
}
