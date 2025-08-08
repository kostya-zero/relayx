use crate::commands::*;
use crate::config::{Config, get_config_path, load_config, save_config};
use crate::terminal::{get_input, print_warn};
use anyhow::Result;
use std::net::{SocketAddrV4, TcpStream};
use std::path::Path;
use terminal::print_error;

mod commands;
mod config;
mod tables;
mod terminal;

fn is_valid_address(s: &str) -> bool {
    s.parse::<SocketAddrV4>().is_ok()
}

fn check_env() -> Result<()> {
    if !Path::new(&get_config_path()).exists() {
        let default_config = Config::default();
        save_config(default_config)?
    }
    Ok(())
}

fn main() {
    println!(
        "\x1b[1mRelayx {}\x1b[0m\nEnter ?/help to display help message.",
        env!("CARGO_PKG_VERSION")
    );
    if check_env().is_err() {
        print_warn("could not generate default configuration due to file system error.");
    }
    let mut connection = String::from("relayx");
    let mut tcp: Option<TcpStream> = None;
    let mut config = load_config().unwrap_or_else(|e| {
        print_error(&format!("cant load your configuration: {e}."));
        print_warn("using default configuration instead.");
        Config::default()
    });
    loop {
        print!("\x1b[1m{connection}>\x1b[0m ");
        let input = get_input("");
        process_input(&input, &mut connection, &mut tcp, &mut config);
    }
}

fn parse_command(input: &str) -> (&str, Vec<&str>) {
    let mut parts = input.split_whitespace();
    let cmd = parts.next().unwrap_or("");
    let args: Vec<&str> = parts.collect();
    (cmd, args)
}

fn process_input(
    input: &str,
    connection: &mut String,
    tcp: &mut Option<TcpStream>,
    config: &mut Config,
) {
    let (cmd, args) = parse_command(input);

    match cmd.to_ascii_lowercase().as_str() {
        "open" | "o" => handle_open(&args, tcp, connection, config),
        "send" | "s" => handle_send(&args, tcp, config),
        "close" => handle_close(tcp, connection),
        "set" => handle_set(&args, config),
        "list" | "ls" => handle_list(config),
        "clear" => handle_clear(),
        "exit" => handle_exit(tcp),
        "help" | "?" => handle_help(),
        _ => println!("Unknown command: {cmd}"),
    }
}
