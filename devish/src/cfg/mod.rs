use std::env;

use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

/// Simple NIX template selector
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Nix flake urls for load from
    pub repos: Vec<String>,

    /// Initialize git repository
    #[arg(long)]
    pub git: bool,

    /// Just print output, don't run generation
    #[arg(long)]
    pub dry: bool,
}
pub fn parse_args() -> Args {
    let args = Args::parse();

    return args;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DevishConfig {
    pub repositories: Vec<String>,
}

// TODO use
pub fn load_cfg() -> Option<DevishConfig> {
    let mut home = match env::home_dir() {
        Some(path) => path,
        None => {
            println!("Impossible to get your home dir! Skipping config");
            return None;
        }
    };

    home.push(".config/devish/config");
    let home_path = match home.to_str() {
        Some(p) => p,
        None => {
            println!("Impossible to get your home dir! Skipping config");
            return None;
        }
    };

    let settings = match config::Config::builder()
        .add_source(config::File::with_name(home_path))
        .build()
    {
        Ok(settings) => settings,
        Err(_) => {
            return None;
        }
    };

    match settings
        .try_deserialize()
        .context("Cannot parse configuration")
    {
        Ok(settings) => settings,
        Err(e) => {
            println!(
                "Failed to deserialize config file. Skipping config. Err: {:?}",
                e
            );
            None
        }
    }
}
