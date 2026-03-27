import { chmodSync, copyFileSync, existsSync, mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { targets } from './targets.mjs';

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = resolve(__dirname, '..');

const fixtureMode = process.argv.includes('--fixture');

for (const target of targets) {
  const dir = join(root, 'bin', target.name);
  mkdirSync(dir, { recursive: true });
  const destination = join(dir, target.filename);

  if (fixtureMode) {
    const contents =
      target.filename.endsWith('.exe')
        ? '@echo off\r\necho stub:%*\r\n'
        : '#!/bin/sh\necho "stub:$*"\n';
    writeFileSync(destination, contents);
    if (!target.filename.endsWith('.exe')) {
      chmodSync(destination, 0o755);
    }
    continue;
  }

  const source = process.env[target.env];
  if (!source || !existsSync(source)) {
    throw new Error(`Missing source binary for ${target.name}; expected env ${target.env}`);
  }
  copyFileSync(source, destination);
  if (!target.filename.endsWith('.exe')) {
    chmodSync(destination, 0o755);
  }
}
