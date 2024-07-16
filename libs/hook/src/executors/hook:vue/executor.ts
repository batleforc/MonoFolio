import { PromiseExecutor } from '@nx/devkit';
import { HookVueExecutorSchema } from './schema';
import { exec } from 'child_process';
import { promisify } from 'util';

const runExecutor: PromiseExecutor<HookVueExecutorSchema> = async (options) => {
  console.info('Running Vue hook for', options['target-dir']);
  const hookCommand = [
    'nx lint front',
    'cd ' + options['target-dir'],
    'yarn audit',
  ];
  const command = hookCommand.join(' && ');
  await promisify(exec)(command).then((result) => {
    console.log(result.stdout);
    console.log(result.stderr);
  });
  return {
    success: true,
  };
};

export default runExecutor;
