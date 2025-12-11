use colored::Colorize;
use indicatif::ProgressBar;
use std::io;
use std::io::Write;

use crate::macros::print_stdout;

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

pub fn get_input(msg: &str) -> String {
    if !msg.is_empty() {
        print_stdout!("( {msg} ): ");
    }
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).unwrap();
    temp.strip_suffix("\n").unwrap().trim().to_string()
}
