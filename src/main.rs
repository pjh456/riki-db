mod cli;
mod kv_trait;
mod storage;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use kv_trait::KVStore;
use storage::memory::MemoryStore;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut mem = MemoryStore::new();

    match cli.command {
        Commands::Set { key, value } => {
            let key4log = key.clone();
            let val4log = value.clone();
            match mem.set(key, value) {
                Ok(op) => match op {
                    Some(last) => println!("Update '{}' to '{}'", last, val4log),
                    None => println!("Add pair ({}, {})", key4log, val4log),
                },
                Err(_) => println!("Unable to set pair ({}, {})", key4log, val4log),
            }
        }
        Commands::Get { key } => match mem.get(&key) {
            Ok(op) => match op {
                Some(val) => println!("find pair: ({}, {})", key, val),
                None => println!("Nothing find where key = '{}'", key),
            },
            Err(_) => println!("Unable to get value of '{}'", key),
        },
        Commands::Remove { key } => match mem.remove(&key) {
            Ok(op) => match op {
                Some(val) => println!("Remove pair ({}, {})", key, val),
                None => println!("Remove nothing where key = '{}'", key),
            },
            Err(_) => println!("Unable to remove value of '{}'", key),
        },
        Commands::Prefix { perfix } => {}
    }

    Ok(())
}
