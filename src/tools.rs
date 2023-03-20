use crate::components::*;

mod track_editor;
mod train_editor;
mod viewer;

use crate::tools::track_editor::*;
use crate::tools::train_editor::*;
use crate::tools::viewer::*;

use bevy::{
    ecs::query::QuerySingleError,
    input::{keyboard::KeyboardInput, ButtonState},
    math,
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};
use bevy_prototype_lyon::{entity::*, prelude::*};

use self::track_editor::TrackEditorPlugin;

pub struct ToolPlugin;
impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Tool::Viewer);

        app.add_plugin(TrackEditorPlugin);
        app.add_plugin(TrainEditorPlugin);
        app.add_plugin(ViewerPlugin);

        app.add_system(tool_wheel_system);
    }
}

impl Tool {
    pub fn as_vec() -> Vec<Tool> {
        Vec::from([Tool::Viewer, Tool::TrackEditor, Tool::TrainEditor])
    }
    pub fn next(current_tool: &Tool) -> Tool {
        let mut last_el = *Tool::as_vec().last().unwrap();
        let mut iter = Tool::as_vec();
        for el in iter {
            if last_el == *current_tool {
                return el;
            }
            last_el = el;
        }
        last_el
    }
}

fn tool_wheel_system(
    mut commands: Commands,
    mut current_tool: ResMut<Tool>,
    mut key_evr: EventReader<KeyboardInput>,
) {
    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => match ev.key_code {
                Some(KeyCode::Tab) => {
                    *current_tool.as_mut() = Tool::next(current_tool.as_ref());
                }
                other => (),
            },
            ButtonState::Released => {}
        }
    }
}
