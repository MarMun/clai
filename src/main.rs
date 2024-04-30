mod actions;
mod api;

use std::env;

use actions::command::*;
use actions::user::*;
use actions::*;

#[derive(Debug, Clone)]
pub enum AlertLevel {
    Danger,
    Warning,
    Safe,
    Neutral,
}

impl AlertLevel {
    pub fn from_string(level: &str) -> Self {
        match level.trim() {
            "Danger" => Self::Danger,
            "Warning" => Self::Warning,
            "Safe" => Self::Safe,
            "Neutral" => Self::Neutral,
            _ => panic!("Unknown Alert level"),
        }
    }
}

#[tokio::main]
async fn main() {
    // get question from args
    let args: Vec<String> = env::args().collect();
    let question = args[1..].join(" ");

    // create command
    let command = Command::build(&question).await;

    // present command
    // WTF: Why do I need to clone a enum??
    user::tell(UserMessage {
        level: command.level.clone(),
        body: command.itself.clone(),
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
                command.run();
                break;
            }
            UserChoice::Clip => {
                clipboard::put(&command.itself);
                break;
            }
            UserChoice::Explain => {
                explain::show(&command.itself);
                continue;
            }
            UserChoice::Invalid => {
                continue;
            }
        }
    }
}
