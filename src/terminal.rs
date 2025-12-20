use colored::Colorize;
use indicatif::ProgressBar;
use rustyline::completion::Completer;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::Validator;
use rustyline::{Editor, Helper};
use std::borrow::Cow;

pub fn print_error(msg: &str) {
    eprintln!(" {}: {msg}", "error".bold().red())
}

pub fn print_warn(msg: &str) {
    println!(" {}: {msg}", "warn".bold().yellow())
}

pub fn print_done(msg: &str) {
    println!(" {} {msg}", "✓".green().bold())
}

pub fn get_progress_bar() -> ProgressBar {
    ProgressBar::new_spinner().with_style(
        indicatif::ProgressStyle::with_template(" {spinner:.green} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    )
}

#[derive(Default)]
struct PromptHighlighter;

impl Highlighter for PromptHighlighter {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> Cow<'b, str> {
        Cow::Owned(format!("\x1b[1m{prompt}\x1b[0m"))
    }
}

impl Completer for PromptHighlighter {
    type Candidate = String;
}

impl Hinter for PromptHighlighter {
    type Hint = String;
}

impl Validator for PromptHighlighter {}

impl Helper for PromptHighlighter {}

pub struct Shell {
    editor: Editor<PromptHighlighter, DefaultHistory>,
}

impl Shell {
    pub fn new() -> Self {
        let mut editor = Editor::<PromptHighlighter, DefaultHistory>::new()
            .expect("failed to initialize line editor");
        editor.set_helper(Some(PromptHighlighter));
        Self { editor }
    }

    pub fn read_line(&mut self, prompt: &str) -> Option<String> {
        match self.editor.readline(prompt) {
            Ok(line) => {
                if !line.trim().is_empty() {
                    let _ = self.editor.add_history_entry(line.as_str());
                }
                Some(line)
            }
            Err(ReadlineError::Interrupted) => Some(String::new()),
            Err(ReadlineError::Eof) => None,
            Err(_) => Some(String::new()),
        }
    }
}

pub fn get_input(shell: &mut Shell, msg: &str) -> String {
    let prompt = if msg.is_empty() {
        String::new()
    } else {
        format!("( {msg} ): ")
    };
    shell
        .read_line(&prompt)
        .unwrap_or_default()
        .trim()
        .to_string()
}
