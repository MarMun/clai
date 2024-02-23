import api from '../api.js';

const instructions = `
    You are a terminal command expert.
    You only return the command itself.
    You don't return any explanation.
    Just return plain text command.
    Your output must be directly executable in a shell.

    Create the best command to use for this action:
  `
;

function run (question) {
  return new Promise(async (resolve, reject) => {
    // Call llm ---
    const response = await api.call(`${instructions} ${question}`);
 
    // QA response ---
    // remove code block formatting
    let command = response.replace(/\`{3}.*\n?$/gm, '');
    // remove new lines
    command = command.replace(/\r?\n/g, '');

    resolve(command);
  });
}

export default run;
