use crate::{
    building::*,
    components::{self, *},
    draw::*,
    in_game_components::*,
    input, layer,
    num::{self, vec2_to_pos2},
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
use bevy_egui::{egui, EguiContexts, EguiPlugin};

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
    q_bubble: Query<Entity, With<InfoBubble>>,
) {
    for e in q_nodes.iter() {
        commands.entity(e).insert(Visibility::Hidden);
    }
    for e in q_bubble.iter() {
        commands.entity(e).despawn();
    }
}

fn exit_system(mut commands: Commands, q_to_be_deselected: Query<Entity, With<Selected>>) {
    for e in q_to_be_deselected.iter() {
        commands.entity(e).remove::<Selected>();
    }
}

fn building_info_system(
    mut commands: Commands,
    mut contexts: EguiContexts,
    window_size: Res<MainWindowSize>,
    mouse: Res<MouseState>,
    q_bubble: Query<Entity, With<InfoBubble>>,
    q_building: Query<(Entity, &Building, &Transform), With<Selected>>,
    asset_server: Res<AssetServer>,
) {
    for (e, building, transform) in q_building.iter() {
        let mut lines = Vec::new();
        for (name, data) in &building.resources {
            let (stock, max_stock) = (data.stock, data.max_stock);
            lines.push(format!("{name:} : {stock:.0}/{max_stock:.0}"));
        }
        lines.sort();
        egui::Window::new(building.name.clone())
            .current_pos(vec2_to_pos2(
                transform.translation.truncate(),
                window_size.0,
            ))
            .show(contexts.ctx_mut(), |ui| {
                for line in lines {
                    ui.label(line);
                }
            });
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
