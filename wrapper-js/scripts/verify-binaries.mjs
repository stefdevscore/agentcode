import { accessSync, constants, existsSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { targets } from './targets.mjs';

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = resolve(__dirname, '..');

for (const target of targets) {
  const filePath = join(root, 'bin', target.name, target.filename);
  if (!existsSync(filePath)) {
    throw new Error(`Missing bundled binary for ${target.name}: ${filePath}`);
  }
  if (!target.filename.endsWith('.exe')) {
    accessSync(filePath, constants.X_OK);
  }
}
