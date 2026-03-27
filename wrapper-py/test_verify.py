from __future__ import annotations

import json
import os
import subprocess
from pathlib import Path


def test_wrapper() -> None:
    current_dir = Path(__file__).parent
    root = current_dir.parent
    cli_path = current_dir / "src" / "agentcode" / "cli.py"
    built_binary = root / "core-rs" / "target" / "debug" / "agentcode"

    subprocess.run(
        ["cargo", "build", "--quiet"],
        cwd=root / "core-rs",
        check=True,
    )

    env = os.environ.copy()
    env["AGENTCODE_BINARY"] = str(built_binary)
    env["PYTHONPATH"] = str(current_dir / "src")
    result = subprocess.run(
        [os.sys.executable, str(cli_path), "map", "--json", "--path", str(root)],
        capture_output=True,
        text=True,
        check=True,
        env=env,
    )
    json.loads(result.stdout)


if __name__ == "__main__":
    test_wrapper()
