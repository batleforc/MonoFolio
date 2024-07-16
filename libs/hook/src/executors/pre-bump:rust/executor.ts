import { PromiseExecutor } from '@nx/devkit';
import { PreBumpRustExecutorSchema } from './schema';
import { exec } from 'child_process';
import { promisify } from 'util';

const runExecutor: PromiseExecutor<PreBumpRustExecutorSchema> = async (
  options
) => {
  const version = process.env.RUST_BUMP_VERSION;
  console.info(
    'Running PreBump hook for',
    options['target-dir'],
    'in version',
    version
  );
  const hookCommand = [
    'cd ' + options['target-dir'],
    'echo ' + version,
    `cargo audit`,
    `cargo bump ${version}`,
    'git add Cargo.toml',
    `git commit -m "chore(version): set cargo version to ${version}"`,
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
