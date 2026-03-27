import { execFileSync } from 'node:child_process';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = resolve(__dirname, '..');
const fixtureMode = process.argv.includes('--fixture');

const run = (command, args) => {
  execFileSync(command, args, {
    cwd: root,
    stdio: 'inherit',
  });
};

run('node', ['scripts/clean-binaries.mjs']);
run('node', fixtureMode ? ['scripts/stage-binaries.mjs', '--fixture'] : ['scripts/stage-binaries.mjs']);
run('node', ['scripts/verify-binaries.mjs']);
if (!fixtureMode) {
  run('node', ['scripts/assert-release-ready.mjs']);
}
run('node', ['scripts/check-pack.mjs']);
run('npm', ['pack', '--silent']);
if (fixtureMode) {
  run('node', ['scripts/clean-binaries.mjs']);
}
