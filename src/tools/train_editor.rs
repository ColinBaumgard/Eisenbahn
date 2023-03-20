use crate::{
    building::*,
    components::{self, *},
    draw::*,
    in_game_components::*,
    input, layer,
    num::{self, vec2_to_pos2},
    tracks::*,
    train::get_train_sprite,
    ColorNames, GameColors, InputPlugin, MouseState, ORANGE, PURPLE,
};

use bevy::{
    ecs::{query::QuerySingleError, schedule::SystemConfig},
    input::{keyboard::KeyboardInput, ButtonState},
    math,
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_prototype_lyon::prelude::Stroke;

pub struct TrainEditorPlugin;
impl Plugin for TrainEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            enter_system
                .run_if(resource_changed::<Tool>())
                .run_if(resource_equals(Tool::TrainEditor)),
            exit_system
                .run_if(resource_changed::<Tool>())
                .run_if(not(resource_equals(Tool::TrainEditor))),
        ));

        app.add_systems((
            track_hook_system.run_if(resource_equals(Tool::TrainEditor)),
            update_trains_system.run_if(resource_equals(Tool::TrainEditor)),
        ));
    }
}

fn enter_system(
    mut commands: Commands,
    q_nodes: Query<Entity, (With<NodeComp>, Without<Building>, Without<HookedOnCursor>)>,
    q_bubble: Query<Entity, With<InfoBubble>>,
) {
    for e in q_nodes.iter() {
        commands.entity(e).insert(Visibility::Hidden);
    }
    for e in q_bubble.iter() {
        commands.entity(e).despawn();
    }
}

fn exit_system(
    mut commands: Commands,
    q_to_be_deselected: Query<Entity, With<Selected>>,
    q_track_hook: Query<Entity, With<HookedOnTrack>>,
) {
    for e in q_to_be_deselected.iter() {
        commands.entity(e).remove::<Selected>();
    }
    for e in q_track_hook.iter() {
        commands.entity(e).despawn();
    }
}

fn track_hook_system(
    mut commands: Commands,
    mouse: Res<MouseState>,
    buttons: Res<Input<MouseButton>>,
    q_edges: Query<(Entity, &EdgeComp)>,
    q_trains: Query<&TrackPosition>,
    q_hooked: Query<Entity, With<HookedOnTrack>>,
) {
    for e in q_hooked.iter() {
        commands.entity(e).despawn();
    }
    let mut pos: Option<(f32, Vec2, TrackPosition)> = None;
    for (entity, edge) in q_edges.iter() {
        let u = edge.pos_b - edge.pos_a;
        let u_normalised = u.normalize();
        let v = mouse.position - edge.pos_a;
        let lin_pos = u_normalised.dot(v).clamp(0.0, u.length());
        let d = v.distance(lin_pos * u_normalised);
        if d > 20.0 {
            continue;
        }
        if pos.is_none() {
            pos = Some((
                d,
                edge.pos_a + lin_pos * u_normalised,
                TrackPosition {
                    current_track: entity,
                    lin_pos: lin_pos,
                },
            ));
        } else if pos.clone().unwrap().0 > d {
            pos = Some((
                d,
                edge.pos_a + lin_pos * u_normalised,
                TrackPosition {
                    current_track: entity,
                    lin_pos: lin_pos,
                },
            ));
        }
    }

    if pos.is_some() {
        let (_, w_pos, track_pos) = pos.unwrap();
        let mut train_sprite = get_train_sprite(w_pos);
        match buttons.just_pressed(MouseButton::Left) {
            true => {
                commands.spawn(train_sprite);
            }
            false => {
                train_sprite.3 = Stroke::color(Color::WHITE);
                commands.spawn((train_sprite, HookedOnTrack));
            }
        };
    }
}

fn update_trains_system(
    mut commands: Commands,
    q_pos: Query<&TrackPosition, Without<Train>>,
    q_trains: Query<&TrackPosition, With<Train>>,
    q_edge: Query<&EdgeComp>,
) {
    for pos in q_pos.iter() {
        let edge = q_edge.get(pos.current_track);
        if edge.is_ok() {
            let edge_comp = edge.unwrap();
            commands.spawn(get_train_sprite(track_pos_to_vec2(pos, edge_comp)));
        }
    }
}

pub fn track_pos_to_vec2(pos: &TrackPosition, edge: &EdgeComp) -> Vec2 {
    edge.pos_a + (edge.pos_b - edge.pos_a).normalize() * pos.lin_pos
}
