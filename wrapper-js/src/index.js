import { accessSync, constants } from 'node:fs';
import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';
import { arch as runtimeArch, platform as runtimePlatform } from 'node:os';

const __dirname = dirname(fileURLToPath(import.meta.url));

const TARGETS = {
  darwin: { arm64: 'darwin-arm64', x64: 'darwin-x64' },
  linux: { x64: 'linux-x64' },
  win32: { x64: 'win32-x64' },
};

export function resolveAgentcodeBinary(options = {}) {
  const envOverride = process.env.AGENTCODE_BINARY;
  if (envOverride) {
    return envOverride;
  }

  const platform = options.platform ?? runtimePlatform();
  const arch = options.arch ?? runtimeArch();
  const target = TARGETS[platform]?.[arch];
  if (!target) {
    throw new Error(`Unsupported platform/arch: ${platform}/${arch}`);
  }

  const executable = platform === 'win32' ? 'agentcode.exe' : 'agentcode';
  const baseDir = options.baseDir ?? join(__dirname, '..');
  const packagedPath = join(baseDir, 'bin', target, executable);
  assertExecutable(packagedPath);
  return packagedPath;
}

export function runAgentcode(args = [], options = {}) {
  const binPath = resolveAgentcodeBinary(options);
  const child = spawn(binPath, args, { stdio: 'inherit' });

  child.on('exit', (code) => {
    process.exit(code ?? 0);
  });

  child.on('error', (error) => {
    console.error(`agentcode: ${error.message}`);
    process.exit(1);
  });
}

function assertExecutable(filePath) {
  try {
    accessSync(filePath, constants.X_OK);
  } catch {
    throw new Error(`Bundled agentcode binary not found or not executable at ${filePath}`);
  }
}
