use crate::{
    components::{self, *},
    draw::*,
    input, layer, num,
    tracks::*,
    ColorNames, GameColors, InputPlugin, MouseState, ORANGE, PURPLE,
    {EdgeComp, TrackGraph, TrackNode, TrackWeight},
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

pub struct ToolPlugin;
impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        let mut current_tool = Tool::None;
        app.insert_resource(current_tool);

        app.add_systems((tool_wheel_system, selection_system));

        app.add_system(move_dragged_system);

        app.add_system(
            set_track_tool_system
                .run_if(resource_changed::<Tool>())
                .run_if(resource_equals(Tool::TrackBuilder)),
        );
        app.add_systems((
            track_tool_selection_system.run_if(resource_equals(Tool::TrackBuilder)),
            track_tool_ghost_system
                .run_if(resource_equals(Tool::TrackBuilder))
                .after(track_tool_selection_system),
        ));

        app.add_systems((
            update_selection_system.run_if(resource_exists::<UpdateSelection>()),
            remove_selection_system.run_if(resource_exists::<RemoveSelection>()),
            update_ghost_system,
        ));

        app.add_event::<GhostEvent>();

        // app.add_system(
        //     set_default_system
        //         .run_if(resource_changed::<Tool>())
        //         .run_if(resource_equals(Tool::None)),
        // );
    }
}

fn track_tool_ghost_system(
    mut commands: Commands,
    mouse: Res<MouseState>,
    mut ev_left: EventReader<LeftMouseEvent>,
    q_nodes: Query<(Entity, &Transform, Option<&Selected>), With<NodeComp>>,
    mut ev_ghost: EventWriter<GhostEvent>,
    keys: Res<Input<KeyCode>>,
) {
    if !ev_left.is_empty() {
        let mut selected_nodes = Vec::new();
        for (e, t, is_selected) in q_nodes.iter() {
            let d = t.translation.truncate().distance(mouse.position);
            if d < 10.0 {
                return;
            }
            if let Some(_) = is_selected {
                selected_nodes.push((e, t.translation.truncate()));
            }
        }
        if selected_nodes.len() == 0 {
            let mut node = get_node_bundle(mouse.position);
            let e = commands.spawn((node)).id();
            commands.insert_resource(UpdateSelection { entities: vec![e] });
            ev_ghost.send(GhostEvent::update(vec![e]));
            ev_left.clear();
            println!("Spawned new node");
        } else if selected_nodes.len() == 1 {
            let mut node = get_node_bundle(mouse.position);
            let e_node = commands.spawn(node).insert(FollowCursor).id();
            let mut track = get_track_bundle(
                selected_nodes[0].0,
                e_node,
                &selected_nodes[0].1,
                &mouse.position,
            );
            let e_track = commands.spawn(track).id();
            ev_ghost.send(GhostEvent::add(vec![e_node, e_track]))
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        ev_ghost.send(GhostEvent::remove(Vec::new()));
    }
    // else if buttons.just_released(MouseButton::Left) {
    //     for (e, t, d, _) in q_nodes.iter() {
    //         if let Some(_) = d {
    //             commands.entity(e).remove::<Dragged>();
    //         }
    //     }
    // }

    // if buttons.just_pressed(MouseButton::Left) {
    //     let selected_node = match q_selected.get_single() {
    //         Ok(e) => e,
    //         Err(QuerySingleError::NoEntities(_)) => commands
    //             .spawn(get_node_bundle(mouse.position))
    //             .insert(Selected)
    //             .insert(Dragged)
    //             .id(),
    //         Err(QuerySingleError::MultipleEntities(_)) => {
    //             return;
    //         }
    //     };
    // }
}
fn track_tool_selection_system(
    mut commands: Commands,
    mouse: Res<MouseState>,
    mut ev_left: EventReader<LeftMouseEvent>,
    keys: Res<Input<KeyCode>>,
    q_nodes: Query<(Entity, &Transform), With<NodeComp>>,
) {
    if !ev_left.is_empty() {
        for (e, t) in q_nodes.iter() {
            let d = t.translation.truncate().distance(mouse.position);
            if d < 10.0 {
                commands.insert_resource(UpdateSelection { entities: vec![e] });
                ev_left.clear();
                println!("{:?}", ev_left.is_empty());
                return;
            }
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        commands.insert_resource(RemoveSelection {
            entities: Vec::new(),
        });
    }
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    TrackBuilder,
    None,
}
impl Tool {
    pub fn as_vec() -> Vec<Tool> {
        Vec::from([Tool::None, Tool::TrackBuilder])
    }
    pub fn next(current_tool: &Tool) -> Tool {
        let mut last_el = *Tool::as_vec().last().unwrap();
        let mut iter = Tool::as_vec();
        for el in iter {
            if last_el == *current_tool {
                return el;
            }
            last_el = el;
        }
        last_el
    }
}

fn move_dragged_system(
    mut commands: Commands,
    mouse: Res<MouseState>,
    mut q_dragged_nodes: Query<(Entity, &mut Transform), With<Dragged>>,
    mut q_tracks: Query<(Entity, &mut Path, &mut EdgeComp)>,
) {
    match q_dragged_nodes.get_single_mut() {
        Ok((dragged_node, mut transform)) => {
            transform.translation.x = mouse.position.x;
            transform.translation.y = mouse.position.y;
            let mut found = false;

            for (entity, mut path, mut edge) in q_tracks.iter_mut() {
                if edge.a == dragged_node {
                    edge.pos_a = mouse.position;
                    found = true;
                } else if edge.b == dragged_node {
                    edge.pos_b = mouse.position;
                    found = true;
                }
                if found {
                    commands.entity(entity).insert(Update);
                }
            }
        }
        other => (),
    }
}

fn tool_wheel_system(
    mut commands: Commands,
    mut current_tool: ResMut<Tool>,
    mut key_evr: EventReader<KeyboardInput>,
    mut q_ghost: Query<(Entity), With<Ghost>>,
) {
    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => match ev.key_code {
                Some(KeyCode::Tab) => {
                    *current_tool.as_mut() = Tool::next(current_tool.as_ref());
                    for entity in q_ghost.iter() {
                        commands.entity(entity).despawn();
                    }
                }
                other => (),
            },
            ButtonState::Released => {}
        }
    }
}

