use std::{env, path::PathBuf, process::Command};

use anyhow::{Result, anyhow};

fn nix_binary_path() -> PathBuf {
    env::var("NIX_BINARY_PATH")
        .unwrap_or_else(|_| "nix".to_owned())
        .into()
}

pub fn nix_eval(expr: &str) -> String {
    let args = vec!["flake", "show", "--json", "--refresh"];

    let output = Command::new(nix_binary_path())
        .args(&args[..])
        .arg(expr)
        .output()
        .unwrap();
    if !output.status.success() {
        panic!(
            "flake show {expr} failed!\n    stdout: {}\n    stderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    }

    String::from_utf8(output.stdout).unwrap()
}

pub fn nix_init_template(expr: &str, template: &str) -> Result<()> {
    let args = vec!["flake", "init", "--refresh", "--template"];

    let t = format!("{}#{}", expr.trim_end_matches("#"), template);

    let output = Command::new(nix_binary_path())
        .args(&args[..])
        .arg(&t)
        .output()
        .unwrap();

    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            "flake init --template {t} failed!\n    stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
