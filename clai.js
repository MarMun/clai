#!/usr/bin/env node

import buildAction from './actions/command.build.js';
import runAction from './actions/command.run.js';
import userChoice from './actions/user.choice.js';
// import explainAction from './actions/command.build.js';
// import copyAction from './actions/command.copy.js';

const args = process.args || process.argv;
const request = args.splice(2).join(' ');

// --- lifecycle ---
const command = await buildAction(request) 

while (true) {
  const choice = await userChoice.ask(command);

  if (choice === 'x') {
    await userChoice.end();
    break;
  }

  if (['', 'r'].includes(choice)) {
    await runAction(command)
    await userChoice.end();
    break;
  }

  if (['e'].includes(choice)) {
    // await explainAction(command)
    continue;
  }
}


