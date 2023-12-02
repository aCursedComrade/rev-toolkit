mod structs;
use hudhook::hooks::{ImguiRenderLoop, dx11::ImguiDx11Hooks};
use imgui::*;
use std::time::Instant;

struct RFHud {
    start: Instant,
}

impl RFHud {
    fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl ImguiRenderLoop for RFHud {
    fn render(&mut self, ui: &mut Ui) {
        ui.window("#rf_hud")
            .size([320., 200.], Condition::Always)
            .build(|| {
                ui.text("ayo actually my first internal hud");
                ui.text("god bless hudhook");
                ui.text(format!("Elapsed: {:?}", self.start.elapsed()));
            });
    }
}

hudhook::hudhook!(RFHud::new().into_hook::<ImguiDx11Hooks>());
