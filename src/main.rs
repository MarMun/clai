mod actions;
mod api;

use console::Term;
use std::env;

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
        let choice = actions::choice::ask().expect("Couldn't read choice");
        let choice = choice.as_str();

        // valid user inputs
        if ["\n", "r"].contains(&choice) {
            actions::execute::run(&command);
            break;
        }

        if choice == "c" {
            actions::clipboard::put(&command);
            break;
        }

        if choice == "a" {
            break;
        }

        if choice == "e" {
            actions::explain::get(&command);
        }

        // any other user input
        continue;
    }
}
