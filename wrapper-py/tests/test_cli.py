from __future__ import annotations

import os
from pathlib import Path
import platform
import tempfile

import click
from click.testing import CliRunner
import pytest

from agentcode.cli import TARGETS, cli, get_binary_path, get_packaged_binary_path


def test_env_override_binary_runs_fixture() -> None:
    fixture = Path(__file__).parent / "fixtures" / "agentcode-stub.sh"
    fixture.chmod(0o755)
    runner = CliRunner()
    env = os.environ.copy()
    env["AGENTCODE_BINARY"] = str(fixture)
    result = runner.invoke(cli, ["map", "--json"], env=env)
    assert result.exit_code == 0


def test_resolve_binary_uses_override() -> None:
    env_binary = "/tmp/agentcode"
    os.environ["AGENTCODE_BINARY"] = env_binary
    try:
        assert get_binary_path() == env_binary
    finally:
        os.environ.pop("AGENTCODE_BINARY", None)


def test_packaged_binary_path_resolution() -> None:
    with tempfile.TemporaryDirectory() as tmpdir:
        base_dir = Path(tmpdir)
        folder, executable = TARGETS[(platform.system(), platform.machine())]
        binary = base_dir / "bin" / folder / executable
        binary.parent.mkdir(parents=True)
        binary.write_text('#!/bin/sh\necho "ok"\n', encoding="utf-8")
        if not executable.endswith(".exe"):
            binary.chmod(0o755)
        assert get_packaged_binary_path(base_dir) == str(binary)


def test_missing_packaged_binary_fails_clearly() -> None:
    with tempfile.TemporaryDirectory() as tmpdir:
        with pytest.raises(click.ClickException, match="Bundled agentcode binary not found"):
            get_packaged_binary_path(Path(tmpdir))
