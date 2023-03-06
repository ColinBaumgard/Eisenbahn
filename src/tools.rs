use crate::{
    eisenbahn::*,
    layer, mouse, num, ColorNames, GameColors, MouseState, ORANGE, PURPLE,
    {TrackEdge, TrackGraph, TrackNode, TrackWeight},
};

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    math,
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};
use itertools::*;

use std::collections::HashMap;
use std::slice::Iter;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    TrackBuilder,
    None,
}
impl Tool {
    pub fn as_vec() -> Vec<Tool> {
        Vec::from([Tool::None, Tool::TrackBuilder])
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

#[derive(Component)]
pub struct OnBuild;

pub struct ToolPlugin;
impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        let mut current_tool = Tool::None;
        app.insert_resource(current_tool);

        app.add_system_set(
            SystemSet::new()
                .label("input")
                .with_system(tool_selection_system),
        );
        app.add_system_set(
            SystemSet::new()
                .label("tools")
                .after("input")
                .before("draw")
                .with_system(track_tool_system),
        );
    }
}

fn tool_selection_system(mut current_tool: ResMut<Tool>, mut key_evr: EventReader<KeyboardInput>) {
    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                match ev.scan_code {
                    20 => {
                        *current_tool.as_mut() = Tool::next(current_tool.as_ref());
                    }
                    other => (),
                }
                println!("Changed tool : {:?}", current_tool.as_ref());
            }
            ButtonState::Released => {}
        }
    }
}

fn track_tool_system(
    mut commands: Commands,
    mut track_graph: ResMut<TrackGraph>,
    current_tool: Res<Tool>,
    mouse: Res<MouseState>,
    buttons: Res<Input<MouseButton>>,
) {
    if *current_tool.as_ref() != Tool::TrackBuilder {
        return;
    }
    if buttons.just_pressed(MouseButton::Left) {
        let node_sprite = get_node_sprite(mouse.position);
        let draw_tag = DrawTag {
            action: DrawActions::FollowCursor,
        };
        commands.spawn((node_sprite, OnBuild, draw_tag));
    }
}
