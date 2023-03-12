use crate::{MouseState, Tool};

use std::str::FromStr;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_prototype_lyon::prelude::tess::geom::euclid::default;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin);

        app.add_system(ui_example);
    }
}

#[derive(Component)]
struct FpsCounter;

fn ui_example(
    mut contexts: EguiContexts,
    time: Res<Time>,
    mouse: Res<MouseState>,
    current_tool: Res<Tool>,
) {
    let delta_seconds = time.raw_delta_seconds_f64();
    if delta_seconds == 0.0 {
        return;
    }
    let fps = 1.0 / delta_seconds;
    let mouse_x = mouse.position.x;
    let mouse_y = mouse.position.y;
    let tool = current_tool.as_ref();

    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("fps : {fps:.1}"));
        ui.label(format!("mouse : ({mouse_x:.1}, {mouse_y:.1})"));
        ui.label(format!("tool : {tool:?}"));
    });
}
