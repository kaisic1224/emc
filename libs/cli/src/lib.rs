use std::io;

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

pub fn get_input() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file {}", &args.path.display()))?;

    println!("{}", content);
    Ok(())
}

fn printOutput() -> Result<()> {
    let stdout = io::stdout();
    // aquire a lock on stdout and create a bufwriter on it
    let mut buf = io::BufWriter::new(stdout.lock());

    Ok(())
}

struct Config {
    pingyin: bool,
}
trait Options {
    fn get_song();
}

impl Options for Config {
    fn get_song() {
        todo!();
    }
}

#[test]
fn check_contains() {
    todo!()
}
