use crate::{
    draw::*,
    input, layer, num,
    tracks::*,
    ColorNames, GameColors, InputPlugin, MouseState, ORANGE, PURPLE,
    {TrackComp, TrackGraph, TrackNode, TrackWeight},
};

use bevy::{
    ecs::query::QuerySingleError,
    input::{keyboard::KeyboardInput, ButtonState},
    math,
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};
use itertools::*;

use std::collections::HashMap;
use std::slice::Iter;

#[derive(Component)]
pub struct Selected;
#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Ghost;

pub struct ToolPlugin;
impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        let mut current_tool = Tool::None;
        app.insert_resource(current_tool);

        app.add_systems((
            tool_wheel_system,
            selection_system,
            track_tool_system
                .after(selection_system)
                .run_if(resource_equals(Tool::TrackBuilder)),
        ));
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
    q_selected: Query<
        (Entity, &Transform),
        (Or<(With<TrackComp>, With<NodeComp>)>, With<Selected>),
    >,
) {
    if buttons.just_pressed(MouseButton::Left) {
        // for (ghost, _, _) in q_tracks.iter() {
        //     commands.entity(ghost).despawn();
        // }
        let selected = q_selected.get_single();
        match selected {
            Err(QuerySingleError::NoEntities(_)) => {
                let node_sprite_fixed = get_node_sprite(mouse.position);
                commands.spawn((node_sprite_fixed, Ghost, Selected));

                let node_sprite_next = get_node_sprite(mouse.position);
                commands.spawn((node_sprite_next, Ghost));

                let track_sprite = get_track_sprite(mouse.position, mouse.position);
                commands.spawn((track_sprite, FollowCursor, Ghost));
            }
            Err(QuerySingleError::MultipleEntities(_)) => {}
            Ok(selected) => {
                commands.entity(selected.0).remove::<Selected>();
                commands.entity(selected.0).remove::<Ghost>();
                let node_sprite_fixed = get_node_sprite(mouse.position);
                commands.spawn((node_sprite_fixed, Selected));
                let track_sprite =
                    get_track_sprite(selected.1.translation.truncate(), mouse.position);
                commands.spawn((track_sprite,));
            }
        }
    }
}
