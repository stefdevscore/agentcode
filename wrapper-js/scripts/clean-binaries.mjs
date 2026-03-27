import { existsSync, rmSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { targets } from './targets.mjs';

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = resolve(__dirname, '..');

for (const target of targets) {
  const filePath = join(root, 'bin', target.name, target.filename);
  if (existsSync(filePath)) {
    rmSync(filePath, { force: true });
  }
}
