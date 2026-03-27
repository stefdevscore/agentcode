# agentcode

JavaScript wrapper for agentcode. Release-built packages expose the `agentcode` CLI and resolve bundled platform binaries at runtime.

## Usage

```bash
agentcode map --budget 8k --json
agentcode mcp
```

## Release Workflow

Stage real binaries with these environment variables:

```bash
export AGENTCODE_BIN_DARWIN_ARM64=/abs/path/to/agentcode-darwin-arm64
export AGENTCODE_BIN_DARWIN_X64=/abs/path/to/agentcode-darwin-x64
export AGENTCODE_BIN_LINUX_X64=/abs/path/to/agentcode-linux-x64
export AGENTCODE_BIN_WIN32_X64=/abs/path/to/agentcode-win32-x64.exe
```

Run the repeatable local release check with real binaries:

```bash
npm run release:check
```

That command:

- removes any previously staged binaries
- stages the current binaries from the environment
- verifies platform payloads
- rejects fixture stub payloads
- checks the packed artifact contents
- produces a fresh tarball with `npm pack`

For a local dry-run with fixture payloads, use:

```bash
npm run release:check:fixture
```

The fixture variant automatically cleans the staged placeholders afterward so they do not get left behind for a later publish.

When the tarball looks correct and `npm whoami` succeeds, publish with:

```bash
npm publish --access public
```

## Notes

- `--budget` accepts raw integers and `Nk` shorthand such as `8k`.
- `--json` emits pure JSON without status lines.
- Release artifacts are expected to contain staged binaries for the supported targets.
- A source checkout is not a valid distributable npm package unless binaries have been staged.
- Set `AGENTCODE_BINARY` to point at a custom binary when testing from a source checkout.
