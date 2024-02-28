import api from '../api.js'
import chalk from 'chalk'

function terminalNerd (question) {
  return `
    You are a terminal command expert.

    Create the best command to use for this action: ${question}

    Don't return any explanation.
    Only return the command itself.
    Your output must be directly executable in a shell.
  `
}

function run (question) {
  return new Promise(async (resolve, reject) => {
    // Call llm ---
    let answer = await api.call(terminalNerd(question))

    // QA response
    answer = answer.replace(/\r?\n/g, '')
    answer = answer.trim()

    // resolve(command)
    resolve(answer)
  })
}

export default run
