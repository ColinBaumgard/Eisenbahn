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

use petgraph::{graphmap::*, visit::IntoEdges};

pub struct ToolPlugin;
impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        let mut current_tool = Tool::TrackEditor;
        app.insert_resource(current_tool);

        app.insert_resource(TrackGraph(UnGraphMap::new()));

        app.add_startup_system(init_track_editor_system);

        app.add_systems((
            update_hooked_on_cursor_system,
            update_tracks_system.after(update_hooked_on_cursor_system),
            track_editor_idle_system
                .run_if(resource_exists_and_equals(TrackEditorMode::Idle))
                .after(update_tracks_system),
            track_editor_building_system
                .run_if(resource_exists_and_equals(TrackEditorMode::PathBuilding))
                .after(update_tracks_system),
            position_hook_system.after(track_editor_building_system),
        ));
    }
}

fn init_track_editor_system(mut commands: Commands) {
    let mut mode = TrackEditorMode::Idle;
    commands.insert_resource(mode);
}

fn position_hook_system(
    mut commands: Commands,
    mouse: Res<MouseState>,
    q_nodes: Query<
        (Entity, &Transform),
        (With<NodeComp>, Without<HookedOnCursor>, Without<Selected>),
    >,
    q_attraction_rings: Query<Entity, With<AttractionRing>>,
) {
    for e in q_attraction_rings.iter() {
        commands.entity(e).despawn();
    }
    let mut attraction_node = q_nodes.iter().min_by(|a, b| {
        mouse
            .position
            .distance(a.1.translation.truncate())
            .total_cmp(&mouse.position.distance(b.1.translation.truncate()))
    });
    if attraction_node.is_some()
        && mouse
            .position
            .distance(attraction_node.unwrap().1.translation.truncate())
            < 10.0
    {
        let (entity, transform) = attraction_node.unwrap();
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: 10.0,
                    center: Vec2::ZERO,
                }),
                transform: transform.clone(),
                ..default()
            },
            Fill::color(Color::Hsla {
                hue: 0.,
                saturation: 0.,
                lightness: 0.,
                alpha: 0.,
            }),
            Stroke::new(Color::WHITE, 1.0),
            AttractionRing(entity),
        ));
    }
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
                if edge_comp.a == node_e {
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
    mut q_attraction_ring: Query<(&AttractionRing, &Transform)>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        // We check attraction nodes
        let attraction_node_res = q_attraction_ring.get_single();
        let (position, node_e) = match attraction_node_res.is_ok() {
            true => {
                let (att_ring, t) = attraction_node_res.unwrap();
                let e = att_ring.0;
                commands.entity(e).insert(Selected);
                (t.translation.truncate(), e)
            }
            false => {
                let position = mouse.position;
                let node = get_node_bundle(mouse.position);
                (mouse.position, commands.spawn((node, Selected)).id())
            }
        };
        if !graph.0.contains_node(node_e) {
            graph.0.add_node(node_e);
        }

        let node2 = get_node_bundle(mouse.position);
        let node2_e = commands.spawn((node2, HookedOnCursor)).id();
        graph.0.add_node(node2_e);

        let track = get_track_bundle(node_e, node2_e, &position, &mouse.position);
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
    q_selected: Query<(Entity, &Transform), With<Selected>>,
    mut q_hooked: Query<Entity, With<HookedOnCursor>>,
    mut q_attraction_ring: Query<(&AttractionRing, &Transform)>,
) {
    let hooked = q_hooked.single();
    let (selected, selected_transform) = q_selected.single();
    if buttons.just_pressed(MouseButton::Left) {
        commands.entity(selected).remove::<Selected>();

        // We check attraction nodes
        let attraction_node_res = q_attraction_ring.get_single();
        if attraction_node_res.is_ok() {
            let (att_ring, position_hooked_t) = attraction_node_res.unwrap();
            let position_hooked = att_ring.0;

            // remove building track
            let _t = graph.0.edge_weight(selected, hooked).unwrap();
            commands.entity(*_t).despawn();
            // Create new track
            let _track = get_track_bundle(
                selected,
                position_hooked,
                &selected_transform.translation.truncate(),
                &position_hooked_t.translation.truncate(),
            );
            let _track_e = commands.spawn((_track)).id();
            graph.0.add_edge(selected, position_hooked, _track_e);

            graph.0.remove_node(hooked);
            commands.entity(hooked).despawn();
            commands.insert_resource(TrackEditorMode::Idle);

            return;
        }

        commands
            .entity(hooked)
            .remove::<HookedOnCursor>()
            .insert(Selected);

        let new_node = get_node_bundle(mouse.position);
        let new_node_e = commands.spawn((new_node, HookedOnCursor)).id();
        graph.0.add_node(new_node_e);

        let track = get_track_bundle(hooked, new_node_e, &mouse.position, &mouse.position);
        let track_e = commands.spawn((track)).id();
        graph.0.add_edge(hooked, new_node_e, track_e);
    } else if keys.just_pressed(KeyCode::Escape) {
        commands.entity(hooked).despawn();
        commands.entity(selected).remove::<Selected>();
        let affected_edges = graph.0.edges(hooked);
        let mut to_be_removed = vec![hooked];
        for edge in affected_edges {
            commands.entity(*edge.2).despawn();
            for node in [edge.0, edge.1] {
                if node != hooked && graph.0.edges(node).exactly_one().is_ok() {
                    // graph.0.remove_node(node);
                    to_be_removed.push(node);
                }
            }
        }
        for node in to_be_removed {
            graph.0.remove_node(node);
            commands.entity(node).despawn();
        }
        commands.insert_resource(TrackEditorMode::Idle);
    }
}
