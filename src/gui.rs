use crate::{components::TrackEditorMode, MouseState, Tool};

use std::str::FromStr;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::tess::geom::euclid::default;
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_plugin(WorldInspectorPlugin::new());

        app.add_system(ui_debug);
    }
}

#[derive(Component)]
struct FpsCounter;

fn ui_debug(
    mut contexts: EguiContexts,
    time: Res<Time>,
    mouse: Res<MouseState>,
    current_tool: Res<Tool>,
    editor_mode: Option<Res<TrackEditorMode>>,
) {
    let game_time = time.elapsed_seconds();
    let delta_seconds = time.raw_delta_seconds_f64();
    if delta_seconds == 0.0 {
        return;
    }
    let fps = 1.0 / delta_seconds;
    let mouse_x = mouse.position.x;
    let mouse_y = mouse.position.y;
    let tool = current_tool.as_ref();

    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("time : {game_time:.1}"));
        ui.label(format!("fps : {fps:.1}"));
        ui.label(format!("mouse : ({mouse_x:.1}, {mouse_y:.1})"));
        ui.label(format!("tool : {tool:?}"));
        if editor_mode.is_some() {
            let mode_res = editor_mode.unwrap();
            let mode = mode_res.as_ref();
            ui.label(format!("mode : {mode:?}"));
        }
    });
}
