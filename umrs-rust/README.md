# Rust workspace

Rust project

Short answer: yes, you’ll want a separate library crate, and the clean way to share it with umrs-state (and future tools) is to put them all in a Rust workspace.

I’ll walk you through a concrete layout and exactly how the dependency wiring works.

	1.	Directory layout (workspace approach)

Let’s say you create a top-level directory to hold all UMRS Rust things:

§umrs-rust/
§  Cargo.toml          <– workspace root
§  umrs-core/          <– shared library crate
§  umrs-state/         <– your existing binary crate

	2.	Step 1: create the workspace and library crate

Inside your development directory:

§cargo new umrs-rust
§cd umrs-rust
§rm src/main.rs

Now turn the top-level into a pure workspace root by editing its Cargo.toml like this:
```
[workspace]
members = [
    “umrs-core”,
    “umrs-state”,
]
```

Then create the library and move your existing binary:

§cargo new umrs-core –lib
§cargo new umrs-state

Now you have:
	•	umrs-core (lib): shared code for all tools
	•	umrs-state (bin): your state CLI that depends on umrs-core

	3.	Step 2: define the shared library API

In umrs-core/src/lib.rs you put the common pieces: path handling, JSON load/save, state types, etc.

Example skeleton:

§use std::path::Path;
§use std::fs;
§use std::io::{self, Read, Write};
§
§use serde::{Deserialize, Serialize};
§
§#[derive(Debug, Serialize, Deserialize)]
§pub struct UmrsState {
§    pub system_metadata: SystemMetadata,
§    // later: dynamic fields, integrity status, etc.
§}
§
§#[derive(Debug, Serialize, Deserialize)]
§pub struct SystemMetadata {
§    pub purpose: Option,
§    pub system_type: Option,
§    pub virtualization: Option,
§}
§
§impl Default for UmrsState {
§    fn default() -> Self {
§        UmrsState {
§            system_metadata: SystemMetadata {
§                purpose: None,
§                system_type: None,
§                virtualization: None,
§            },
§        }
§    }
§}
§
§pub fn load_state(path: &Path) -> io::Result {
§    if !path.exists() {
§        return Ok(UmrsState::default());
§    }
§    let mut file = fs::File::open(path)?;
§    let mut buf = String::new();
§    file.read_to_string(&mut buf)?;
§    let state: UmrsState = serde_json::from_str(&buf)
§        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
§    Ok(state)
§}
§
§pub fn save_state(path: &Path, state: &UmrsState) -> io::Result<()> {
§    let tmp_path = path.with_extension(“json.tmp”);
§    {
§        let mut file = fs::File::create(&tmp_path)?;
§        let data = serde_json::to_string_pretty(state)
§            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
§        file.write_all(data.as_bytes())?;
§        file.sync_all()?;
§    }
§    fs::rename(tmp_path, path)?;
§    Ok(())
§}

Corresponding umrs-core/Cargo.toml would include serde:

§[package]
§name = “umrs-core”
§version = “0.1.0”
§edition = “2021”
§
§[dependencies]
§serde = { version = “1”, features = [“derive”] }
§serde_json = “1”
	4.	Step 3: make umrs-state depend on umrs-core via a path dependency

In umrs-state/Cargo.toml:

§[package]
§name = “umrs-state”
§version = “0.1.0”
§edition = “2021”
§
§[dependencies]
§umrs-core = { path = “../umrs-core” }
§clap = { version = “4”, features = [“derive”] }
§serde = { version = “1”, features = [“derive”] }
§serde_json = “1”
§syslog = “6”

Now in umrs-state/src/main.rs you just use the library:

