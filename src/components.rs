use bevy::prelude::*;
use petgraph::graphmap::*;

#[derive(Component)]
pub struct Dragged;

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Ghost;
#[derive(Component)]
pub struct ActiveGhost;

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

#[derive(Resource)]
pub struct UpdateSelection {
    pub entities: Vec<Entity>,
}
#[derive(Resource)]
pub struct RemoveSelection {
    pub entities: Vec<Entity>,
}

#[derive(Resource)]
pub struct DragEnable;
#[derive(Resource)]
pub struct SelectEnable;

#[derive(Component)]
pub struct NormalColor(pub Color);
#[derive(Component)]
pub struct SelectedColor(pub Color);
#[derive(Component)]
pub struct DraggedColor(pub Color);
#[derive(Component)]
pub struct GhostColor(pub Color);

pub struct LeftMouseEvent;

pub enum GhostAction {
    Add,
    Remove,
    Deghost,
    Update,
}
pub struct GhostEvent {
    pub action: GhostAction,
    pub entities: Vec<Entity>,
}
impl GhostEvent {
    pub fn add(entities: Vec<Entity>) -> GhostEvent {
        GhostEvent {
            action: GhostAction::Add,
            entities: entities,
        }
    }
    pub fn update(entities: Vec<Entity>) -> GhostEvent {
        GhostEvent {
            action: GhostAction::Update,
            entities: entities,
        }
    }
    pub fn remove(entities: Vec<Entity>) -> GhostEvent {
        GhostEvent {
            action: GhostAction::Remove,
            entities: entities,
        }
    }
    pub fn deghost(entities: Vec<Entity>) -> GhostEvent {
        GhostEvent {
            action: GhostAction::Deghost,
            entities: entities,
        }
    }
}

// New components :

#[derive(Resource, Debug)]
pub enum Tool {
    Viewer,
    TrackEditor,
}

#[derive(Resource, Debug, PartialEq)]
pub enum TrackEditorMode {
    Idle,
    PathBuilding,
}

#[derive(Component)]
pub struct HookedOnCursor;

#[derive(Component)]
pub struct Moved;

#[derive(Resource)]
pub struct TrackGraph(pub UnGraphMap<Entity, Entity>);

#[derive(Component)]
pub struct AttractionRing(pub Entity);

#[derive(Component)]
pub struct Selected;
