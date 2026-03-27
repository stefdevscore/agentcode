import { readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { targets } from './targets.mjs';

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = resolve(__dirname, '..');

const fixturePayloads = new Set([
  '#!/bin/sh\necho "stub:$*"\n',
  '@echo off\r\necho stub:%*\r\n',
]);

for (const target of targets) {
  const filePath = join(root, 'bin', target.name, target.filename);
  const payload = readFileSync(filePath).toString('utf8');
  if (fixturePayloads.has(payload)) {
    throw new Error(
      `Refusing production release with fixture payload for ${target.name}: ${filePath}`
    );
  }
}
