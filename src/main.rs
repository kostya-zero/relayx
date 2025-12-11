use crate::commands::*;
use crate::config::{Config, get_config_path, load_config, save_config};
use crate::macros::print_stdout;
use crate::terminal::{get_input, print_warn};
use anyhow::{Result, anyhow};
use colored::Colorize;
use std::io::Write;
use std::net::{Shutdown, TcpStream, ToSocketAddrs};
use std::path::Path;
use std::process::exit;
use terminal::print_error;

mod commands;
mod config;
mod macros;
mod tables;
mod terminal;

fn is_valid_address(s: &str) -> bool {
    s.to_socket_addrs().is_ok()
}

fn check_env() -> Result<()> {
    if !Path::new(&get_config_path()).exists() {
        let default_config = Config::default();
        save_config(&default_config)?
    }
    Ok(())
}

fn main() {
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

    let program_title = format!("Relayx {}", env!("CARGO_PKG_VERSION"));
    println!(
        "{}\nEnter ?/help to display help message.",
        program_title.bold()
    );

    loop {
        print_stdout!("\x1b[1m{connection}>\x1b[0m ");
        let input = get_input("");
        if input.is_empty() {
            continue;
        }
        if let Err(e) = process_input(&input, &mut connection, &mut tcp, &mut config) {
            print_error(&e.to_string());
        }
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
) -> Result<()> {
    let (cmd, args) = parse_command(input);

    if cmd.to_ascii_lowercase().as_str() == "exit" {
        if let Some(stream) = tcp.take() {
            println!("Shutting down current connection...");
            let _ = stream.shutdown(Shutdown::Both);
        }
        exit(0);
    }

    match cmd.to_ascii_lowercase().as_str() {
        "open" | "o" => handle_open(&args, tcp, connection, config),
        "send" | "s" => handle_send(&args, tcp, config),
        "close" => handle_close(tcp, connection),
        "set" => handle_set(&args, config),
        "list" | "ls" => handle_list(config),
        "clear" => handle_clear(),
        "help" | "?" => handle_help(),
        "" => Ok(()),
        _ => Err(anyhow!("Unknown command: {cmd}")),
    }
}
