# Nix templates

My nix templates for dev shells.

## Devish
Small rust utility to select the template and init nix flake.

### Usage
Install it via nix flake or use directly via nix run.
Just pass your repo with nix flake templates.
```bash
devish github:wexder/nix-templates
# Or directly
nix run github:wexder/nix-templates?dir=devish github:wexder/nix-templates
```
