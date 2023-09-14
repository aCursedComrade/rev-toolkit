mod inject;
mod interface;
use clap::{Parser, Subcommand};

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
        /// Absolute or relative path to the DLL
        dll_path: String,
    },
}

fn main() -> Result<(), eframe::Error> {
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

            Ok(())
        }

        // handle GUI mode
        None => {
            let options = eframe::NativeOptions {
                initial_window_size: Some(eframe::egui::vec2(480., 640.)),
                ..Default::default()
            };

            eframe::run_native(
                "Comrade's Injector",
                options,
                Box::new(|_| Box::<interface::Injector>::default()),
            )
        }
    }
}
