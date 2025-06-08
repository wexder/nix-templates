use anyhow::{Result, anyhow};
use iocraft::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;

mod nix;
mod ui;

#[derive(Serialize, Deserialize, Debug)]
struct Templates {
    templates: HashMap<String, Value>,
}

impl Templates {
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
    let expr = match env::args().skip(1).next() {
        Some(e) => e,
        None => {
            return Err(anyhow!(
                "Invalid argument count\nExample usage: devish github:wexder/nix-templates#"
            ));
        }
    };

    let templates: Templates = serde_json::from_str(&nix::nix_eval(&expr))?;

    let mut selected = String::new();
    smol::block_on(
        element!(ui::template_selector::List(templates: templates.to_templates(), output: &mut selected) {  })
            .render_loop(),
    )
    .unwrap();

    // TODO better logger
    println!("Initializing flake using template \"{}\"", selected);
    nix::nix_init_template(expr.as_str(), selected.as_str())
}
