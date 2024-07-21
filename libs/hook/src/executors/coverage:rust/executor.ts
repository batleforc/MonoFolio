import { PromiseExecutor } from '@nx/devkit';
import { CoverageRustExecutorSchema } from './schema';
import { exec } from 'child_process';
import { promisify } from 'util';

const runExecutor: PromiseExecutor<CoverageRustExecutorSchema> = async (
  options
) => {
  console.info('Running Rust coverage for', options['target-dir']);
  const hookCommand = [`cd ${options['target-dir']}`, 'cargo llvm-cov'];
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
