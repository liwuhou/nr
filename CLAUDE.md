# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**nr** is a CLI tool written in Rust that simplifies running npm scripts by automatically detecting the correct package manager (npm/yarn/pnpm) and providing script aliasing with interactive selection.

## Build & Development

```bash
# Build (debug)
cargo build

# Build (release)
cargo build --release

# Run tests
cargo test

# Run the CLI
cargo run -- <command>

# Install locally
cargo install --path .
```

## Architecture

### Entry Point
- [`src/main.rs`](src/main.rs) - Simple entry point that calls `Cli::run()` and handles errors

### Core Logic
- [`src/lib.rs`](src/lib.rs) - CLI definition using `clap` with subcommands:
  - `run` - Execute aliased scripts (prompts for selection on first use)
  - `ls` - List stored aliases
  - `alias` - Create new script aliases
  - `delete` - Remove aliases
  - `install` - Install dependencies with detected package manager

### Utilities
- [`src/utils.rs`](src/utils.rs) - All implementation logic:
  - Package manager detection via lockfile presence (`package-lock.json`, `yarn.lock`, `pnpm-lock.yaml`)
  - Config file management at `~/.config/runner.ini` (INI format via `rust-ini`)
  - Interactive fuzzy selection via `dialoguer`
  - Script execution via `std::process::Command`

### Key Dependencies
- `clap` - CLI argument parsing
- `dialoguer` - Interactive prompts (fuzzy select)
- `rust-ini` - Config persistence
- `anyhow` - Error handling
- `serde/serde_json` - package.json parsing

### Config Storage Format

Aliases are stored in `~/.config/runner.ini`:
```ini
[<script-name>]
<project-path>=<actual-npm-script>
```

Example:
```ini
[-]
/Users/awu/Data/project=start
[dev]
/Users/awu/Data/project=dev
```
