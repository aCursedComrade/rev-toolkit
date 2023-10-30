mod inject;
mod errors;
mod interface;
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
                Box::new(|cc| {
                    egui_extras::install_image_loaders(&cc.egui_ctx);

                    Box::new(interface::Interface::new(cc))
                }),
            )
        }
    }
}
