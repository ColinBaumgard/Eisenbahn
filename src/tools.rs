use crate::{
    components::{self, *},
    draw::*,
    input, layer, num,
    tracks::*,
    ColorNames, GameColors, InputPlugin, MouseState, ORANGE, PURPLE,
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

use petgraph::graphmap::*;

pub struct ToolPlugin;
impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        let mut current_tool = Tool::TrackEditor;
        app.insert_resource(current_tool);

        app.insert_resource(TrackGraph(UnGraphMap::new()));

        app.add_startup_system(init_track_editor_system);

        app.add_systems((
            update_hooked_on_cursor_system,
            update_tracks_system,
            track_editor_idle_system
                .run_if(resource_exists_and_equals(TrackEditorMode::Idle))
                .before(update_hooked_on_cursor_system),
            track_editor_building_system
                .run_if(resource_exists_and_equals(TrackEditorMode::PathBuilding))
                .before(update_hooked_on_cursor_system),
        ));
    }
}

fn init_track_editor_system(mut commands: Commands) {
    let mut mode = TrackEditorMode::Idle;
    commands.insert_resource(mode);
}

fn update_hooked_on_cursor_system(
    mut commands: Commands,
    mouse: Res<MouseState>,
    mut q_hooked: Query<(Entity, &mut Transform), With<HookedOnCursor>>,
) {
    for (e, mut transform) in q_hooked.iter_mut() {
        transform.translation.x = mouse.position.x;
        transform.translation.y = mouse.position.y;
        commands.entity(e).insert(Moved);
    }
}

fn update_tracks_system(
    mut commands: Commands,
    mut q_moved_nodes: Query<(Entity, &Transform), (With<Moved>, With<NodeComp>)>,
    mut q_tracks: Query<(Entity, &mut EdgeComp, &mut Path)>,
    graph: Res<TrackGraph>,
) {
    for (node_e, node_t) in q_moved_nodes.iter() {
        let affected_edges = graph.0.edges(node_e);
        for (from, to, edge) in affected_edges {
            let get_result = q_tracks.get_mut(*edge);
            if get_result.is_ok() {
                let (_, mut edge_comp, mut path) = get_result.unwrap();
                if from == node_e {
                    edge_comp.pos_a = node_t.translation.truncate();
                } else {
                    edge_comp.pos_b = node_t.translation.truncate();
                }
                let mut path_builder = PathBuilder::new();
                path_builder.move_to(edge_comp.pos_a);
                path_builder.line_to(edge_comp.pos_b);
                *path = path_builder.build();
            }
        }
        commands.entity(node_e).remove::<Moved>();
    }
}

fn track_editor_idle_system(
    mut commands: Commands,
    mouse: Res<MouseState>,
    buttons: Res<Input<MouseButton>>,
    mut graph: ResMut<TrackGraph>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let node = get_node_bundle(mouse.position);
        let node_e = commands.spawn(node).id();
        graph.0.add_node(node_e);

        let node2 = get_node_bundle(mouse.position);
        let node2_e = commands.spawn((node2, HookedOnCursor)).id();
        graph.0.add_node(node2_e);

        let track = get_track_bundle(node_e, node2_e, &mouse.position, &mouse.position);
        let track_e = commands.spawn((track)).id();
        graph.0.add_edge(node_e, node2_e, track_e);

        commands.insert_resource(TrackEditorMode::PathBuilding);
    }
}
fn track_editor_building_system(
    mut commands: Commands,
    mouse: Res<MouseState>,
    buttons: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    mut graph: ResMut<TrackGraph>,
    q_hooked: Query<Entity, With<HookedOnCursor>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let hooked = q_hooked.single();
        commands.entity(hooked).remove::<HookedOnCursor>();
        commands.insert_resource(TrackEditorMode::Idle);
    } else if keys.just_pressed(KeyCode::Escape) {
    }
}
