import { spawn } from 'child_process';
import chalk from 'chalk';

function run (command) {
  return new Promise(async (resolve, reject) => {
    console.log(chalk.gray('---------'));

    const child = spawn(
      command , {
        shell: true,
        stdio: [null, process.stdout, process.stderr]
      }
    );

    child.on('close', (code) => { resolve(code); });
  });
}

export default run;
