#!/usr/bin/env node

import { runAgentcode } from '../src/index.js';

const args = process.argv.slice(2);
runAgentcode(args);
