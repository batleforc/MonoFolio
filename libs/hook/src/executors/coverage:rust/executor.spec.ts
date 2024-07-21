import { ExecutorContext } from '@nx/devkit';

import { CoverageRustExecutorSchema } from './schema';
import executor from './executor';

const options: CoverageRustExecutorSchema = {};
const context: ExecutorContext = {
  root: '',
  cwd: process.cwd(),
  isVerbose: false,
};

describe('CoverageRust Executor', () => {
  it('can run', async () => {
    const output = await executor(options, context);
    expect(output.success).toBe(true);
  });
});
