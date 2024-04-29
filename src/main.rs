mod actions;
mod api;

use console::Term;
use std::env;

use crate::actions::user::UserChoice;

#[tokio::main]
async fn main() {
    // get question from args
    let args: Vec<String> = env::args().collect();
    let question = args[1..].join(" ");

    // create command
    let command = actions::build::run(&question).await;

    // present command
    let term = Term::stdout();
    term.write_line(&command).expect("Couldn't present command");

    loop {
        // get user choice
        let choice = actions::user::ask().expect("Couldn't read choice");

        // valid user inputs
        match choice {
            UserChoice::Abort => {
                break;
            }
            UserChoice::Run => {
                actions::execute::run(&command);
                break;
            }
            UserChoice::Clip => {
                actions::clipboard::put(&command);
                break;
            }
            UserChoice::Explain => {
                actions::explain::get(&command);
                continue;
            }
            UserChoice::Invalid => {
                continue;
            }
        }
    }
}
