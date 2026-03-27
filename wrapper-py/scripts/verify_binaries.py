from __future__ import annotations

import os
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
TARGETS = [
    ("darwin-arm64", "agentcode"),
    ("darwin-x64", "agentcode"),
    ("linux-x64", "agentcode"),
    ("win32-x64", "agentcode.exe"),
]


def main() -> None:
    for folder, filename in TARGETS:
        path = ROOT / "src" / "agentcode" / "bin" / folder / filename
        if not path.exists():
            raise SystemExit(f"missing bundled binary for {folder}: {path}")
        if not filename.endswith(".exe") and not os.access(path, os.X_OK):
            raise SystemExit(f"bundled binary is not executable: {path}")


if __name__ == "__main__":
    main()
