import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = join(__dirname, '..', '..');
const builtBinary = join(root, 'core-rs', 'target', 'debug', 'agentcode');

execFileSync('cargo', ['build', '--quiet'], {
  cwd: join(root, 'core-rs'),
  stdio: 'inherit',
});

process.env.AGENTCODE_BINARY = builtBinary;

try {
  const output = execFileSync(
    'node',
    [join(__dirname, '..', 'bin', 'agentcode.js'), 'map', '--json', '--path', root],
    { encoding: 'utf8' }
  );
  JSON.parse(output);
  console.log('JS wrapper verification successful.');
} catch {
  console.error('JS wrapper verification failed.');
  process.exit(1);
}
