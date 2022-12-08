use clap::{Parser, Subcommand};
use std::path::PathBuf;

use rta::{
    psarc::print_psarc_header,
    device_audio::system_test,
};

#[derive(Debug, Parser)]
#[command(name = "rta")]
#[command(about = "Real-time audio analysis", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Reads a PSARC file
    #[command(arg_required_else_help = true)]
    Read {
        /// The input PSARC file to read
        #[arg(required = true)]
        path: PathBuf,
    },
    /// Tests audio input and output devices
    Devices {
        #[arg(long, short, action)]
        overview: bool,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Read { path } => {
            println!("Reading file: {:?}", path);
            print_psarc_header(path)
                .unwrap_err();
        }
        Commands::Devices { overview } => {
            if overview {
                system_test()
                    .unwrap_err();
            } else {
                eprintln!("Not implemented yet")
            }
        }
    }
}
