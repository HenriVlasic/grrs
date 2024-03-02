use anyhow::{Context, Result};
use std::io::{self, Write};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()>{
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let mut line_num : u32 = 0;
    for line in content.lines() {
        line_num += 1;
        if line.contains(&args.pattern) {
            writeln!(handle,"{} {}", line_num, line)?;
        }
    }
    Ok(())
}
