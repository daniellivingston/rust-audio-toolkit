#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[macro_use]
extern crate log;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

use rta::{
    device_audio::{analyze_wav, system_overview, system_test},
    psarc::PlaystationArchive,
};

/// Print a summary of the PSARC file
fn print_psarc_summary(path: PathBuf) {
    let psarc = PlaystationArchive::read(&path);

    println!("FILENAME:\n  {:?}\n", path);
    println!("PSARC HEADER:\n  {:#?}\n", psarc.header);
    println!("TOC TABLE:\n  ENTRIES: {}\n", psarc.toc.len());
}

#[derive(Debug, Parser)]
#[command(name = "rta")]
#[command(about = "Real-time audio analysis", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Launch app with `egui` backend
    Run { },
    /// PSARC file introspection
    #[command(arg_required_else_help = true)]
    Read {
        /// The input PSARC file to read
        #[arg(required = true)]
        path: PathBuf,

        /// Print a summary of the PSARC file
        #[arg(long, short, action)]
        summary: bool,

        /// Analyze the WAV file
        #[arg(long, short, action)]
        analyze: bool,
    },
    /// Audio device real-time input and output tests
    Device {
        /// List available audio devices
        #[arg(long, short, action)]
        list: bool,
    },
}

fn main() {
    // Log to stdout (if ran with `RUST_LOG=debug`)
    tracing_subscriber::fmt::init();
    debug!("Starting rta...");

    let args = Cli::parse();
    debug!("Parsed argv: {:#?}", args);

    match args.command {
        Commands::Run { } => {
            let native_options = eframe::NativeOptions::default();
            eframe::run_native(
                "RTA",
                native_options,
                Box::new(|cc| Box::new(rta::TemplateApp::new(cc))),
            );
        }
        Commands::Read {
            path,
            summary,
            analyze,
        } => {
            if summary {
                print_psarc_summary(path);
            } else if analyze {
                analyze_wav(path).expect("Analyze WAV failed");
            } else {
                eprintln!("Invalid arguments");
            }
        }
        Commands::Device { list } => {
            if list {
                system_overview();
            } else {
                system_test().expect("System test failed");
            }
        }
    }
}

