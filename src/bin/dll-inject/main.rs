mod inject;
mod errors;
use clap::{Parser, Subcommand};

#[derive(Parser)]
/// Arguments
pub struct CliArgs {
    #[command(subcommand)]
    command: Option<CliCmd>,
}

#[derive(Subcommand)]
/// Subcommands
pub enum CliCmd {
    /// Use the injector in CLI mode
    Cli {
        #[arg(short, long)]
        /// Target program (ex: explorer.exe)
        target: String,

        #[arg(short, long)]
        /// Absolute or relative path to the DLL
        dll_path: String,
    },
}

fn main() {
    let args = CliArgs::parse();

    match &args.command {
        // handle CLI mode
        Some(CliCmd::Cli { target, dll_path }) => {
            let status = unsafe { inject::inject_dll(target, dll_path) };

            match status {
                Ok(()) => {
                    println!("[*] DLL injected");
                }
                Err(error) => {
                    println!("[!] Error: {}", error);
                }
            }
        }

        // handle GUI mode
        None => {
            println!("GUI is work-in-progress. Use the `-h` flag to see the help message");
        }
    }
}
