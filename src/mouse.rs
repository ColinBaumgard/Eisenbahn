use crate::MainCamera;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::tess::geom::euclid::default;

#[derive(Resource, Default, Debug)]
pub struct MouseState {
    pub position: Vec2,
    pub window_position: Vec2,
}

pub struct MousePlugin;
impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(mouse_movement_system);

        app.init_resource::<MouseState>();
    }
}

fn mouse_movement_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse: ResMut<MouseState>,
    windows: Res<Windows>,
    q_camera: Query<&GlobalTransform, With<MainCamera>>,
) {
    let camera_transform = q_camera.single();

    for event in cursor_moved_events.iter() {
        let window = windows.get(event.id).unwrap();
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let pos = event.position - size / 2.0;

        mouse.position = (camera_transform.compute_matrix() * pos.extend(0.0).extend(1.0))
            .truncate()
            .truncate();

        mouse.window_position = event.position;
    }
}
