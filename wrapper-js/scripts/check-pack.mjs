import { execFileSync } from 'node:child_process';

const output = execFileSync('npm', ['pack', '--json', '--dry-run'], { encoding: 'utf8' });
const [{ files }] = JSON.parse(output);
const required = [
  'bin/darwin-arm64/agentcode',
  'bin/darwin-x64/agentcode',
  'bin/linux-x64/agentcode',
  'bin/win32-x64/agentcode.exe',
];

for (const path of required) {
  if (!files.some((file) => file.path === path)) {
    throw new Error(`Packed npm artifact is missing ${path}`);
  }
}
