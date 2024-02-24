import api from '../api.js';
import chalk from 'chalk';

function terminalNerd (question) {
  return `
    You are a terminal command expert.

    Create the best command to use for this action: ${question}

    Don't return any explanation.
    Only return the command itself.
    Your output must be directly executable in a shell.
  `;
}

function commandCleaner (text) {
  return `
    Remove markdown formatting: ${text}
  `;
}

function run (question) {
  return new Promise(async (resolve, reject) => {
    // Call llm ---
    const answer = await api.call(terminalNerd(question));
    // const command = await api.call(commandCleaner(answer));
    // console.log('command', command);
    // QA response ---

    // remove new lines
    // command = command.replace(/\r?\n/g, '');

    // resolve(command);
    resolve(answer);
  });
}

export default run;
