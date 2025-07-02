use anyhow::{Context, Result};
use iocraft::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

mod cfg;
mod commands;
mod ui;

#[derive(Serialize, Deserialize, Debug)]
struct NixTemplates {
    templates: HashMap<String, Value>,
}

impl NixTemplates {
    fn to_templates(self: Self) -> Vec<ui::template_selector::Template> {
        self.templates
            .iter()
            .map(|(k, v)| {
                if !v.is_object() {
                    panic!("Expected {} to be object", k)
                }
                let desc = v
                    .get("description")
                    .map(|d| match d {
                        Value::String(s) => s.to_string(),
                        _ => todo!(),
                    })
                    .unwrap_or(String::new());
                ui::template_selector::Template {
                    name: k.clone(),
                    description: desc.to_string(),
                }
            })
            .collect()
    }
}

fn main() -> Result<()> {
    let mut args = cfg::parse_args();
    match cfg::load_cfg() {
        Some(config) => {
            args.repos.extend(config.repositories.into_iter());
        }
        _ => {}
    };

    let templates: Vec<ui::template_selector::Repo> = args
        .repos
        .iter()
        .map(|expr| {
            let templates = match serde_json::from_str::<NixTemplates>(&commands::nix_eval(expr)) {
                Ok(json) => json.to_templates(),
                Err(_) => Vec::new(),
            };
            ui::template_selector::Repo {
                expr: expr.clone(),
                templates: templates,
            }
        })
        .collect();

    let mut selected = ui::template_selector::Selected::default();
    smol::block_on(
        element!(ui::template_selector::List(repositories: templates, output: &mut selected) {  })
            .render_loop(),
    )
    .unwrap();

    if selected.expr == "" || selected.name == "" {
        return Ok(());
    }

    // TODO better logger
    println!(
        "Initializing flake using template \"{}\" \"{}\"",
        selected.expr, selected.name
    );
    if !args.dry {
        commands::nix_init_template(selected.expr.as_str(), selected.name.as_str())
            .context("Failed to init template")?
    }

    if args.git && !args.dry {
        commands::init_git().context("Failed to init git repository")?
    }

    Ok(())
}
