from __future__ import annotations

import glob
from pathlib import Path
import zipfile


ROOT = Path(__file__).resolve().parent.parent
REQUIRED = {
    "agentcode/bin/darwin-arm64/agentcode",
    "agentcode/bin/darwin-x64/agentcode",
    "agentcode/bin/linux-x64/agentcode",
    "agentcode/bin/win32-x64/agentcode.exe",
}


def main() -> None:
    wheels = sorted(glob.glob(str(ROOT / "dist" / "*.whl")))
    if not wheels:
        raise SystemExit("no wheel found in dist/")
    with zipfile.ZipFile(wheels[-1]) as archive:
        names = set(archive.namelist())
    missing = sorted(REQUIRED - names)
    if missing:
        raise SystemExit(f"wheel is missing bundled binaries: {', '.join(missing)}")


if __name__ == "__main__":
    main()
