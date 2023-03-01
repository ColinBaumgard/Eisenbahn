use crate::{graph, layer, mouse, num, ColorNames, GameColors, MouseState, ORANGE, PURPLE};

use bevy::{
    math,
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};

use bevy_prototype_lyon::{entity::*, prelude::*};
use petgraph::{graphmap::GraphMap, Undirected};

pub struct EisenbahnPlugin;
impl Plugin for EisenbahnPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialise_world);

        let mut tracks = graph::TrackGraph::new();
        app.insert_resource(tracks);

        app.add_system(update_graph_system);
    }
}

fn initialise_world(
    mut commands: Commands,
    game_colors: Res<GameColors>,
    mut track_graph: ResMut<graph::TrackGraph>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let pos1 = Vec2 { x: 0.0, y: 0.0 };
    let pos2 = Vec2 { x: 0.0, y: 100.0 };
    let pos3 = Vec2 { x: 0.0, y: 150.0 };
    let pos4 = Vec2 { x: 100.0, y: 150.0 };

    let (n1, n2) = track_graph.add_unconnected_track(pos1, pos2);
    let n3 = track_graph.extend_track(n2, pos3);
    let n4 = track_graph.extend_track(n3, pos4);
    //     track_graph.add_track(n2, n4);
}

#[derive(Component)]
pub struct Position(Vec2);

fn update_graph_system(
    mut commands: Commands,
    mut track_graph: ResMut<graph::TrackGraph>,
    mut q_nodes: Query<(Entity, &mut graph::TrackNode, &mut Transform)>,
) {
    let mut graph_nodes_vec: Vec<graph::TrackNode> = track_graph.get_all_nodes().collect();
    for (entity, node, mut transform) in q_nodes.iter_mut() {
        graph_nodes_vec.retain(|vec_node| *vec_node == *node);
        if !track_graph.contains_node(*node) {
            commands.entity(entity).despawn();
        } else {
            transform.translation.x = node.position.x;
            transform.translation.y = node.position.y;
        }
    }
    for g_node in graph_nodes_vec {
        let mut node_sprite = get_node_shape(g_node.position);
        commands.spawn((g_node, node_sprite));
    }
}

// pub fn add_unconnected_track(
//     mut commands: &Commands,
//     mut track_graph: &ResMut<graph::TrackGraph>,
//     pos1: Vec2,
//     pos2: Vec2,
// ) {
//     let (mut a, mut b) = track_graph.add_unconnected_track(pos1, pos2);

//     // add_track(commands, track_graph, a, b);

//     let mut sprite_a = get_node_shape(a.position);
//     let mut sprite_b = get_node_shape(b.position);
//     commands.spawn((a, sprite_a));
//     commands.spawn((b, sprite_b));
// }

// pub fn add_track(
//     mut commands: Commands,
//     mut track_graph: ResMut<graph::TrackGraph>,
//     a: graph::TrackNode,
//     b: graph::TrackNode,
// ) {
//     let mut sprite_track = get_track_shape(a.position, b.position);
//     commands.spawn((graph::TrackEdge, sprite_track));
// }

// pub fn extend_track(
//     mut commands: Commands,
//     mut track_graph: ResMut<graph::TrackGraph>,
//     node: Node,
//     pos: Vec2,
// ) {
// }

// pub fn split_track(
//     mut commands: Commands,
//     mut track_graph: ResMut<graph::TrackGraph>,
//     a: Node,
//     b: Node,
//     pos: Vec2,
// ) {
// }

// pub fn remove_track(
//     mut commands: Commands,
//     mut track_graph: ResMut<graph::TrackGraph>,
//     a: Node,
//     b: Node,
// ) {
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
