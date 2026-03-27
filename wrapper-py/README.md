# agentcode (Python)

Install `agentcode`, then run `agentcode`.

## Install

```bash
pip install agentcode
```

## Use

```bash
agentcode map --budget 8k --json
agentcode mcp
```

This package bundles the runtime, so normal installs should work out of the box.

## Maintainers

The wheel includes bundled binaries for:

- `darwin-arm64`
- `darwin-x64`
- `linux-x64`
- `win32-x64`

For local development, you can override the bundled runtime with `AGENTCODE_BINARY=/path/to/agentcode`.
