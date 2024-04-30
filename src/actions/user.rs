use console::{style, StyledObject, Term};
use std::error::Error;

pub enum MessageType {
    Danger,
    Warning,
    Save,
    Neutral,
}

pub struct UserMessage {
    pub message_type: MessageType,
    pub body: String,
}

pub enum UserChoice {
    Run,
    Explain,
    Clip,
    Abort,
    Invalid,
}

fn ask_options() -> String {
    // Run
    let icon = style("󰌑").for_stdout().color256(192);
    let post = style("run").for_stdout().color256(192);
    let run = format!("{icon} {post}");

    // Explain
    let explain_style = style("e(x)plain").for_stdout().color256(64);
    let explain = format!("{explain_style}");

    // Clipboard
    let clip_style = style("(c)lipboard").for_stdout().color256(64);
    let clip = format!("{clip_style}");

    // Abort
    let abort_style = style("(a)bort").for_stdout().color256(64);
    let abort = format!("{abort_style}");

    format!("{run} | {explain} | {clip} | {abort}")
}

pub fn ask() -> Result<UserChoice, Box<dyn Error>> {
    let term = Term::stdout();

    // Add options ui
    let options = ask_options();
    term.write_line(&options)?;
    // Read single char from stdin
    let choice = term.read_char()?;
    // Remove ui from stdin
    let _ = term.clear_last_lines(1);

    // Cast char to str
    // (we can't use char in match)
    let choice = choice.to_string();
    let choice = choice.as_str();

    // Match input
    let choice = match choice {
        "\n" | "r" => UserChoice::Run,
        "c" => UserChoice::Clip,
        "e" => UserChoice::Explain,
        "a" => UserChoice::Abort,
        _ => UserChoice::Invalid,
    };

    Ok(choice)
}

fn icon_create(icon: String) -> StyledObject<String> {
    style(icon).for_stdout()
}

pub fn tell(message: UserMessage) {
    let term = Term::stdout();

    let icon = match message.message_type {
        MessageType::Save => icon_create("+".to_string()).green(),
        MessageType::Danger => icon_create("!".to_string()).red(),
        MessageType::Warning => icon_create("X".to_string()).yellow(),
        MessageType::Neutral => icon_create(">".to_string()).dim(),
    };

    let output = format!("{icon} {0}", message.body);
    let output = output.as_str();

    term.write_line(output).expect("Error");
}
