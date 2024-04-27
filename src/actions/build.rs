use crate::api;

fn terminal_nerd(question: &String) -> String {
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

pub async fn run(question: &String) -> String {
    let prompt = terminal_nerd(question);

    match api::call(&prompt, None).await {
        Ok(r) => r,
        Err(e) => panic!("{e}"),
    }
}
