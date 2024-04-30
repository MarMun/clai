mod actions;
mod api;

use std::env;

use actions::user::*;
use actions::*;

#[tokio::main]
async fn main() {
    // get question from args
    let args: Vec<String> = env::args().collect();
    let question = args[1..].join(" ");

    // create command
    let command = build::run(&question).await;

    // present command
    user::tell(UserMessage {
        message_type: MessageType::Neutral,
        body: command.to_string(),
    });

    loop {
        // get user choice
        let choice = user::ask().expect("Couldn't read choice");

        // valid user inputs
        match choice {
            UserChoice::Abort => {
                break;
            }
            UserChoice::Run => {
                execute::run(&command);
                break;
            }
            UserChoice::Clip => {
                clipboard::put(&command);
                break;
            }
            UserChoice::Explain => {
                explain::show(&command);
                continue;
            }
            UserChoice::Invalid => {
                continue;
            }
        }
    }
}
