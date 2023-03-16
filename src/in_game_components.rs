use bevy::prelude::*;

#[derive(Resource)]
pub struct InGameResources(Vec<String>);

#[derive(Component)]
pub struct Building {
    pub inputs: Vec<(String, f32)>,
    pub outputs: Vec<(String, f32)>,
    pub storage: Vec<(String, f32)>,
    pub max_storage: Vec<(String, f32)>,
}

#[derive(Component)]
pub struct InfoBubble;
