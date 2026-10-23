use bevy_app::{App, Plugin};

pub struct EguiBackendResource {
    egui_glow: egui_glow::EguiGlow,
    repaint_delay: core::time::Duration,
    clear_color: [f32; 3],
}

pub struct EguiBackendPlugin;

impl Plugin for EguiBackendPlugin {
    fn build(&self, app: &mut App) {

    }
}
