use bevy_app::{App, AppExit, Plugin};
use bevy_ecs::system::{In, InRef};

use egui_winit::winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

use crate::handlers::{HandlerAppExt, HandlerKind};


pub struct WinitResumed;
pub struct WinitWindowEvent;
pub struct WinitExiting;

pub type LoopInput = InRef<'static, ActiveEventLoop>;

pub type WindowInput = (
    InRef<'static, ActiveEventLoop>,
    In<WindowId>,
    InRef<'static, WindowEvent>,
);

impl HandlerKind for WinitResumed {
    type Input = LoopInput;
}

impl HandlerKind for WinitWindowEvent {
    type Input = WindowInput;
}

impl HandlerKind for WinitExiting {
    type Input = LoopInput;
}

pub struct WinitRunnerPlugin;

impl Plugin for WinitRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(run_winit);
    }
}

fn run_winit(app: App) -> AppExit {
    let event_loop = EventLoop::new()
        .expect("Failed to create winit event loop");

    let mut handler = WinitLoopHandler::new(app);

    event_loop
        .run_app(&mut handler)
        .expect("Failed to run winit event loop");

    handler.exit_status
}

struct WinitLoopHandler {
    bevy_app: App,
    exit_status: AppExit,
}

impl WinitLoopHandler {
    fn new(bevy_app: App) -> Self {
        Self {
            bevy_app,
            exit_status: AppExit::Success,
        }
    }
}

impl ApplicationHandler for WinitLoopHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);

        self.bevy_app
            .run_handlers_with::<WinitResumed>(event_loop)
            .expect("Winit resumed handler failed");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        self.bevy_app
            .run_handlers_with::<WinitWindowEvent>((
                event_loop,
                window_id,
                &event,
            ))
            .expect("Winit window event handler failed");

        if matches!(event, WindowEvent::CloseRequested) {
            event_loop.exit();
        }
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        self.bevy_app
            .run_handlers_with::<WinitExiting>(event_loop)
            .expect("Winit exiting handler failed");
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.bevy_app.update();

        if let Some(exit) = self.bevy_app.should_exit() {
            self.exit_status = exit;
            event_loop.exit();
        }
    }
}