import readline from 'readline'
import chalk from 'chalk'

// colors
const { green, gray, yellow, red, magenta } = chalk

// user options
function userOptions () {
  const div = gray(' | ')
  const icon_start = gray('↳ ')
  const icon_end = gray(' > ')
  const run = gray('run \u23CE')
  const explain = chalk.blue('e') + gray('xplain')
  const copy = green('c') + gray('lipboard')
  const end = gray('e') + yellow('x') + gray('it')

  // user prompt
  return icon_start +
    run + div +
    explain + div +
    copy + div +
    end + 
    icon_end
  
}

// shell 
const shell = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  error: process.stderr
})

// action
async function ask (question) {
  console.log(chalk.gray('-'))
  console.log(magenta('⏺ ') + question)

  return new Promise((resolve) => {
    shell.question(userOptions(), (choice) => {
      readline.moveCursor(process.stdout, 0, -1)
      readline.clearLine(process.stdout, 0)
      readline.cursorTo(process.stdout, 0)

      resolve(choice.toLowerCase())
    })
  })
}

async function end (question) {
  shell.close()
}

export default { ask, end }
