use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Resource)]
pub struct InGameResources(Vec<String>);

#[derive(Component)]
pub struct Building {
    pub name: String,
    pub resources: HashMap<String, ResourceData>,
}

#[derive(Component)]
pub struct InfoBubble;

#[derive(Clone)]
pub struct ResourceData {
    pub stock: f32,
    pub rate: f32,
    pub max_stock: f32,
}
