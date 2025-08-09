use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::env::var;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub wait_for_response: bool,
    pub read_timeout: u64,
    pub connection_timeout: u64,
    pub recent_connection: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            wait_for_response: true,
            read_timeout: 10000,
            connection_timeout: 10000,
            recent_connection: String::new(),
        }
    }
}

pub enum ConfigOption {
    WaitForResponse,
    ReadTimeout,
    ConnectionTimeout,
}

impl ConfigOption {
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "wait_for_response" => Some(Self::WaitForResponse),
            "read_timeout" => Some(Self::ReadTimeout),
            "connection_timeout" => Some(Self::ConnectionTimeout),
            _ => None,
        }
    }

    pub fn print(&self, cfg: &Config) {
        match self {
            Self::WaitForResponse => println!(
                "\x1b[3m\x1b[1mwait_for_response\x1b[0m\nCurrent: {}. Possible: true, false",
                cfg.wait_for_response
            ),
            Self::ReadTimeout => println!(
                "\x1b[3m\x1b[1mread_timeout\x1b[0m\nCurrent: {}.",
                cfg.read_timeout
            ),
            Self::ConnectionTimeout => println!(
                "\x1b[3m\x1b[1mconnection_timeout\x1b[0m\nCurrent: {}.",
                cfg.connection_timeout
            ),
        }
    }

    pub fn set(&self, cfg: &mut Config, val: &str) -> Result<()> {
        match self {
            Self::WaitForResponse => match val {
                "true" => {
                    cfg.wait_for_response = true;
                    Ok(())
                }
                "false" => {
                    cfg.wait_for_response = false;
                    Ok(())
                }
                _ => Err(anyhow!("invalid value for wait_for_response")),
            },
            Self::ReadTimeout => val
                .parse::<u64>()
                .map(|n| cfg.read_timeout = n)
                .map_err(|_| anyhow!("invalid value for read_timeout")),
            Self::ConnectionTimeout => val
                .parse::<u64>()
                .map(|n| cfg.connection_timeout = n)
                .map_err(|_| anyhow!("invalid value for connection_timeout")),
        }
    }
}

pub fn get_config_path() -> PathBuf {
    match std::env::consts::OS {
        "windows" => {
            let appdata = var("LOCALAPPDATA").unwrap();
            Path::new(&appdata).join("relayx-client.toml").to_path_buf()
        }
        _ => {
            let home_dir = var("HOME").unwrap();
            Path::new(&home_dir)
                .join(".config")
                .join("relayx-client.toml")
                .to_path_buf()
        }
    }
}

pub fn load_config() -> Result<Config> {
    let content = fs::read_to_string(get_config_path())
        .map_err(|e| anyhow!("failed to read configuration file content: {e}."))?;
    let config = toml::from_str::<Config>(&content)
        .map_err(|e| anyhow!("configuration file is broken: {e}."))?;
    Ok(config)
}

pub fn save_config(cfg: Config) -> Result<()> {
    let ctx_str = toml::to_string(&cfg)?;
    fs::write(get_config_path(), ctx_str)
        .map_err(|e| anyhow!("failed to save configuration: {}", e.to_string()))
}
