import { ExecutorContext } from '@nx/devkit';

import { PreBumpRustExecutorSchema } from './schema';
import executor from './executor';

const options: PreBumpRustExecutorSchema = {};
const context: ExecutorContext = {
  root: '',
  cwd: process.cwd(),
  isVerbose: false,
};

describe('PreBumpRust Executor', () => {
  it('can run', async () => {
    const output = await executor(options, context);
    expect(output.success).toBe(true);
  });
});
