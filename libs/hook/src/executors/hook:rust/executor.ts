import { PromiseExecutor, workspaceRoot } from '@nx/devkit';
import { HookRustExecutorSchema } from './schema';
import { exec } from 'child_process';
import { promisify } from 'util';

const runExecutor: PromiseExecutor<HookRustExecutorSchema> = async (
  options
) => {
  console.info('Running Rust hook for', options['target-dir']);
  const hookCommand = [
    `cd ${workspaceRoot}/${options['target-dir']}`,
    'cargo fmt -v --all --check',
    'cargo clippy',
    'gitleaks protect --verbose --redact --staged',
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
