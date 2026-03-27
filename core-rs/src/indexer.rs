use crate::models::{Symbol, SymbolKind};
use std::path::Path;
use tree_sitter::{Language, Parser, Query, QueryCursor};

pub fn index_file(path: &Path) -> Vec<Symbol> {
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    let content = std::fs::read_to_string(path).unwrap_or_default();

    match extension {
        "ts" | "js" => extract_symbols(&content, tree_sitter_typescript::language_typescript()),
        "tsx" | "jsx" => extract_symbols(&content, tree_sitter_typescript::language_tsx()),
        "py" => extract_symbols(&content, tree_sitter_python::language()),
        "rs" => extract_symbols(&content, tree_sitter_rust::language()),
        _ => Vec::new(),
    }
}

fn extract_symbols(content: &str, language: Language) -> Vec<Symbol> {
    let mut parser = Parser::new();
    parser
        .set_language(language)
        .expect("failed to load tree-sitter language");

    let Some(tree) = parser.parse(content, None) else {
        return Vec::new();
    };

    let query_str = if language == tree_sitter_python::language() {
        "
        (class_definition name: (identifier) @name) @item
        (function_definition name: (identifier) @name) @item
        "
    } else if language == tree_sitter_rust::language() {
        "
        (struct_item name: (type_identifier) @name) @item
        (enum_item name: (type_identifier) @name) @item
        (function_item name: (identifier) @name) @item
        "
    } else {
        "
        (class_declaration name: (type_identifier) @name) @item
        (function_declaration name: (identifier) @name) @item
        (method_definition name: (property_identifier) @name) @item
        (interface_declaration name: (type_identifier) @name) @item
        (type_alias_declaration name: (type_identifier) @name) @item
        (lexical_declaration
          (variable_declarator
            name: (identifier) @name
            value: [(arrow_function) (function_expression)])) @item
        "
    };

    let query = Query::new(language, query_str).expect("failed to compile query");
    let mut cursor = QueryCursor::new();
    let matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

    let mut symbols = Vec::new();
    for matched in matches {
        let mut name = None;
        let mut item = None;

        for capture in matched.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            match capture_name.as_str() {
                "name" => name = Some(content[capture.node.byte_range()].to_string()),
                "item" => item = Some(capture.node),
                _ => {}
            }
        }

        let (Some(name), Some(node)) = (name, item) else {
            continue;
        };

        let kind = match node.kind() {
            "class_declaration" | "class_definition" | "struct_item" => SymbolKind::Class,
            "interface_declaration" => SymbolKind::Interface,
            "type_alias_declaration" | "enum_item" | "enum_declaration" => SymbolKind::Type,
            "function_declaration" | "function_definition" | "function_item" => {
                SymbolKind::Function
            }
            "method_definition" => SymbolKind::Method,
            "lexical_declaration" => SymbolKind::Function,
            _ => SymbolKind::Other,
        };

        symbols.push(Symbol {
            name,
            kind,
            line: node.start_position().row + 1,
            signature: content[node.byte_range()]
                .lines()
                .next()
                .map(|value| value.trim().to_string()),
        });
    }

    symbols.sort_by(|left, right| {
        left.line
            .cmp(&right.line)
            .then_with(|| left.name.cmp(&right.name))
    });
    symbols
}

#[cfg(test)]
mod tests {
    use super::extract_symbols;
    use crate::models::SymbolKind;

    #[test]
    fn extracts_representative_typescript_symbols() {
        let source = r#"
        export interface Shape {}
        export type Identifier = string;
        export class Box {}
        export function makeBox() {}
        export const makeThing = () => {};
        "#;

        let symbols = extract_symbols(source, tree_sitter_typescript::language_typescript());
        let names: Vec<_> = symbols.iter().map(|symbol| symbol.name.as_str()).collect();
        assert!(names.contains(&"Shape"));
        assert!(names.contains(&"Identifier"));
        assert!(names.contains(&"Box"));
        assert!(names.contains(&"makeBox"));
        assert!(names.contains(&"makeThing"));
        assert!(symbols
            .iter()
            .any(|symbol| symbol.kind == SymbolKind::Interface));
        assert!(symbols.iter().any(|symbol| symbol.kind == SymbolKind::Type));
    }

    #[test]
    fn extracts_tsx_symbols_from_jsx_bearing_source() {
        let source = r#"
        export interface Props { name: string }
        export const Widget = ({ name }: Props) => <div>{name}</div>;
        export function App() { return <Widget name="x" />; }
        "#;

        let symbols = extract_symbols(source, tree_sitter_typescript::language_tsx());
        let names: Vec<_> = symbols.iter().map(|symbol| symbol.name.as_str()).collect();
        assert!(names.contains(&"Props"));
        assert!(names.contains(&"Widget"));
        assert!(names.contains(&"App"));
    }
}
