use anyhow::{Result, anyhow};

use crate::config::{Config, ConfigOption, save_config};
use crate::is_valid_address;
use crate::tables::{TableEntry, print_table};
use crate::terminal::{get_input, print_done, print_error};
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::process::exit;
use std::str::FromStr;
use std::time::Duration;

pub fn handle_open(
    args: &[&str],
    tcp: &mut Option<TcpStream>,
    connection: &mut String,
    config: &mut Config,
) -> Result<()> {
    if tcp.is_some() {
        return Err(anyhow!("you're already connected to another host."));
    }

    let address_input: String;
    let mut address_input_ref: &str = if args.len() == 1 {
        args[0]
    } else {
        address_input = get_input("address");
        &address_input
    };

    if address_input_ref.is_empty() && config.recent_connection.is_empty() {
        return Err(anyhow!("address is empty."));
    }

    if address_input_ref.is_empty() {
        if config.recent_connection.is_empty() {
            return Err(anyhow!("no recent connection available."));
        }
        address_input_ref = &config.recent_connection;
    }

    if !is_valid_address(address_input_ref) {
        return Err(anyhow!("invalid address format"));
    }

    println!("Connecting to {address_input_ref}...");
    let addr = SocketAddr::from_str(address_input_ref).unwrap();

    let tcp_stream =
        TcpStream::connect_timeout(&addr, Duration::from_millis(config.connection_timeout));
    if tcp_stream.is_err() {
        return Err(anyhow!("failed to connect to server"));
    }

    connection.clear();
    connection.push_str(address_input_ref);
    *tcp = Some(tcp_stream.unwrap());
    config.recent_connection = address_input_ref.to_string();
    print_done("Connection established");
    Ok(())
}

pub fn handle_send(args: &[&str], tcp: &mut Option<TcpStream>, config: &mut Config) -> Result<()> {
    if tcp.is_none() {
        return Err(anyhow!(
            "Connection not established. Use 'open' command first."
        ));
    }

    let message_input: String;
    let message_input_ref: &str = if !args.is_empty() {
        message_input = args.join(" ");
        &message_input
    } else {
        message_input = get_input("message");
        &message_input
    };

    let mut stream = tcp.as_ref().unwrap();
    let result = stream.write(message_input_ref.as_bytes());
    if result.is_err() {
        return Err(anyhow!("failed to send message"));
    }

    if !config.wait_for_response {
        print_done("Message sent successfully.");
        return Ok(());
    }

    let mut buf = vec![0u8; 1024];
    let _ = stream.set_read_timeout(Some(Duration::from_millis(config.read_timeout)));
    let read_result = stream.read(&mut buf);
    if read_result.is_err() {
        return Err(anyhow!(
            "failed to read the response, but message was sent."
        ));
    }
    let n = read_result.unwrap();
    let response = String::from_utf8_lossy(&buf[..n]);
    println!("{response}");
    Ok(())
}

pub fn handle_close(tcp: &mut Option<TcpStream>, connection: &mut String) -> Result<()> {
    if tcp.is_none() {
        return Err(anyhow!("No active connection."));
    }

    let _ = tcp.as_ref().unwrap().shutdown(Shutdown::Both);
    *tcp = None;
    connection.clear();
    connection.push_str("relayx");
    print_done("Closed the connection.");
    Ok(())
}

pub fn handle_set(args: &[&str], config: &mut Config) -> Result<()> {
    if args.is_empty() {
        return Err(anyhow!("No arguments provided."));
    }

    if args.len() == 1 {
        let opt = args[0];
        match ConfigOption::parse(opt) {
            Some(option) => option.print(config),
            None => return Err(anyhow!("unknown option.")),
        }
        return Ok(());
    }

    if args.len() == 2 {
        let opt = args[0];
        let val = args[1];
        let option = match ConfigOption::parse(opt) {
            Some(option) => option,
            None => {
                print_error("unknown option.");
                return Err(anyhow!("unknown option"));
            }
        };

        if let Err(e) = option.set(config, val) {
            return Err(anyhow!("{}", e.to_string()));
        }
        if let Err(e) = save_config(config.clone()) {
            print_error(&e.to_string());
            return Err(anyhow!("{}", e.to_string()));
        }
        return Ok(());
    }

    return Err(anyhow!("too many arguments"));
}

pub fn handle_list(config: &mut Config) -> Result<()> {
    let commands = vec![
        TableEntry {
            name: "wait_for_response".to_string(),
            description: format!("{}", config.wait_for_response),
        },
        TableEntry {
            name: "read_timeout".to_string(),
            description: format!("{} milliseconds", config.read_timeout),
        },
        TableEntry {
            name: "connection_timeout".to_string(),
            description: format!("{} milliseconds", config.connection_timeout),
        },
    ];

    println!("\n{}\n", print_table(commands));
    Ok(())
}

pub fn handle_clear() -> Result<()> {
    print!("\x1B[2J\x1B[H");
    Ok(())
}

pub fn handle_help() -> Result<()> {
    let commands = vec![
        TableEntry {
            name: "open, o".to_string(),
            description: "Open a new TCP connection".to_string(),
        },
        TableEntry {
            name: "send, s".to_string(),
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
            name: "list, ls".to_string(),
            description: "List current configuration.".to_string(),
        },
        TableEntry {
            name: "clear".to_string(),
            description: "Clear the console screen".to_string(),
        },
        TableEntry {
            name: "exit".to_string(),
            description: "Exit Relayx".to_string(),
        },
    ];
    println!("\n{}\n", print_table(commands));
    Ok(())
}
