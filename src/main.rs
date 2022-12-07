use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Psarc {
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.debug {
        0 => println!("Debug mode = OFF"),
        1 => println!("Debug mode = INFO"),
        2 => println!("Debug mode = DEBUG"),
        3 => println!("Debug mode = TRACE"),
        _ => println!("Set to {}", cli.debug),
    }

    match &cli.command {
        Some(Commands::Psarc { file }) => {
            println!("psarc");
            println!("{:?}", file);
        }
        None => {}
    }
}
