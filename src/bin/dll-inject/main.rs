mod inject;
use clap::{Parser, Subcommand};
use inject::inject_dll;
use std::fmt::Display;

// TODO cross-bitness injection, dealing with 32 bit processes from a 64 bit context
// current state of the injector requires it be in the same mode (32 or 64 bit) as the target

// https://stackoverflow.com/questions/8776437/c-injecting-32-bit-targets-from-64-bit-process
// dll-syringe - https://github.com/OpenByteDev/dll-syringe/

#[derive(Parser)]
/// CLI arguments
pub struct CliArgs {
    #[command(subcommand)]
    command: Option<CliCmd>,
}

#[derive(Subcommand)]
/// CLI subcommands
pub enum CliCmd {
    /// Use the injector in CLI mode
    Cli {
        #[arg(short, long)]
        /// Target program (ex: explorer.exe)
        target: String,

        #[arg(short, long)]
        /// Absolute path to the DLL
        path: String,
    },
}

/// Error enums
pub enum InjectError {
    InvalidProcess,
    MemoryAllocError,
    MemoryWriteError,
    SpawnThreadError,
}

impl Display for InjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidProcess => write!(f, "Invalid process specified"),
            Self::MemoryAllocError => write!(f, "Failed to allocate memory on target"),
            Self::MemoryWriteError => write!(f, "Failed to write to memory on target"),
            Self::SpawnThreadError => write!(f, "Failed to spawn remote thread on target"),
        }
    }
}

fn main() {
    let args = CliArgs::parse();

    match &args.command {
        // handle CLI mode
        Some(CliCmd::Cli { target, path }) => {
            let status = unsafe { inject_dll(target.to_string(), path.to_string()) };

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
            println!("[!] GUI mode is not implemented yet. Please use the CLI mode.");
            println!("[!] Use -h flag for help")
        }
    }
}
