import { ExecutorContext } from '@nx/devkit';

import { HookVueExecutorSchema } from './schema';
import executor from './executor';

const options: HookVueExecutorSchema = {};
const context: ExecutorContext = {
  root: '',
  cwd: process.cwd(),
  isVerbose: false,
};

describe('HookVue Executor', () => {
  it('can run', async () => {
    const output = await executor(options, context);
    expect(output.success).toBe(true);
  });
});
