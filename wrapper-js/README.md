# agentcode-js

Install `agentcode-js`, then run `agentcode`.

## Install

```bash
npm install -g agentcode-js
```

## Use

```bash
agentcode map --budget 8k --json
agentcode mcp
```

This package bundles the runtime, so normal installs should work out of the box.

## Maintainers

Publishing requires staged binaries for:

- `darwin-arm64`
- `darwin-x64`
- `linux-x64`
- `win32-x64`

Release prep:

```bash
export AGENTCODE_BIN_DARWIN_ARM64=/abs/path/to/agentcode-darwin-arm64
export AGENTCODE_BIN_DARWIN_X64=/abs/path/to/agentcode-darwin-x64
export AGENTCODE_BIN_LINUX_X64=/abs/path/to/agentcode-linux-x64
export AGENTCODE_BIN_WIN32_X64=/abs/path/to/agentcode-win32-x64.exe
npm run release:check
npm publish
```
