import { PromiseExecutor } from '@nx/devkit';
import { HookVueExecutorSchema } from './schema';
import { exec } from 'child_process';
import { promisify } from 'util';

const runExecutor: PromiseExecutor<HookVueExecutorSchema> = async (options) => {
  console.info('Running Vue hook for', options['target-dir']);
  const hookCommand = ['nx lint front', 'yarn audit'];
  const command = hookCommand.join(' && ');
  return await promisify(exec)(command)
    .then((result) => {
      console.log(result.stdout);
      console.log(result.stderr);
      return {
        success: true,
      };
    })
    .catch((err) => {
      console.error('Error running command', command);
      console.error(err);
      return {
        success: false,
      };
    });
};

export default runExecutor;
