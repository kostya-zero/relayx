use crate::config::{Config, ConfigOption, save_config};
use crate::is_valid_address;
use crate::tables::{TableEntry, print_table};
use crate::terminal::{get_input, printerr};
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
) {
    if tcp.is_some() {
        printerr("you're already connected to another host.")
    }

    let address_input: String;
    let address_input_ref: &str = if args.len() == 1 {
        args[0]
    } else {
        address_input = get_input("address");
        &address_input
    };

    if address_input_ref.is_empty() {
        printerr("address is empty.");
        return;
    }

    if !is_valid_address(address_input_ref) {
        printerr("given address is not a valid IP address.");
        return;
    }

    println!("Connecting to {address_input_ref}...");
    let addr = SocketAddr::from_str(address_input_ref).unwrap();

    let tcp_stream =
        TcpStream::connect_timeout(&addr, Duration::from_millis(config.connection_timeout));
    if tcp_stream.is_err() {
        printerr("couldn't establish connection with server.");
        return;
    }

    connection.clear();
    connection.push_str(address_input_ref);
    *tcp = Some(tcp_stream.unwrap());
    println!("Connected successfully.");
}

pub fn handle_send(args: &[&str], tcp: &mut Option<TcpStream>, config: &mut Config) {
    if tcp.is_none() {
        println!("Connection is not established.");
        return;
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
        printerr("failed to send message to TCP stream.");
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
        printerr("failed to read the response, but message was sent.");
        return;
    }
    let n = read_result.unwrap();
    let response = String::from_utf8_lossy(&buf[..n]);
    println!("{response}");
}

pub fn handle_close(tcp: &mut Option<TcpStream>, connection: &mut String) {
    if tcp.is_none() {
        println!("No active connection.");
        return;
    }

    let _ = tcp.as_ref().unwrap().shutdown(Shutdown::Both);
    *tcp = None;
    connection.clear();
    connection.push_str("relayx");
    println!("Closed the connection.");
}

pub fn handle_set(args: &[&str], config: &mut Config) {
    if args.is_empty() {
        println!("Nothing to set. Type \x1b[1mlist\x1b[0m for options to set.");
        return;
    }

    if args.len() == 1 {
        let opt = args[0];
        match ConfigOption::parse(opt) {
            Some(option) => option.print(config),
            None => printerr("unknown option."),
        }
        return;
    }

    if args.len() == 2 {
        let opt = args[0];
        let val = args[1];
        let option = match ConfigOption::parse(opt) {
            Some(option) => option,
            None => {
                printerr("unknown option.");
                return;
            }
        };

        if let Err(e) = option.set(config, val) {
            printerr(&e.to_string());
            return;
        }
        if let Err(e) = save_config(config.clone()) {
            printerr(&e.to_string());
        }
        return;
    }

    printerr("Too many arguments.");
}

pub fn handle_list(config: &mut Config) {
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
}

pub fn handle_clear() {
    print!("\x1B[2J\x1B[H");
}

pub fn handle_exit(tcp: &mut Option<TcpStream>) {
    if tcp.is_some() {
        println!("Shutting down current connection...");
        let raw_tcp = tcp.as_ref().unwrap();
        let _ = raw_tcp.shutdown(Shutdown::Both);
    }
    exit(0);
}

pub fn handle_help() {
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
}

