use anyhow::{Result, anyhow};

use crate::config::{Config, ConfigOption, save_config};
use crate::is_valid_address;
use crate::tables::{TableEntry, print_table};
use crate::terminal::{get_input, get_progress_bar, print_done};
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;

pub fn handle_open(
    args: &[&str],
    tcp: &mut Option<TcpStream>,
    connection: &mut String,
    config: &mut Config,
) -> Result<()> {
    if tcp.is_some() {
        return Err(anyhow!("You're already connected to another host."));
    }

    let address_input: String;
    let mut address_input_ref: &str = if args.len() == 1 {
        args[0]
    } else {
        address_input = get_input("address");
        &address_input
    };

    if address_input_ref.is_empty() && config.recent_connection.is_empty() {
        return Err(anyhow!("Address is empty."));
    }

    if address_input_ref.is_empty() {
        if config.recent_connection.is_empty() {
            return Err(anyhow!("No recent connection available."));
        }
        address_input_ref = &config.recent_connection;
    }

    if !is_valid_address(address_input_ref) {
        return Err(anyhow!("Invalid address format"));
    }

    let progress = get_progress_bar();
    progress.set_message("Connecting...");
    progress.enable_steady_tick(Duration::from_millis(100));
    let addr = SocketAddr::from_str(address_input_ref).unwrap();

    let tcp_stream =
        TcpStream::connect_timeout(&addr, Duration::from_millis(config.connection_timeout));
    if tcp_stream.is_err() {
        progress.finish_and_clear();
        return Err(anyhow!("Failed to connect to server"));
    }

    progress.finish_and_clear();
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
    let progress = get_progress_bar();
    progress.set_message("Sending...");
    progress.enable_steady_tick(Duration::from_millis(100));
    let result = stream.write(message_input_ref.as_bytes());
    if result.is_err() {
        progress.finish_and_clear();
        return Err(anyhow!("Failed to send message"));
    }

    if !config.wait_for_response {
        progress.finish_and_clear();
        print_done("Message sent successfully.");
        return Ok(());
    }

    progress.set_message("Receiving...");
    let mut buf = vec![0u8; 1024];
    let _ = stream.set_read_timeout(Some(Duration::from_millis(config.read_timeout)));
    let read_result = stream.read(&mut buf);
    if read_result.is_err() {
        progress.finish_and_clear();
        return Err(anyhow!(
            "failed to read the response, but message was sent."
        ));
    }
    let n = read_result.unwrap();
    let response = String::from_utf8_lossy(&buf[..n]);

    progress.finish_and_clear();
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
            None => return Err(anyhow!("Unknown option.")),
        }
        return Ok(());
    }

    if args.len() == 2 {
        let opt = args[0];
        let val = args[1];
        let option = match ConfigOption::parse(opt) {
            Some(option) => option,
            None => {
                return Err(anyhow!("Unknown option"));
            }
        };

        if let Err(e) = option.set(config, val) {
            return Err(anyhow!("{e}"));
        }
        if let Err(e) = save_config(config.clone()) {
            return Err(anyhow!("{e}"));
        }
        return Ok(());
    }

    Err(anyhow!("Too many arguments"))
}

pub fn handle_list(config: &mut Config) -> Result<()> {
    let wait_for_response_desc = &format!("{}", config.wait_for_response);
    let read_timeout_desc = &format!("{} milliseconds", config.read_timeout);
    let connection_timeout_desc = &format!("{} milliseconds", config.connection_timeout);
    let commands = vec![
        TableEntry {
            name: "wait_for_response",
            description: wait_for_response_desc,
        },
        TableEntry {
            name: "read_timeout",
            description: read_timeout_desc,
        },
        TableEntry {
            name: "connection_timeout",
            description: connection_timeout_desc,
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
            name: "open, o",
            description: "Open a new TCP connection",
        },
        TableEntry {
            name: "send, s",
            description: "Send a message",
        },
        TableEntry {
            name: "close",
            description: "Close current connection",
        },
        TableEntry {
            name: "set",
            description: "Set configuration options",
        },
        TableEntry {
            name: "list, ls",
            description: "List current configuration.",
        },
        TableEntry {
            name: "clear",
            description: "Clear the console screen",
        },
        TableEntry {
            name: "exit",
            description: "Exit Relayx",
        },
    ];
    println!("\n{}\n", print_table(commands));
    Ok(())
}
