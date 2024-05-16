use console::{style, StyledObject, Term};

use crate::AlertLevel;

pub struct UserMessage {
    pub level: AlertLevel,
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

pub fn ask() -> UserChoice {
    let term = Term::stdout();

    // Add options ui
    let options = ask_options();
    match term.write_line(&options) {
        Ok(v) => v,
        Err(e) => panic!("{e}"),
    };
    // Read single char from stdin
    let choice = match term.read_char() {
        Ok(v) => v,
        Err(e) => panic!("{e}"),
    };
    // Remove ui from stdin
    let _ = term.clear_last_lines(1);

    // Cast char to str
    // (we can't use char in match)
    let choice = choice.to_string();
    let choice = choice.as_str();

    // Match input
    match choice {
        "\n" | "r" => UserChoice::Run,
        "c" => UserChoice::Clip,
        "e" => UserChoice::Explain,
        "a" => UserChoice::Abort,
        _ => UserChoice::Invalid,
    }
}

fn icon_create(icon: &str) -> StyledObject<String> {
    style(icon.to_string()).for_stdout()
}

pub fn tell(message: UserMessage) {
    let term = Term::stdout();

    let icon = match message.level {
        AlertLevel::Safe => icon_create("+").green(),
        AlertLevel::Danger => icon_create("!").red(),
        AlertLevel::Warning => icon_create("X").yellow(),
        AlertLevel::Neutral => icon_create(">").dim(),
    };

    let output = format!("{icon} {0}", message.body);
    let output = output.as_str();

    term.write_line(output).expect("Error");
}
