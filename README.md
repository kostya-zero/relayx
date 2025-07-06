# Relayx

Relayx is a TCP client with a shell-like interface, inspired by Microsoft's Telnet client. 
It allows you to open a TCP connection to a server, send messages, and view responses.

## Installation

#### With Cargo

```shell
cargo install relayx
```

#### From releases

Download the latest release from the [GitHub releases page](https://github.com/kostya-zero/relayx/releases) that matches your platform.

## Features

- Interactive shell interface.
- Connect to any TCP server.
- Send and receive data.
- Configurable options.

## Commands

Relayx uses a simple command-based interface. Here are the available commands:

| Command                | Description                                                                   |
|:-----------------------|:------------------------------------------------------------------------------|
| `open <address>`       | Opens a new TCP connection to the specified address (e.g., `127.0.0.1:8080`). |
| `send <message>`       | Sends a message over the active connection.                                   |
| `close`                | Closes the current TCP connection.                                            |
| `set <option> <value>` | Sets a configuration option.                                                  |
| `list`                 | Lists the current configuration options and their values.                     |
| `clear`                | Clears the console screen.                                                    |
| `help` or `?`          | Displays the help message with all available commands.                        |
| `exit`                 | Closes any active connection and exits Relayx.                                |

## Configuration

Relayx has a few configuration options that can be modified using the `set` command.

| Option               | Description                                                             |
|:---------------------|:------------------------------------------------------------------------|
| `wait_for_response`  | Whether to wait for a response from the server after sending a message. |
| `read_timeout`       | The time in milliseconds to wait for a response from the server.        |
| `connection_timeout` | The time in milliseconds to wait for a connection to be established.    |

## Building from Source

To build Relayx from the source code, you will need to have the Rust toolchain installed.

1.  Clone the repository:
    ```sh
    git clone https://github.com/kostya-zero/relayx.git
    ```
2.  Navigate to the project directory:
    ```sh
    cd relayx
    ```
3.  Build the project:
    ```sh
    cargo build --release
    ```
4.  Run the executable:
    ```sh
    ./target/release/relayx
    ```

## License

Relayx is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.