from __future__ import annotations

import os
import platform
import subprocess
from importlib import resources
from pathlib import Path

import click


TARGETS = {
    ("Darwin", "arm64"): ("darwin-arm64", "agentcode"),
    ("Darwin", "x86_64"): ("darwin-x64", "agentcode"),
    ("Linux", "x86_64"): ("linux-x64", "agentcode"),
    ("Windows", "AMD64"): ("win32-x64", "agentcode.exe"),
}


def get_packaged_binary_path(base_dir: Path | None = None) -> str:
    base_dir = base_dir or Path(resources.files("agentcode"))
    env_override = os.environ.get("AGENTCODE_BINARY")
    if env_override:
        return env_override

    system = platform.system()
    machine = platform.machine()
    target = TARGETS.get((system, machine))
    if target is None:
        raise click.ClickException(f"Unsupported platform/arch: {system}/{machine}")

    folder, executable = target
    binary_path = base_dir / "bin" / folder / executable
    if not binary_path.exists():
        raise click.ClickException(
            f"Bundled agentcode binary not found at {binary_path}"
        )
    return str(binary_path)


def get_binary_path() -> str:
    return get_packaged_binary_path()


@click.command(context_settings={"ignore_unknown_options": True})
@click.argument("args", nargs=-1, type=click.UNPROCESSED)
def cli(args: tuple[str, ...]) -> None:
    """Python wrapper for the agentcode codebase indexer."""
    result = subprocess.run([get_binary_path(), *args], check=False)
    raise SystemExit(result.returncode)


if __name__ == "__main__":
    cli()
