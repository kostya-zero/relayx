use std::io;
use std::io::Write;

use indicatif::ProgressBar;

pub fn print_error(msg: &str) {
    eprintln!(" \x1b[91m\x1b[1mError\x1b[0m: {msg}")
}

pub fn print_warn(msg: &str) {
    eprintln!(" \x1b[93m\x1b[1m⚠\x1b[0m {msg}")
}

pub fn print_done(msg: &str) {
    println!(" \x1b[92m\x1b[1m✓\x1b[0m {msg}")
}

pub fn get_progress_bar() -> ProgressBar {
    ProgressBar::new_spinner().with_style(
        indicatif::ProgressStyle::with_template(" {spinner:.green} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    )
}

pub fn get_input(msg: &str) -> String {
    if !msg.is_empty() {
        print!("( {msg} ): ");
    }
    io::stdout().flush().unwrap();
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).unwrap();
    temp.strip_suffix("\n").unwrap().trim().to_string()
}
