use crate::{
    building::*,
    components::{self, *},
    draw::*,
    in_game_components::*,
    input, layer, num,
    tracks::*,
    ColorNames, GameColors, InputPlugin, MouseState, ORANGE, PURPLE,
};

use bevy::{
    ecs::{query::QuerySingleError, schedule::SystemConfig},
    input::{keyboard::KeyboardInput, ButtonState},
    math,
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};

pub struct ViewerPlugin;
impl Plugin for ViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            enter_system
                .run_if(resource_changed::<Tool>())
                .run_if(resource_equals(Tool::Viewer)),
            exit_system
                .run_if(resource_changed::<Tool>())
                .run_if(not(resource_equals(Tool::Viewer))),
        ));

        app.add_systems((
            position_hook_system.run_if(resource_equals(Tool::Viewer)),
            building_info_system.run_if(resource_equals(Tool::Viewer)),
        ));
    }
}

fn enter_system(
    mut commands: Commands,
    q_nodes: Query<Entity, (With<NodeComp>, Without<Building>)>,
) {
    for e in q_nodes.iter() {
        commands.entity(e).insert(Visibility::Hidden);
    }
}

fn exit_system(mut commands: Commands, q_to_be_deselected: Query<Entity, With<Selected>>) {
    for e in q_to_be_deselected.iter() {
        commands.entity(e).remove::<Selected>();
    }
}

fn building_info_system(
    mut commands: Commands,
    q_bubble: Query<Entity, With<InfoBubble>>,
    q_building: Query<(Entity, &Building, &Transform), With<Selected>>,
    asset_server: Res<AssetServer>,
) {
    for e in q_bubble.iter() {
        commands.entity(e).despawn();
    }
    for (e, b, t) in q_building.iter() {
        let info_bubble = get_info_bubble(
            t.translation.truncate(),
            b,
            asset_server.load("fonts/FiraSans-Bold.ttf"),
        );
        commands.spawn(info_bubble);
    }
}

fn position_hook_system(
    mut commands: Commands,
    mouse: Res<MouseState>,
    buttons: Res<Input<MouseButton>>,
    q_selected: Query<Entity, With<Selected>>,
    q_nodes: Query<(Entity, &Transform), With<Building>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    for e in q_selected.iter() {
        commands.entity(e).remove::<Selected>();
    }
    let mut closest_node = q_nodes.iter().min_by(|a, b| {
        mouse
            .position
            .distance(a.1.translation.truncate())
            .total_cmp(&mouse.position.distance(b.1.translation.truncate()))
    });
    if closest_node.is_some()
        && mouse
            .position
            .distance(closest_node.unwrap().1.translation.truncate())
            < 10.0
    {
        let (entity, _) = closest_node.unwrap();
        commands.entity(entity).insert(Selected);
    }
}
