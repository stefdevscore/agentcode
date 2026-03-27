use crate::models::{ContextMap, FileMap, Symbol};

const BYTES_PER_TOKEN: usize = 4;
const MIN_ROOT: &str = "";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Budget {
    max_tokens: usize,
}

impl Budget {
    pub fn parse(input: &str) -> Result<Self, String> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err("budget cannot be empty".to_string());
        }

        let normalized = trimmed.to_ascii_lowercase();
        let digits = if let Some(raw) = normalized.strip_suffix('k') {
            let base = raw
                .parse::<usize>()
                .map_err(|_| format!("invalid budget value: {input}"))?;
            base.checked_mul(1000)
                .ok_or_else(|| format!("budget is too large: {input}"))?
        } else {
            normalized
                .parse::<usize>()
                .map_err(|_| format!("invalid budget value: {input}"))?
        };

        if digits == 0 {
            return Err("budget must be greater than zero".to_string());
        }

        Ok(Self { max_tokens: digits })
    }

    pub fn max_tokens(self) -> usize {
        self.max_tokens
    }

    pub fn max_bytes(self) -> usize {
        self.max_tokens.saturating_mul(BYTES_PER_TOKEN)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetSummary {
    pub max_tokens: usize,
    pub max_bytes: usize,
    pub requested_max_bytes: usize,
    pub output_bytes: usize,
    pub indexed_files: usize,
    pub retained_files: usize,
}

pub fn apply_budget(full_map: &ContextMap, budget: Budget) -> (ContextMap, BudgetSummary) {
    let requested_max_bytes = budget.max_bytes();
    let mut ranked_files = full_map.files.clone();
    ranked_files.sort_by(|left, right| {
        file_score(right)
            .cmp(&file_score(left))
            .then_with(|| left.path.cmp(&right.path))
    });

    let empty_map = minimal_map(&full_map.root, requested_max_bytes);
    let max_bytes = requested_max_bytes.max(serialized_len(&empty_map));
    let mut pruned = empty_map;

    for file in ranked_files {
        let mut ranked_symbols = file.symbols.clone();
        ranked_symbols.sort_by(|left, right| {
            symbol_score(right)
                .cmp(&symbol_score(left))
                .then_with(|| left.name.cmp(&right.name))
                .then_with(|| left.line.cmp(&right.line))
        });

        let mut retained_symbols = Vec::new();
        for symbol in ranked_symbols {
            let mut candidate = pruned.clone();
            let existing = candidate
                .files
                .iter_mut()
                .find(|entry| entry.path == file.path);
            match existing {
                Some(entry) => entry.symbols.push(symbol.clone()),
                None => candidate.files.push(FileMap {
                    path: file.path.clone(),
                    symbols: vec![symbol.clone()],
                }),
            }

            if serialized_len(&candidate) <= max_bytes {
                retained_symbols.push(symbol);
                pruned = candidate;
            }
        }

        if retained_symbols.is_empty() {
            continue;
        }
    }

    while serialized_len(&pruned) > max_bytes {
        if !drop_last_symbol(&mut pruned) {
            break;
        }
    }

    let output_bytes = serialized_len(&pruned);
    let summary = BudgetSummary {
        max_tokens: budget.max_tokens(),
        max_bytes,
        requested_max_bytes,
        output_bytes,
        indexed_files: full_map.files.len(),
        retained_files: pruned.files.len(),
    };

    (pruned, summary)
}

fn minimal_map(root: &str, max_bytes: usize) -> ContextMap {
    for candidate_root in candidate_roots(root) {
        let candidate = ContextMap {
            root: candidate_root,
            files: Vec::new(),
        };
        if serialized_len(&candidate) <= max_bytes {
            return candidate;
        }
    }

    ContextMap {
        root: MIN_ROOT.to_string(),
        files: Vec::new(),
    }
}

fn candidate_roots(root: &str) -> Vec<String> {
    let mut candidates = Vec::new();
    candidates.push(root.to_string());
    if root != MIN_ROOT {
        for truncated in truncated_roots(root) {
            if !candidates.contains(&truncated) {
                candidates.push(truncated);
            }
        }
        candidates.push(MIN_ROOT.to_string());
    }
    candidates
}

fn truncated_roots(root: &str) -> Vec<String> {
    let chars: Vec<char> = root.chars().collect();
    if chars.len() <= 1 {
        return Vec::new();
    }

    let mut results = Vec::new();
    for len in (1..chars.len()).rev() {
        let mut truncated: String = chars.iter().take(len).collect();
        truncated.push_str("...");
        results.push(truncated);
    }
    results
}

fn drop_last_symbol(map: &mut ContextMap) -> bool {
    if let Some(file) = map.files.last_mut() {
        file.symbols.pop();
        if file.symbols.is_empty() {
            map.files.pop();
        }
        return true;
    }
    false
}

fn serialized_len(map: &ContextMap) -> usize {
    serde_json::to_vec(map)
        .map(|json| json.len())
        .unwrap_or(usize::MAX)
}

fn file_score(file: &FileMap) -> usize {
    let symbol_count = file.symbols.len() * 100;
    let richness = file
        .symbols
        .iter()
        .map(symbol_score)
        .fold(0usize, |acc, score| acc.saturating_add(score));
    symbol_count
        .saturating_add(richness)
        .saturating_add(file.path.len())
}

fn symbol_score(symbol: &Symbol) -> usize {
    let signature = symbol
        .signature
        .as_ref()
        .map(|value| value.len())
        .unwrap_or(0);
    signature
        .saturating_add(symbol.name.len() * 8)
        .saturating_add(kind_weight(symbol))
}

fn kind_weight(symbol: &Symbol) -> usize {
    use crate::models::SymbolKind;

    match symbol.kind {
        SymbolKind::Class => 80,
        SymbolKind::Interface => 70,
        SymbolKind::Type => 65,
        SymbolKind::Method => 60,
        SymbolKind::Function => 55,
        SymbolKind::Variable => 40,
        SymbolKind::Other => 20,
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_budget, Budget};
    use crate::models::{ContextMap, FileMap, Symbol, SymbolKind};

    #[test]
    fn parses_budget_values() {
        assert_eq!(Budget::parse("1").unwrap().max_tokens(), 1);
        assert_eq!(Budget::parse("8000").unwrap().max_tokens(), 8000);
        assert_eq!(Budget::parse("8k").unwrap().max_tokens(), 8000);
        assert_eq!(Budget::parse("32K").unwrap().max_tokens(), 32000);
        assert!(Budget::parse("0").is_err());
        assert!(Budget::parse("abc").is_err());
    }

    #[test]
    fn prunes_output_to_budget() {
        let map = ContextMap {
            root: ".".to_string(),
            files: vec![
                FileMap {
                    path: "a.rs".to_string(),
                    symbols: vec![
                        Symbol {
                            name: "Alpha".to_string(),
                            kind: SymbolKind::Class,
                            line: 1,
                            signature: Some("pub struct Alpha {}".to_string()),
                        },
                        Symbol {
                            name: "beta".to_string(),
                            kind: SymbolKind::Function,
                            line: 5,
                            signature: Some("pub fn beta() {}".to_string()),
                        },
                    ],
                },
                FileMap {
                    path: "b.rs".to_string(),
                    symbols: vec![Symbol {
                        name: "gamma".to_string(),
                        kind: SymbolKind::Function,
                        line: 3,
                        signature: Some("pub fn gamma() {}".to_string()),
                    }],
                },
            ],
        };

        let budget = Budget::parse("20").unwrap();
        let (pruned, summary) = apply_budget(&map, budget);
        assert!(serde_json::to_vec(&pruned).unwrap().len() <= summary.max_bytes);
        assert!(summary.retained_files <= summary.indexed_files);
    }

    #[test]
    fn enforces_minimum_representable_budget_for_tiny_values() {
        let map = ContextMap {
            root: "/very/long/root/path".to_string(),
            files: vec![],
        };

        let (pruned, summary) = apply_budget(&map, Budget::parse("1").unwrap());
        assert_eq!(pruned.files.len(), 0);
        assert!(serde_json::to_vec(&pruned).unwrap().len() <= summary.max_bytes);
        assert_eq!(summary.requested_max_bytes, 4);
        assert_eq!(
            summary.output_bytes,
            serde_json::to_vec(&pruned).unwrap().len()
        );
    }

    #[test]
    fn truncates_root_when_empty_map_would_overflow_requested_budget() {
        let map = ContextMap {
            root: "/this/is/a/path/that/will/not/fit".to_string(),
            files: vec![FileMap {
                path: "a.rs".to_string(),
                symbols: vec![Symbol {
                    name: "alpha".to_string(),
                    kind: SymbolKind::Function,
                    line: 1,
                    signature: Some("fn alpha() {}".to_string()),
                }],
            }],
        };

        let (pruned, summary) = apply_budget(&map, Budget::parse("2").unwrap());
        assert!(serde_json::to_vec(&pruned).unwrap().len() <= summary.max_bytes);
        assert!(pruned.root.len() <= map.root.len());
    }
}
