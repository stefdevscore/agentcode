# agentcode

> A fast codebase context mapper for AI tooling, with a Rust core and installable wrappers for JavaScript and Python.

agentcode scans Rust, Python, TypeScript, JavaScript, TSX, and JSX source files, extracts high-signal symbols, and emits a compact JSON map that can be fed into agents, prompts, or MCP clients.

---

## Features

- **Shared Rust Core**: One indexing and budgeting engine powers the CLI and MCP server.
- **Budget-Aware Output**: `--budget` accepts raw integers and shorthand like `8k`, `32k`, and `128k`.
- **Machine-Friendly JSON**: `--json` prints pure JSON without status text.
- **MCP Mode**: `agentcode mcp` serves line-delimited JSON-RPC over stdin/stdout.
- **Multi-Ecosystem Distribution**: Publishable wrappers exist for npm and PyPI, backed by the same Rust runtime.

---

## Quick Start

### Rust

```bash
cargo install agentcode
agentcode map --budget 8k --json
```

### JavaScript

```bash
npx agentcode map --budget 8k --json
```

### Python

```bash
pip install agentcode
agentcode map --budget 32k --json
```

---

## Example

```bash
agentcode map --path . --budget 8k --json
```

Example output:

```json
{
  "root": "/path/to/project",
  "files": [
    {
      "path": "src/main.rs",
      "symbols": [
        {
          "name": "main",
          "kind": "function",
          "line": 12,
          "signature": "fn main() {"
        }
      ]
    }
  ]
}
```

---

## Budget Semantics

- agentcode treats the budget as an approximate token target and enforces it against serialized JSON size.
- Very small budgets still return a valid JSON object, even when that means truncating or clearing `root`.
- Human-readable mode prints a summary plus the final JSON payload.

---

## MCP Mode

Start the MCP server:

```bash
agentcode mcp
```

Supported public method:

- `get_map`

Request shape:

```json
{"jsonrpc":"2.0","id":1,"method":"get_map","params":{"path":".","budget":"8k"}}
```

Response shape:

```json
{"jsonrpc":"2.0","id":1,"result":{"map":"{\"root\":\"/repo\",\"files\":[]}"}}
```

JSON-RPC notifications are accepted and do not produce stdout output.

---

## Platform Support

The Rust crate publishes the native CLI directly.

The npm and PyPI wrappers resolve bundled runtime binaries for:

- `darwin-arm64`
- `darwin-x64`
- `linux-x64`
- `win32-x64`

Use `AGENTCODE_BINARY=/path/to/agentcode` to override the bundled runtime during local development or custom deployment.

---

## Repository Layout

- `core-rs/` — Rust library, CLI, and MCP server
- `wrapper-js/` — npm wrapper
- `wrapper-py/` — Python wrapper

---

## License

Unlicense
