use eframe::egui;

pub struct Injector {
    target: String,
    dll_path: String,
}

impl Default for Injector {
    fn default() -> Self {
        Self {
            target: String::new(),
            dll_path: String::new(),
        }
    }
}

fn spacer(ui: &mut egui::Ui, size: f32) {
    ui.add(egui::Separator::default().spacing(size));
}

impl eframe::App for Injector {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Comrade's Injector");
            ui.label(format!("WIP DLL injector by @aCursed_Comrade"));
            ui.label(format!(r"For now, you need to compile this injector in the same arch as the target. Cross-bitness injection is still in the works."));
            spacer(ui, 16.);

            ui.horizontal(|ui| {
                let name_label = ui.label("Target: ");
                ui.text_edit_singleline(&mut self.target)
                    .labelled_by(name_label.id);
            });

            ui.horizontal(|ui| {
                let name_label = ui.label("DLL path: ");
                ui.text_edit_singleline(&mut self.dll_path)
                    .labelled_by(name_label.id);
            });
            spacer(ui, 16.);

            ui.label(format!("Target: {}", self.target));
            ui.label(format!("DLL: {}", self.dll_path));
        });
    }
}
