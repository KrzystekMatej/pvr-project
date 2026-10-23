use bevy_app::App;
use engine::{runtime::application::WinitRunnerPlugin, window::{LogicalSize, WindowAttributes, GlWindowPlugin}};

fn main() {
    App::new()
        .add_plugins((
            WinitRunnerPlugin,
            GlWindowPlugin::new(WindowAttributes::default()
                .with_resizable(true)
                .with_inner_size(LogicalSize {
                    width: 800.0,
                    height: 600.0,
                })
                .with_title("Example")
                .with_visible(false))
        ))
        .run();
}