§use std::path::PathBuf;
§
§use clap::{Parser, Subcommand};
§use umrs-core::{UmrsState, SystemMetadata, load_state, save_state};
§
§#[derive(Parser, Debug)]
§#[command(name = “umrs-state”)]
§struct Cli {
§    #[arg(long = “state-file”)]
§    state_file: Option,
§
§    #[command(subcommand)]
§    command: Command,
§}
§
§#[derive(Subcommand, Debug)]
§enum Command {
§    Get {
§        key: String,
§    },
§    Set {
§        key: String,
§        value: String,
§    },
§}
§
§fn main() -> std::io::Result<()> {
§    let cli = Cli::parse();
§
§    let state_path = cli
§        .state_file
§        .unwrap_or_else(|| PathBuf::from(”/var/lib/umrs/umrs-state.json”));
§
§    let mut state = load_state(&state_path)?;
§
§    match cli.command {
§        Command::Get { key } => {
§            handle_get(&state, &key);
§        }
§        Command::Set { key, value } => {
§            handle_set(&mut state, &key, &value)?;
§            save_state(&state_path, &state)?;
§            log_manual_set(&key, &value);
§        }
§    }
§
§    Ok(())
§}
§
§fn handle_get(state: &UmrsState, key: &str) {
§    match key {
§        “system_metadata.purpose” => {
§            if let Some(p) = &state.system_metadata.purpose {
§                println!(”{p}”);
§            }
§        }
§        “system_metadata.system_type” => {
§            if let Some(t) = &state.system_metadata.system_type {
§                println!(”{t}”);
§            }
§        }
§        “system_metadata.virtualization” => {
§            if let Some(v) = &state.system_metadata.virtualization {
§                println!(”{v}”);
§            }
§        }
§        _ => {
§            eprintln!(“Unknown key: {key}”);
§        }
§    }
§}
§
§fn handle_set(state: &mut UmrsState, key: &str, value: &str) -> std::io::Result<()> {
§    match key {
§        “system_metadata.purpose” => {
§            state.system_metadata.purpose = Some(value.to_string());
§        }
§        “system_metadata.system_type” => {
§            state.system_metadata.system_type = Some(value.to_string());
§        }
§        “system_metadata.virtualization” => {
§            state.system_metadata.virtualization = Some(value.to_string());
§        }
§        _ => {
§            eprintln!(“Unknown key: {key}”);
§        }
§    }
§    Ok(())
§}
§
§fn log_manual_set(key: &str, value: &str) {
§    if let Err(e) = syslog::init_unix(
§        syslog::Facility::LOG_AUTH,
§        log::LevelFilter::Info,
§        Some(“umrs-state”),
§    ) {
§        eprintln!(“Failed to init syslog: {e}”);
§        return;
§    }
§
§    let msg = format!(“Manual umrs-state set: {key}={value}”);
§    if let Err(e) = syslog::log(syslog::Severity::LOG_INFO, &msg) {
§        eprintln!(“Failed to write syslog: {e}”);
§    }
§}

Now:
	•	All shared behavior lives in umrs-core.
	•	umrs-state is just a thin CLI wrapper that uses the library.
	•	Future tools (e.g. umrs-audit-chain, umrs-about, etc.) can also depend on umrs-core = { path = "../umrs-core" } and reuse the same types and helpers.

	5.	How you’ll use this going forward

From the workspace root (umrs-rust):
	•	Build everything:
§cargo build
	•	Run just umrs-state:
§cargo run -p umrs-state – get system_metadata.purpose
§cargo run -p umrs-state – set system_metadata.purpose “High-assurance MLS reference system”

This gives you exactly what you asked for:
	•	A reusable UMRS library,
	•	A path-based dependency (no network, fine for air-gapped RHEL 10),
	•	A clean way to expand the shared functionality as the UMRS project grows.

If you want, next step we can factor more of your existing umrs-state logic into umrs-core (e.g., JSON schema evolution, timestamp metadata, “dynamic” vs “static” fields) in this same pattern.





Now:
	•	All shared behavior lives in umrs-core.
	•	umrs-state is just a thin CLI wrapper that uses the library.
	•	Future tools (e.g. umrs-audit-chain, umrs-about, etc.) can also depend on umrs-core = { path = "../umrs-core" } and reuse the same types and helpers.

	5.	How you’ll use this going forward

From the workspace root (umrs-rust):
	•	Build everything:
§cargo build
	•	Run just umrs-state:
§cargo run -p umrs-state – get system_metadata.purpose
§cargo run -p umrs-state – set system_metadata.purpose “High-assurance MLS reference system”

This gives you exactly what you asked for:
	•	A reusable UMRS library,
	•	A path-based dependency (no network, fine for air-gapped RHEL 10),
	•	A clean way to expand the shared functionality as the UMRS project grows.




