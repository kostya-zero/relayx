use crate::config::{get_config_path, load_config, save_config, Config, ConfigOption};
use crate::tables::{print_table, TableEntry};
use crate::terminal::{printerr, printwarn};
use anyhow::Result;
use std::io;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddrV4, TcpStream};
use std::path::Path;
use std::process::exit;
use std::time::Duration;

mod config;
mod tables;
mod terminal;

fn is_valid_address(s: &str) -> bool {
    s.parse::<SocketAddrV4>().is_ok()
}

fn sanitize_input(input: String) -> String {
    let cloned_input = input.clone();
    let sanitized = cloned_input.strip_suffix("\n").unwrap().trim();
    sanitized.to_string()
}

fn check_env() -> Result<()> {
    if !Path::new(&get_config_path()).exists() {
        let default_config = Config::default();
        save_config(default_config)?
    }
    Ok(())
}

fn main() {
    println!("Relayx TCP Client {}", env!("CARGO_PKG_VERSION"));
    println!("Enter ?/help to display help message.");
    if let Err(e) = check_env() {
        printwarn(&format!("failed to generate default configuration: {e}"));
    }
    let mut connection = String::from("relayx");
    let mut tcp: Option<TcpStream> = None;
    let mut config = load_config().unwrap_or_else(|e| {
        printwarn(&format!(
            "cant load your configuration: {e} using default instead."
        ));
        Config::default()
    });
    loop {
        print!("\x1b[1m{connection}>\x1b[0m ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = sanitize_input(input);
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
        "open" => {
            let address_input: String;
            let address_input_ref: &str = if args.len() == 1 {
                args[0]
            } else {
                print!("(address): ");
                io::stdout().flush().unwrap();
                let mut temp = String::new();
                io::stdin().read_line(&mut temp).unwrap();
                address_input = sanitize_input(temp);
                &address_input
            };
            if address_input_ref.is_empty() {
                println!("Address is empty.");
                return;
            }
            if !is_valid_address(address_input_ref) {
                println!("Given address is not a valid IP address.");
                return;
            }
            println!("Connecting to {address_input_ref}...");
            let tcp_stream = TcpStream::connect(address_input_ref);
            if let Err(_) = tcp_stream {
                println!("Couldn't establish connection with server.");
                return;
            }
            connection.clear();
            connection.push_str(address_input_ref);
            *tcp = Some(tcp_stream.unwrap());
            println!("Connected successfully.");
        }
        "send" => {
            if tcp.is_none() {
                println!("Connection is not established.");
                return;
            }

            let message_input: String;
            let message_input_ref: &str = if !args.is_empty() {
                message_input = args.join(" ");
                &message_input
            } else {
                print!("(message): ");
                io::stdout().flush().unwrap();
                let mut temp = String::new();
                io::stdin().read_line(&mut temp).unwrap();
                message_input = sanitize_input(temp);
                &message_input
            };

            let mut stream = tcp.as_ref().unwrap();
            let result = stream.write(message_input_ref.as_bytes());
            if result.is_err() {
                println!("Failed to send message to TCP stream.");
                return;
            }

            if !config.wait_for_response {
                println!("Sent.");
                return;
            }

            let mut buf = vec![0u8; 1024];
            let _ = stream.set_read_timeout(Some(Duration::from_millis(config.read_timeout)));
            let read_result = stream.read(&mut buf);
            if read_result.is_err() {
                println!("Failed to read the response, but message was sent.");
                return;
            }
            let n = read_result.unwrap();
            let response = String::from_utf8_lossy(&buf[..n]);
            println!("{response}");
        }
        "close" => {
            if tcp.is_none() {
                println!("No active connection.");
                return;
            }

            let _ = tcp.as_ref().unwrap().shutdown(Shutdown::Both);
            connection.clear();
            connection.push_str("relayx");
            println!("Closed the connection.")
        }
        "set" => match args.as_slice() {
            [] => {
                println!("Nothing to set. Type \x1b[1mlist\x1b[0m for options to set.");
            }
            [opt] => {
                if let Some(option) = ConfigOption::parse(opt) {
                    option.print(config);
                } else {
                    println!("unknown option.");
                }
            }
            [opt, val] => {
                if let Some(option) = ConfigOption::parse(opt) {
                    if let Err(e) = option.set(config, val) {
                        printerr(&e.to_string());
                        return;
                    }

                    if let Err(e) = save_config(config.clone()) {
                        printerr(&e.to_string());
                    }
                } else {
                    println!("unknown option.");
                }
            }
            _ => println!("Too many arguments."),
        },
        "list" => {
            let commands = vec![
                TableEntry {
                    name: "wait_for_response".to_string(),
                    description: format!("{}", config.wait_for_response),
                },
                TableEntry {
                    name: "read_timeout".to_string(),
                    description: format!("{} milliseconds", config.read_timeout),
                },
            ];

            println!("\n{}\n", print_table(commands));
        }
        "clear" => {
            print!("\x1B[2J\x1B[H");
        }
        "exit" => {
            if tcp.is_some() {
                let raw_tcp = tcp.as_ref().unwrap();
                println!("{}", raw_tcp.peer_addr().unwrap());
                let _ = raw_tcp.shutdown(Shutdown::Both);
            }
            exit(0);
        }
        "help" | "?" => {
            let commands = vec![
                TableEntry {
                    name: "open".to_string(),
                    description: "Open a new TCP connection".to_string(),
                },
                TableEntry {
                    name: "send".to_string(),
                    description: "Send a message".to_string(),
                },
                TableEntry {
                    name: "close".to_string(),
                    description: "Close current connection".to_string(),
                },
                TableEntry {
                    name: "set".to_string(),
                    description: "Set configuration options".to_string(),
                },
                TableEntry {
                    name: "list".to_string(),
                    description: "List current configuration.".to_string(),
                },
                TableEntry {
                    name: "clear".to_string(),
                    description: "Clear the console screen".to_string(),
                },
                TableEntry {
                    name: "exit".to_string(),
                    description: "Close Wire".to_string(),
                },
            ];
            println!("\n{}\n", print_table(commands));
        }
        _ => {
            println!("Unknown input: {input}");
        }
    }
}
