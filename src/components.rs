use bevy::prelude::*;

#[derive(Component)]
pub struct Dragged;

#[derive(Component)]
pub struct Selected;
#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Ghost;

#[derive(Component)]
pub struct EdgeComp {
    pub a: Entity,
    pub b: Entity,
    pub pos_a: Vec2,
    pub pos_b: Vec2,
}
#[derive(Component)]
pub struct NodeComp;

#[derive(Component)]
pub struct Update;
