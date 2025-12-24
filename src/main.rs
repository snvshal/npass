mod crypto;
mod store;

use crate::store::Store;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use rpassword::read_password;
use std::io::Write;

#[derive(Parser)]
#[command(
    name = "npass",
    author,
    version,
    about = "npass: simple password manager (get, set, rm, ls)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set a password: name [PASSWORD]
    Set {
        name: String,
        #[arg(value_name = "PASSWORD")]
        value: Option<String>,
        /// Overwrite existing entry
        #[arg(long)]
        overwrite: bool,
    },
    /// Get a password by name
    Get { name: String },
    /// Remove a password. By default moves to backup; use --force to permanently delete
    Rm {
        name: String,
        #[arg(short, long)]
        force: bool,
    },
    /// List names. Use --all to show values and backups
    Ls {
        #[arg(short, long)]
        all: bool,
    },
    /// Manage backups (ls / rm)
    Backup {
        #[command(subcommand)]
        cmd: BackupCommands,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut store = Store::load()?;

    match cli.cmd {
        Commands::Set {
            name,
            value,
            overwrite,
        } => {
            let val = match value {
                Some(v) => v,
                None => {
                    if !atty::is(atty::Stream::Stdin) {
                        anyhow::bail!("No TTY available for entering value.");
                    }
                    eprint!("Value for '{}': ", name);
                    std::io::stderr().flush().ok();
                    read_password().context("read value")?
                }
            };

            match store.set(&name, &val, overwrite) {
                Ok(()) => {
                    store.save().context("save store")?;
                    println!("Set '{}'.", name);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(2);
                }
            }
        }
        Commands::Get { name } => {
            if let Some(e) = store.get(&name) {
                println!("{}", e.value);
            } else {
                eprintln!("Not found: {}", name);
                std::process::exit(2);
            }
        }
        Commands::Rm { name, force } => {
            if let Some(entry) = store.remove(&name) {
                if force {
                    // permanently delete
                    store.save().context("save store")?;
                    println!("Removed '{}' permanently.", name);
                } else {
                    store.backup_entry(entry);
                    store.save().context("save store")?;
                    println!("Moved '{}' to backups.", name);
                }
            } else {
                eprintln!("Not found: {}", name);
                std::process::exit(2);
            }
        }
        Commands::Ls { all } => {
            if all {
                for (k, v) in &store.entries {
                    println!("{}:{}", k, v.value);
                }
                for b in &store.backups {
                    println!("[backup] {}:{}", b.name, b.value);
                }
            } else {
                for k in store.entries.keys() {
                    println!("{}", k);
                }
            }
        }
        Commands::Backup { cmd } => match cmd {
            BackupCommands::Rm { name } => {
                if let Some(_entry) = store.remove_backup(&name) {
                    store.save().context("save store")?;
                    println!("Removed '{}' from backups permanently.", name);
                } else {
                    eprintln!("Not found in backups: {}", name);
                    std::process::exit(2);
                }
            }
            BackupCommands::Restore { name } => {
                if let Some(entry) = store.remove_backup(&name) {
                    if store.entries.contains_key(&entry.name) {
                        // Put it back into backups to avoid accidental loss
                        store.backup_entry(entry);
                        eprintln!("Entry '{}' already exists in store", name);
                        std::process::exit(2);
                    }
                    store.entries.insert(entry.name.clone(), entry);
                    store.save().context("save store")?;
                    println!("Restored '{}' from backups.", name);
                } else {
                    eprintln!("Not found in backups: {}", name);
                    std::process::exit(2);
                }
            }
            BackupCommands::Ls => {
                for b in &store.backups {
                    println!("{}:{}", b.name, b.value);
                }
            }
        },
    }

    Ok(())
}

#[derive(Subcommand)]
enum BackupCommands {
    /// Remove an entry from backups permanently
    Rm { name: String },
    /// List backups
    Ls,
    /// Restore an entry from backups back into the store
    Restore { name: String },
}
