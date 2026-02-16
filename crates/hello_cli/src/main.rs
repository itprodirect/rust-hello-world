use std::process;

use clap::Parser;
use hello_lib::{greet, parse_name};

#[derive(Parser)]
#[command(name = "hello_cli", version, about = "Greet someone by name")]
struct Cli {
    /// Name to greet. Falls back to the USER/USERNAME env var if omitted.
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let raw_name = cli.name.unwrap_or_else(|| {
        std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_default()
    });

    match parse_name(&raw_name) {
        Ok(name) => println!("{}", greet(name)),
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    }
}
