use crate::{
    components::*,
    draw::*,
    input, layer, num,
    tracks::*,
    ColorNames, GameColors, InputPlugin, MouseState, ORANGE, PURPLE,
    {EdgeComp, TrackGraph, TrackNode, TrackWeight},
};

use bevy::{
    ecs::query::QuerySingleError,
    input::{keyboard::KeyboardInput, ButtonState},
    math,
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};
use bevy_prototype_lyon::{entity::*, prelude::*};
use itertools::*;

use std::collections::HashMap;
use std::slice::Iter;

pub struct ToolPlugin;
impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        let mut current_tool = Tool::None;
        app.insert_resource(current_tool);

        app.add_systems((tool_wheel_system, selection_system));

        app.add_system(move_dragged_system);
        app.add_system(set_dragged_system.run_if(resource_equals(Tool::TrackBuilder)));
    }
}

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

fn set_dragged_system(
    mut commands: Commands,
    q_nodes: Query<(Entity, &Transform, Option<&Dragged>), With<NodeComp>>,
    mouse: Res<MouseState>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for (e, t, _) in q_nodes.iter() {
            let d = t.translation.truncate().distance(mouse.position);
            if d < 5.0 {
                commands.entity(e).insert(Dragged);
                return;
            }
        }
    } else if buttons.just_released(MouseButton::Left) {
        for (e, t, d) in q_nodes.iter() {
            if let Some(_) = d {
                commands.entity(e).remove::<Dragged>();
            }
        }
    }
}

fn move_dragged_system(
    mut commands: Commands,
    mouse: Res<MouseState>,
    mut q_dragged_nodes: Query<(Entity, &mut Transform), With<Dragged>>,
    mut q_tracks: Query<(Entity, &mut Path, &mut EdgeComp)>,
) {
    match q_dragged_nodes.get_single_mut() {
        Ok((dragged_node, mut transform)) => {
            transform.translation.x = mouse.position.x;
            transform.translation.y = mouse.position.y;
            let mut found = false;

            for (entity, mut path, mut edge) in q_tracks.iter_mut() {
                if edge.a == dragged_node {
                    edge.pos_a = mouse.position;
                    found = true;
                } else if edge.b == dragged_node {
                    edge.pos_b = mouse.position;
                    found = true;
                }
                if found {
                    commands.entity(entity).insert(Update);
                }
            }
        }
        other => (),
    }
}

fn tool_wheel_system(
    mut commands: Commands,
    mut current_tool: ResMut<Tool>,
    mut key_evr: EventReader<KeyboardInput>,
    mut q_ghost: Query<(Entity), With<Ghost>>,
) {
    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => match ev.key_code {
                Some(KeyCode::Tab) => {
                    *current_tool.as_mut() = Tool::next(current_tool.as_ref());
                    for entity in q_ghost.iter() {
                        commands.entity(entity).despawn();
                    }
                }
                other => (),
            },
            ButtonState::Released => {}
        }
    }
}

fn is_tool(current_tool: Res<Tool>, tool: &Tool) -> bool {
    current_tool.as_ref() == tool
}

fn selection_system(
    mut commands: Commands,
    q_selectable: Query<Entity, With<Selectable>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        for entity in q_selectable.iter() {
            commands.entity(entity).remove::<Selected>();
        }
    }
}
fn ghost_system(
    mut commands: Commands,
    q_ghosted: Query<Entity, With<Ghost>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        for entity in q_ghosted.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn track_tool_system(
    mut commands: Commands,
    mut track_graph: ResMut<TrackGraph>,
    current_tool: Res<Tool>,
    mouse: Res<MouseState>,
    buttons: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    q_selected: Query<(Entity, &Transform), (Or<(With<EdgeComp>, With<NodeComp>)>, With<Selected>)>,
) {
    if buttons.just_pressed(MouseButton::Left) {}
}
