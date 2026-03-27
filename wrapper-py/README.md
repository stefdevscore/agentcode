# agentcode (Python)

Python wrapper for agentcode. Release-built packages expose the `agentcode` CLI and resolve bundled platform binaries at runtime.

## Usage

```bash
agentcode map --budget 8k --json
agentcode mcp
```

## Notes

- `--budget` accepts `Nk` shorthand such as `8k` and `32k`.
- `--json` emits pure JSON without status lines.
- Release artifacts are expected to contain staged binaries for the supported targets.
- A source checkout is not a valid wheel or sdist unless binaries have been staged.
- Set `AGENTCODE_BINARY` to point at a custom binary when testing from a source checkout.
