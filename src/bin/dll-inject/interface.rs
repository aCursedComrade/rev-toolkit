use eframe::egui;

pub struct Injector {
    target: String,
    dll_path: String,
    status: String,
}

impl Default for Injector {
    fn default() -> Self {
        Self { target: String::new(), dll_path: String::from("None"), status: String::from("Standby...") }
    }
}

impl eframe::App for Injector {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // pic
            
            ui.heading("Comrade's Injector");
            ui.label(format!("WIP DLL injector by @aCursed_Comrade"));
            ui.label(format!(r"For now, you need to compile this injector in the same arch as the target. Cross-bitness injection is still in the works."));
            spacer(ui, 16.);

            ui.horizontal(|ui| {
                let name_label = ui.label("Target: ");
                ui.text_edit_singleline(&mut self.target)
                    .labelled_by(name_label.id);
            });
            ui.label(format!("DLL: {}", self.dll_path));
            if ui.button("Select DLL").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.dll_path = path.to_str().unwrap().to_owned();
                }
            }
            spacer(ui, 16.);

            if ui.button("Inject").clicked() {
                let status = unsafe { crate::inject::inject_dll(&self.target, &self.dll_path) };
                match status {
                    Ok(()) => { self.status = format!("(Ok) Successfully injected") },
                    Err(error) => { self.status = format!("(Error) {}", error) }
                }
            }
            ui.label(format!("Last status: {}", self.status));
        });
    }
}

fn spacer(ui: &mut egui::Ui, size: f32) -> egui::Response {
    ui.add(egui::Separator::default().spacing(size))
}
