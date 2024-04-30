use std::process;
use std::process::Stdio;

use crate::{api, AlertLevel};

pub struct Command {
    pub level: AlertLevel,
    pub itself: String,
}

impl self::Command {
    pub async fn build(question: &String) -> self::Command {
        let prompt = builder(question);

        match api::call(&prompt, None).await {
            Ok(r) => {
                let mut suggestion = self::Command {
                    itself: r,
                    level: AlertLevel::Danger, // Default
                };
                suggestion.classify().await;
                suggestion
            }
            Err(e) => panic!("{e}"),
        }
    }

    pub async fn classify(&mut self) {
        let prompt = classifier(&self.itself);

        match api::call(&prompt, None).await {
            Ok(r) => {
                println!("classification: {r}");
                let level = AlertLevel::from_string(&r);
                println!("Found Level {level:?}");
                self.level = level;
            }
            Err(e) => panic!("{e}"),
        }
    }

    pub fn run(&self) {
        process::Command::new("sh")
            .arg("-c")
            .arg(&self.itself)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to execute command");
    }
}

fn builder(question: &String) -> String {
    format!(
        "
        You are a terminal command expert.

        Create the best command to use for this action: ${question}

        Don't return any explanation.
        Don't add '$' or other symbols in front of the command.
        Trim leading and following spaces.
        Only return the command itself.
        Your output must be directly executable in a shell.
        "
    )
}

// -- AI prompts ------
fn classifier(command: &String) -> String {
    format!(
        "
        You are a terminal command expert.
        You determin the alert level of a bash terminal command.

        The alert level tells if a command is:
        - Dangerous (AlertLevel = Danger)
        - Possibly Dangerous (AlertLevel = Warning)
        - Not Dangerous (AlertLevel = Safe)

        Determine the alert level of this command: {command}

        Don't return any explanation.
        Only return the AlertLevel (Danger, Warning, Safe) as one word.
        "
    )
}
