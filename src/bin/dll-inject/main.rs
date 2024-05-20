use clap::{Parser, Subcommand};
use rev_toolkit::Injector;

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
        /// PID of target program
        target: u32,

        #[arg(short, long)]
        /// Absolute or relative path to the DLL
        dll_path: String,
    },
}

fn main() -> Result<(), rev_toolkit::RTStatus> {
    let args = CliArgs::parse();

    match &args.command {
        // handle CLI mode
        Some(CliCmd::Cli { target, dll_path }) => Ok(Injector::new(*target, dll_path)?.inject()?),

        // handle GUI mode
        None => {
            println!("GUI is work-in-progress. Use the `-h` flag to see the help message");
            Ok(())
        }
    }
}
