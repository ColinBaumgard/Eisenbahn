use crate::{
    components::*,
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
use bevy_prototype_lyon::{entity::*, prelude::*};

pub struct TrackPlugin;
impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(test_system);
        app.add_system(update_tracks);
    }
}

fn test_system(mut commands: Commands) {
    let mut pos = Vec2::ZERO;
    let d_pos = Vec2 { x: 20.0, y: 0.0 };
    let factor = 5.0;

    let t1 = Vec2 {
        x: 00.0 * factor,
        y: -55.0,
    };
    let n1 = get_node_bundle(t1);
    let t2 = Vec2 {
        x: 20.0 * factor,
        y: 10.0,
    };
    let n2 = get_node_bundle(t2);
    let t3 = Vec2 {
        x: 40.0 * factor,
        y: -30.0,
    };
    let n3 = get_node_bundle(t3);
    let t4 = Vec2 {
        x: 100.0 * factor,
        y: 20.0,
    };
    let n4 = get_node_bundle(t4);

    let e1 = commands.spawn(n1).id();
    let e2 = commands.spawn(n2).id();
    let e3 = commands.spawn(n3).id();
    let e4 = commands.spawn(n4).id();

    commands.spawn(get_track_bundle(e1, e2, &t1, &t2));
    commands.spawn(get_track_bundle(e2, e3, &t2, &t3));
    commands.spawn(get_track_bundle(e3, e4, &t3, &t4));
    // commands.spawn(EdgeComp {
    //     a: e2,
    //     b: e3,
    //     pos_a: &n2.1.transform,
    //     pos_b: &n3.1.transform,
    // });
    // commands.spawn(EdgeComp {
    //     a: e3,
    //     b: e4,
    //     pos_a: &n3.1.transform,
    //     pos_b: &n4.1.transform,
    // });
}

// fn draw_nodes(
//     mut commands: Commands,
//     mut q_nodes: Query<(&mut Transform), (With<NodeComp>, With<FollowCursor>)>,
//     mouse: Res<MouseState>,
// ) {
//     for mut transform in q_nodes.iter_mut() {
//         transform.translation = mouse.position.extend(transform.translation.z);
//     }
// }

fn update_tracks(
    mut commands: Commands,
    mut q_tracks: Query<(Entity, &mut Path, &EdgeComp), With<Update>>,
) {
    for (entity, mut path, mut edge) in q_tracks.iter_mut() {
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(edge.pos_a);
        path_builder.line_to(edge.pos_b);
        *path = path_builder.build();
        commands.entity(entity).remove::<Update>();
    }
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
    )
}
