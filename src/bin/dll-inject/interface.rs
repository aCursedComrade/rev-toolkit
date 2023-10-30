use eframe::egui;

pub struct Interface {
    target: String,
    dll_path: String,
    status: String,
}

impl Interface {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::configure_styles(&cc.egui_ctx);
        Self { target: String::new(), dll_path: String::from("None"), status: String::new() }
    }

    fn configure_styles(ctx: &egui::Context) {
        use egui::{FontFamily::{Monospace, Proportional}, TextStyle, FontId};
        let mut style = (*ctx.style()).clone();

        style.text_styles = [
            (TextStyle::Body, FontId::new(16., Proportional)),
            (TextStyle::Button, FontId::new(16., Proportional)),
            (TextStyle::Heading, FontId::new(24., Proportional)),
            (TextStyle::Monospace, FontId::new(16., Monospace)),
            (TextStyle::Small, FontId::new(12., Proportional))
        ]
        .into();

        ctx.set_style(style);
    }
}

impl eframe::App for Interface {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.add(
                        egui::Image::new(egui::include_image!("assets/bigbrain.jpg")).max_size(egui::vec2(240., 240.)),
                    );
                    ui.heading("Comrade's Injector (WIP)");
                    ui.small(format!("A DLL injector by aCursedComrade"));
                });

                ui.label(format!("Cross-bitness injection is still in the works. You need to compile this injector in the same arch as the target."));
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
                        Ok(()) => { self.status = format!("OK: Successfully injected") },
                        Err(error) => { self.status = format!("ERR: {}", error) }
                    }
                }
                ui.label(format!("Last status: {}", self.status));
            })
        });
    }
}

fn spacer(ui: &mut egui::Ui, size: f32) {
    ui.add(egui::Separator::default().spacing(size));
}
