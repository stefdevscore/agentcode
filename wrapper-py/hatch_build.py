from __future__ import annotations

import os
from pathlib import Path

from hatchling.builders.hooks.plugin.interface import BuildHookInterface


TARGETS = [
    ("darwin-arm64", "agentcode"),
    ("darwin-x64", "agentcode"),
    ("linux-x64", "agentcode"),
    ("win32-x64", "agentcode.exe"),
]


class CustomBuildHook(BuildHookInterface):
    def initialize(self, version: str, build_data: dict[str, object]) -> None:
        del version, build_data

        root = Path(self.root)
        missing: list[str] = []
        non_executable: list[str] = []

        for folder, filename in TARGETS:
            binary_path = root / "src" / "agentcode" / "bin" / folder / filename
            if not binary_path.exists():
                missing.append(str(binary_path))
                continue
            if not filename.endswith(".exe") and not os.access(binary_path, os.X_OK):
                non_executable.append(str(binary_path))

        if missing or non_executable:
            messages: list[str] = []
            if missing:
                messages.append(f"missing bundled binaries: {', '.join(missing)}")
            if non_executable:
                messages.append(f"non-executable bundled binaries: {', '.join(non_executable)}")
            raise RuntimeError(
                "agentcode Python package requires staged platform binaries before building; "
                + "; ".join(messages)
            )
