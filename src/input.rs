use crate::{MainCamera, HEIGHT, WIDTH};

use bevy::{
    input::{keyboard::*, ButtonState},
    prelude::*,
};
use bevy_prototype_lyon::prelude::tess::geom::euclid::default;

use std::{any::Any, collections::HashMap};

#[derive(Resource, Default, Debug)]
pub struct MouseState {
    pub position: Vec2,
    pub window_position: Vec2,
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(mouse_movement_system);

        app.init_resource::<MouseState>();
    }
}

fn mouse_movement_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse: ResMut<MouseState>,
    q_camera: Query<&GlobalTransform, With<MainCamera>>,
    buttons: Res<Input<MouseButton>>,
) {
    let camera_transform = q_camera.single();

    for event in cursor_moved_events.iter() {
        let size = Vec2::new(WIDTH, HEIGHT);
        let pos = event.position - size / 2.0;

        mouse.window_position = event.position;
        mouse.position = (camera_transform.compute_matrix() * pos.extend(0.0).extend(1.0))
            .truncate()
            .truncate();
    }
}

//     buttons: Res<Input<MouseButton>>,
// ) {
//     if buttons.just_pressed(MouseButton::Left) {
//         let node_bundle = get_node_bundle(mouse.position, track_graph);
//         commands.spawn((node_bundle.0, node_bundle.1, OnEdit));
//     }
// }
