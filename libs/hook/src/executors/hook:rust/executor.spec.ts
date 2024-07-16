import { ExecutorContext } from '@nx/devkit';

import { HookRustExecutorSchema } from './schema';
import executor from './executor';

const options: HookRustExecutorSchema = {};
const context: ExecutorContext = {
  root: '',
  cwd: process.cwd(),
  isVerbose: false,
};

describe('HookRust Executor', () => {
  it('can run', async () => {
    const output = await executor(options, context);
    expect(output.success).toBe(true);
  });
});
