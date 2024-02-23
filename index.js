#!/usr/bin/env node

import { exec } from 'child_process';
import chalk from 'chalk';
import readline from 'readline'

import buildAction from './actions/command.build.js';
// import explainAction from 'actions/command.build.js';
// import copyAction from 'actions/command.copy.js';
// import runAction from 'actions/command.run.js';
// ...

// --- lifecycle ---
let running = true;
async function main() {
  while (running) {
    const request = await userInput();
    console.log('Request: ', request);
    const command = await buildAction(request) 
    console.log('Command: ', command);
    const choice = await userChoice();
    console.log('Choice: ', choice);

    if (choice.toLowerCase() === 'x') {
      running = false;
    }
  }

  shell.close();
}

// user interaction
const shell = readline.createInterface({
  input: process.stdin,
  output: process.stdout
})

function userInput(question) {
  return new Promise((resolve) => {
    shell.question(chalk.red('> '), resolve);
  });
}

function userChoice(question) {
  const actions = `${chalk.red('R')} | ${chalk.green('E')} | ${chalk.green('C')} | ${chalk.yellow('A')} | ${chalk.grey('X')}`;
  return new Promise((resolve) => {
    shell.question(actions, resolve);
  });
}

// start heartbeat
await main();

