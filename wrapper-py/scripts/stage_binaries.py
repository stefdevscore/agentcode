from __future__ import annotations

import os
from pathlib import Path
import shutil
import stat
import sys


ROOT = Path(__file__).resolve().parent.parent
TARGETS = [
    ("darwin-arm64", "AGENTCODE_BIN_DARWIN_ARM64", "agentcode"),
    ("darwin-x64", "AGENTCODE_BIN_DARWIN_X64", "agentcode"),
    ("linux-x64", "AGENTCODE_BIN_LINUX_X64", "agentcode"),
    ("win32-x64", "AGENTCODE_BIN_WIN32_X64", "agentcode.exe"),
]


def main() -> None:
    fixture_mode = "--fixture" in sys.argv
    for folder, env_name, filename in TARGETS:
        destination_dir = ROOT / "src" / "agentcode" / "bin" / folder
        destination_dir.mkdir(parents=True, exist_ok=True)
        destination = destination_dir / filename

        if fixture_mode:
            if filename.endswith(".exe"):
                destination.write_text("@echo off\r\necho stub:%*\r\n", encoding="utf-8")
            else:
                destination.write_text('#!/bin/sh\necho "stub:$*"\n', encoding="utf-8")
                destination.chmod(destination.stat().st_mode | stat.S_IXUSR)
            continue

        source = os.environ.get(env_name)
        if not source:
            raise SystemExit(f"missing source binary env: {env_name}")
        source_path = Path(source)
        if not source_path.exists():
            raise SystemExit(f"missing source binary file: {source_path}")
        shutil.copy2(source_path, destination)
        if not filename.endswith(".exe"):
            destination.chmod(destination.stat().st_mode | stat.S_IXUSR)


if __name__ == "__main__":
    main()
