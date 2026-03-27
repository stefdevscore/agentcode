# agentcode

Fast codebase context for AI tools.

`agentcode` scans a repo, finds useful symbols, and prints a compact JSON map you can feed into prompts, agents, or MCP clients.

## Install

### Rust CLI

```bash
cargo install agentcode
```

### npm wrapper

```bash
npm install -g agentcode-js
```

### Python wrapper

```bash
pip install agentcode
```

## Use

```bash
agentcode map --path . --budget 8k --json
agentcode mcp
```

`--budget` accepts raw numbers and shorthand like `8k` or `32k`.

## Example Output

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

## MCP Mode

```bash
agentcode mcp
```

Public method:
`get_map`

Example request:

```json
{"jsonrpc":"2.0","id":1,"method":"get_map","params":{"path":".","budget":"8k"}}
```

Example response:

```json
{"jsonrpc":"2.0","id":1,"result":{"map":"{\"root\":\"/repo\",\"files\":[]}"}}
```

## Maintainers

The npm and Python packages ship bundled binaries for:

- `darwin-arm64`
- `darwin-x64`
- `linux-x64`
- `win32-x64`

For local development, you can override the bundled runtime with `AGENTCODE_BINARY=/path/to/agentcode`.

## License

Unlicense
