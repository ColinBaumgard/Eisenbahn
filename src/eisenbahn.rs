use crate::{
    layer, mouse, num, ColorNames, GameColors, MouseState, ORANGE, PURPLE,
    {TrackEdge, TrackGraph, TrackNode, TrackWeight},
};

use bevy::{
    math,
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};

use bevy_prototype_lyon::{entity::*, prelude::*};
use petgraph::{
    graphmap::GraphMap,
    stable_graph::{EdgeIndex, EdgeIndices, EdgeReference, NodeIndex, NodeIndices, StableGraph},
    Undirected,
};
use std::collections::HashMap;
pub struct EisenbahnPlugin;
impl Plugin for EisenbahnPlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system(initialise_world);

        // let mut tracks = TrackGraph::new();
        // app.insert_resource(tracks);

        // app.add_system(update_nodes_system);
    }
}

// fn initialise_world(
//     mut commands: Commands,
//     game_colors: Res<GameColors>,
//     mut track_graph: ResMut<TrackGraph>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
// }

// #[derive(Component)]
// pub struct Position(Vec2);

// fn mouse_system(
//     mut commands: Commands,
//     mut track_graph: ResMut<TrackGraph>,
//     mouse: Res<MouseState>,
//     buttons: Res<Input<MouseButton>>,
// ) {
//     // if buttons.just_pressed(MouseButton::Left) {
//     //     let node_sprite = get_node_shape(mouse.position);
//     //     let node_index = track_graph.add_node();
//     //     commands.spawn((TrackNode { id: node_index }, node_sprite));
//     // }
// }

// fn spawn_node(
//     mut commands: Commands,

// )

// fn update_nodes_system(
//     mut commands: Commands,
//     mut track_graph: ResMut<TrackGraph>,
//     mut q_nodes: Query<(Entity, &mut TrackNode, &mut Transform), Without<TrackEdge>>,
//     mut q_track: Query<(Entity, &mut TrackEdge, &mut Transform), Without<TrackNode>>,
// ) {
//     let mut graph_nodes_vec: Vec<NodeIndex> = track_graph.get_all_nodes().collect();

//     let mut node_entity_map = HashMap::new();

//     // Loop through registered nodes
//     for (entity, node, mut transform) in q_nodes.iter_mut() {
//         graph_nodes_vec.retain(|vec_node| *vec_node == node.id);
//         if !track_graph.contains_node(node.id) {
//             commands.entity(entity).despawn();
//         } else {
//             // node_entity_map.insert(node.id, entity);
//         }
//     }
//     // Loop through unregistered nodes
//     for g_node in graph_nodes_vec {
//         let mut node_sprite = get_node_shape(g_node.position);
//         let entity = commands.spawn((g_node, node_sprite)).id();
//         node_entity_map.insert(g_node.id, entity);
//     }

// let mut graph_tracks_vec: Vec<(TrackNode, TrackNode, &())> =
//     track_graph.get_all_edges().collect();
// Loop through registered tracks
// for (entity, edge, mut transform) in q_track.iter_mut() {
//     graph_tracks_vec.retain(|vec_node| vec_node.0.id == edge.a && vec_node.1.id == edge.b);
//     if !track_graph.contains_node(*node) {
//         commands.entity(entity).despawn();
//     } else {
//         node_entity_map.insert(node.id, entity);
//         transform.translation.x = node.position.x;
//         transform.translation.y = node.position.y;
//     }
// }
// Loop through unregistered tracks
// for (g_node_a, g_node_b, _) in graph_tracks_vec {
//     let mut track_sprite = get_track_shape(g_node_a.position, g_node_b.position);
//     commands.spawn((
//         TrackEdge {
//             a: g_node_a.id,
//             b: g_node_b.id,
//         },
//         track_sprite,
//     ));
// }

fn get_node_shape(pos: Vec2) -> ShapeBundle {
    GeometryBuilder::build_as(
        &shapes::Circle {
            radius: 5.0,
            center: Vec2::splat(0.0),
        },
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::RED),
            outline_mode: StrokeMode::new(Color::RED, 2.0),
        },
        Transform::from_translation(pos.extend(layer::TRACKS)),
    )
}

fn get_track_shape(pos1: Vec2, pos2: Vec2) -> ShapeBundle {
    GeometryBuilder::build_as(
        &shapes::Line(pos1, pos2),
        DrawMode::Stroke(StrokeMode::new(ORANGE, 2.0)),
        Transform::from_xyz(0.0, 0.0, layer::TRACKS),
    )
}
