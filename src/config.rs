use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::env::var;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub wait_for_response: bool,
    pub read_timeout: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            wait_for_response: true,
            read_timeout: 10000,
        }
    }
}

pub enum ConfigOption {
    WaitForResponse,
    ReadTimeout,
}

impl ConfigOption {
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "wait_for_response" => Some(Self::WaitForResponse),
            "read_timeout" => Some(Self::ReadTimeout),
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
        }
    }
}

fn get_config_path() -> PathBuf {
    match std::env::consts::OS {
        "windows" => {
            let appdata = var("LOCALAPPDATA").unwrap();
            Path::new(&appdata)
                .join("wire-client.toml")
                .to_path_buf()
        }
        _ => {
            let home_dir = var("HOME").unwrap();
            Path::new(&home_dir)
                .join(".config")
                .join("wire-client.toml")
                .to_path_buf()
        }
    }
}

pub fn load_config() -> Config {
    if let Ok(content) = fs::read_to_string(get_config_path()) {
        toml::from_str::<Config>(&content).unwrap()
    } else {
        Config::default()
    }
}

pub fn save_config(cfg: Config) -> Result<()> {
    let ctx_str = toml::to_string(&cfg)?;
    fs::write(get_config_path(), ctx_str)
        .map_err(|e| anyhow!("Failed to save context: {}", e.to_string()))
}
