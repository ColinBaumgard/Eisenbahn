use crate::{
    components::*,
    draw::*,
    input,
    layer::{self, CURSOR},
    num, ColorNames, GameColors, MouseState, ORANGE, PURPLE,
};

use bevy::{
    ecs::query::WorldQuery,
    math,
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};
use bevy_prototype_lyon::{entity::*, prelude::*};

pub struct TrackPlugin;
impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn get_node_bundle(
    pos: Vec2,
) -> (
    NodeComp,
    bevy_prototype_lyon::entity::ShapeBundle,
    bevy_prototype_lyon::draw::Fill,
    bevy_prototype_lyon::draw::Stroke,
    NormalColor,
    SelectedColor,
    GhostColor,
) {
    (
        NodeComp,
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle {
                radius: 5.0,
                center: Vec2::ZERO,
            }),
            transform: Transform::from_translation(pos.extend(layer::NODES)),
            ..default()
        },
        Fill::color(Color::RED),
        Stroke::new(Color::RED, 2.0),
        NormalColor(Color::RED),
        SelectedColor(Color::WHITE),
        GhostColor(Color::ORANGE),
    )
}

pub fn get_track_bundle(
    a: Entity,
    b: Entity,
    pos_a: &Vec2,
    pos_b: &Vec2,
) -> (
    EdgeComp,
    bevy_prototype_lyon::entity::ShapeBundle,
    bevy_prototype_lyon::draw::Fill,
    bevy_prototype_lyon::draw::Stroke,
    NormalColor,
    SelectedColor,
    GhostColor,
) {
    (
        EdgeComp {
            a: a,
            b: b,
            pos_a: pos_a.clone(),
            pos_b: pos_b.clone(),
        },
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Line(pos_a.clone(), pos_b.clone())),
            transform: Transform::from_translation(Vec2::ZERO.extend(layer::TRACKS)),
            ..default()
        },
        Fill::color(Color::RED),
        Stroke::new(Color::RED, 2.0),
        NormalColor(Color::RED),
        SelectedColor(Color::WHITE),
        GhostColor(Color::ORANGE),
    )
}
