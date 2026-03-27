use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContextMap {
    pub root: String,
    pub files: Vec<FileMap>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileMap {
    pub path: String,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub line: usize,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SymbolKind {
    Class,
    Function,
    Method,
    Interface,
    Type,
    Variable,
    Other,
}
