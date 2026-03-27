import { chmodSync, mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';
import { tmpdir } from 'node:os';
import { beforeAll, describe, expect, it } from 'vitest';
import { resolveAgentcodeBinary } from '../src/index.js';

const __dirname = dirname(fileURLToPath(import.meta.url));
const fixture = join(__dirname, 'fixtures', 'agentcode-stub.sh');

describe('agentcode JS wrapper', () => {
  beforeAll(() => {
    chmodSync(fixture, 0o755);
  });

  it('resolves supported packaged binary paths', () => {
    const baseDir = join(tmpdir(), `agentcode-js-${Date.now()}`);
    const binary = join(baseDir, 'bin', 'darwin-arm64', 'agentcode');
    mkdirSync(join(baseDir, 'bin', 'darwin-arm64'), { recursive: true });
    writeFileSync(binary, '#!/bin/sh\necho "ok"\n');
    chmodSync(binary, 0o755);
    expect(resolveAgentcodeBinary({ platform: 'darwin', arch: 'arm64', baseDir })).toBe(binary);
    rmSync(baseDir, { recursive: true, force: true });
  });

  it('fails clearly for unsupported targets', () => {
    expect(() => resolveAgentcodeBinary({ platform: 'linux', arch: 'arm64' })).toThrow(
      'Unsupported platform/arch'
    );
  });

  it('allows an override binary for local execution', () => {
    process.env.AGENTCODE_BINARY = fixture;
    const resolved = resolveAgentcodeBinary();
    delete process.env.AGENTCODE_BINARY;
    expect(resolved).toBe(fixture);
  });

  it('fails clearly when bundled binaries are missing', () => {
    const baseDir = join(tmpdir(), `agentcode-js-missing-${Date.now()}`);
    mkdirSync(baseDir, { recursive: true });
    expect(() =>
      resolveAgentcodeBinary({ platform: 'darwin', arch: 'x64', baseDir })
    ).toThrow('Bundled agentcode binary not found or not executable');
    rmSync(baseDir, { recursive: true, force: true });
  });
});
