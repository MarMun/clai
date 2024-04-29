use console::{style, Term};
use std::error::Error;

pub enum UserChoice {
    Run,
    Explain,
    Clip,
    Abort,
    Invalid,
}
fn options() -> String {
    // Run
    let icon = style("󰌑").for_stdout().color256(192);
    let post = style("run").for_stdout().color256(192);
    let run = format!("{icon} {post}");

    // Explain
    let explain_style = style("e(x)plain").for_stdout().color256(64);
    let explain = format!("{explain_style}");

    // clipboard
    let clip_style = style("(c)lipboard").for_stdout().color256(64);
    let clip = format!("{clip_style}");

    // clipboard
    let abort_style = style("(a)bort").for_stdout().color256(64);
    let abort = format!("{abort_style}");

    format!("{run} | {explain} | {clip} | {abort}")
}

pub fn ask() -> Result<UserChoice, Box<dyn Error>> {
    let term = Term::stdout();

    // Add options ui
    let options = options();
    term.write_line(&options)?;

    // Read args from stdin
    let choice = term.read_char()?;
    let choice = choice.to_string();
    let choice = choice.as_str();

    // Remove options ui
    let _ = term.clear_last_lines(1);

    let choice = match choice {
        "\n" | "r" => UserChoice::Run,
        "c" => UserChoice::Clip,
        "e" => UserChoice::Explain,
        "a" => UserChoice::Abort,
        _ => UserChoice::Invalid,
    };

    Ok(choice)
}
