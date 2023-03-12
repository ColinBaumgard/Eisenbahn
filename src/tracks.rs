use crate::{
    draw::*,
    input,
    layer::{self, CURSOR},
    num, ColorNames, GameColors, MouseState, ORANGE, PURPLE, {TrackGraph, TrackNode, TrackWeight},
};

use bevy::{
    ecs::query::WorldQuery,
    math,
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};

pub struct TrackPlugin;
impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(draw_nodes);
        app.add_system(draw_tracks);
    }
}

fn draw_nodes(
    mut commands: Commands,
    mut q_nodes: Query<(&mut Transform), (With<NodeComp>, With<FollowCursor>)>,
    mouse: Res<MouseState>,
) {
    for mut transform in q_nodes.iter_mut() {
        transform.translation = mouse.position.extend(transform.translation.z);
    }
}
fn draw_tracks(
    mut commands: Commands,
    mut q_nodes: Query<(&mut Transform), (With<TrackComp>, With<FollowCursor>)>,
    mouse: Res<MouseState>,
) {
    for mut transform in q_nodes.iter_mut() {
        transform.translation = mouse.position.extend(transform.translation.z);
    }
}

#[derive(Component)]
pub struct TrackComp;
#[derive(Component)]
pub struct NodeComp;

use bevy_prototype_lyon::{entity::*, prelude::*};
pub fn get_node_sprite(
    pos: Vec2,
) -> (
    NodeComp,
    bevy_prototype_lyon::entity::ShapeBundle,
    bevy_prototype_lyon::draw::Fill,
    bevy_prototype_lyon::draw::Stroke,
) {
    (
        NodeComp,
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle {
                radius: 5.0,
                center: Vec2::ZERO,
            }),
            transform: Transform::from_translation(pos.extend(layer::TRACKS)),
            ..default()
        },
        Fill::color(Color::RED),
        Stroke::new(Color::RED, 2.0),
    )
}

pub fn get_track_sprite(
    pos1: Vec2,
    pos2: Vec2,
) -> (
    TrackComp,
    bevy_prototype_lyon::entity::ShapeBundle,
    bevy_prototype_lyon::draw::Fill,
    bevy_prototype_lyon::draw::Stroke,
) {
    (
        TrackComp,
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Line(Vec2::ZERO, pos2 - pos1)),
            transform: Transform::from_translation(pos1.extend(layer::TRACKS)),
            ..default()
        },
        Fill::color(Color::RED),
        Stroke::new(Color::RED, 2.0),
    )
}