fn is_tool(current_tool: Res<Tool>, tool: &Tool) -> bool {
    current_tool.as_ref() == tool
}

fn selection_system(
    mut commands: Commands,
    q_selectable: Query<Entity, With<Selectable>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        for entity in q_selectable.iter() {
            commands.entity(entity).remove::<Selected>();
        }
    }
}
fn ghost_system(
    mut commands: Commands,
    q_ghosted: Query<Entity, With<Ghost>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        for entity in q_ghosted.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn set_default_system(mut commands: Commands) {
    commands.remove_resource::<DragEnable>();
    commands.insert_resource(SelectEnable);
}
fn set_track_tool_system(mut commands: Commands) {
    commands.insert_resource(DragEnable);
    commands.insert_resource(SelectEnable);
}

fn interaction_system(
    mut commands: Commands,
    q_nodes: Query<(Entity, &Transform, Option<&Dragged>, Option<&Selected>), With<NodeComp>>,
    mouse: Res<MouseState>,
    buttons: Res<Input<MouseButton>>,
    select_enable: Option<Res<SelectEnable>>,
    drag_enable: Option<Res<DragEnable>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for (e, t, _, is_selected) in q_nodes.iter() {
            let d = t.translation.truncate().distance(mouse.position);
            if d < 5.0 {
                if let Some(_) = drag_enable {
                    commands.entity(e).insert(Dragged);
                }
                if let Some(_) = select_enable {
                    commands.insert_resource(UpdateSelection { entities: vec![e] });
                }
                return;
            }
        }
        commands.insert_resource(RemoveSelection {
            entities: Vec::new(),
        });
    }
}

fn update_selection_system(
    mut commands: Commands,
    selection: Res<UpdateSelection>,
    mut q_selected: Query<(Entity, &mut Stroke, &NormalColor, &SelectedColor)>,
) {
    for (e, mut stroke, n_c, s_c) in q_selected.iter_mut() {
        if selection.entities.contains(&e) {
            commands.entity(e).insert(Selected);
            stroke.color = s_c.0;
        } else {
            commands.entity(e).remove::<Selected>();
            stroke.color = n_c.0;
        }
    }
    commands.remove_resource::<UpdateSelection>();
}
fn remove_selection_system(
    mut commands: Commands,
    selection: Res<RemoveSelection>,
    mut q_selected: Query<(Entity, &mut Stroke, &NormalColor, &SelectedColor)>,
) {
    for (e, mut stroke, n_c, s_c) in q_selected.iter_mut() {
        if selection.entities.contains(&e) || selection.entities.len() == 0 {
            commands.entity(e).remove::<Selected>();
            stroke.color = n_c.0;
        }
    }
    commands.remove_resource::<RemoveSelection>();
}
fn update_ghost_system(
    mut commands: Commands,
    mut g_events: EventReader<GhostEvent>,
    mut q_all: Query<(Entity, &mut Fill, &NormalColor, &GhostColor, Option<&Ghost>)>,
) {
    for ev in g_events.iter() {
        match ev.action {
            GhostAction::Add => {
                for (e, mut fill, n_c, g_c, is_ghosted) in q_all.iter_mut() {
                    if ev.entities.contains(&e) {
                        commands.entity(e).insert(Ghost);
                        fill.color = g_c.0;
                    }
                }
            }
            GhostAction::Update => {
                for (e, mut fill, n_c, g_c, is_ghosted) in q_all.iter_mut() {
                    if ev.entities.contains(&e) {
                        commands.entity(e).insert(Ghost);
                        fill.color = g_c.0;
                    } else {
                        commands.entity(e).remove::<Ghost>();
                        fill.color = n_c.0;
                    }
                }
            }
            GhostAction::Remove => {
                for (e, mut fill, n_c, g_c, is_ghosted) in q_all.iter_mut() {
                    if (ev.entities.contains(&e) || ev.entities.len() == 0) && is_ghosted.is_some()
                    {
                        commands.entity(e).despawn();
                    }
                }
            }
            GhostAction::Deghost => {
                for (e, mut fill, n_c, g_c, is_ghosted) in q_all.iter_mut() {
                    if (ev.entities.contains(&e) || ev.entities.len() == 0) && is_ghosted.is_some()
                    {
                        commands.entity(e).remove::<Ghost>();
                        fill.color = n_c.0;
                    }
                }
            }
        }
    }
    g_events.clear();
}
// fn remove_selection_system(
//     mut commands: Commands,
//     selection: Res<RemoveSelection>,
//     mut q_selected: Query<(Entity, &mut Stroke, &NormalColor, &SelectedColor)>,
// ) {
//     for (e, mut stroke, n_c, s_c) in q_selected.iter_mut() {
//         if selection.entities.contains(&e) || selection.entities.len() == 0 {
//             commands.entity(e).remove::<Selected>();
//             stroke.color = n_c.0;
//         }
//     }
//     commands.remove_resource::<RemoveSelection>();
// }
