use crate::{layer, mouse, num, ColorNames, GameColors, MouseState, ORANGE, PURPLE};

use bevy::{
    math,
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};

use bevy_prototype_lyon::{entity::*, prelude::*};
use petgraph::{graphmap::GraphMap, Undirected};

pub struct EisenbahnPlugin;
impl Plugin for EisenbahnPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialise_world);

        let mut graph = GraphMap::<Entity, f32, Undirected>::new();
        let mut track_graph = TrackGraph(graph);
        app.insert_resource(track_graph);
    }
}

fn initialise_world(
    mut commands: Commands,
    game_colors: Res<GameColors>,
    mut graph: ResMut<TrackGraph>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let p1 = Vec2 { x: 100.0, y: 50.0 };
    let p2 = Vec2 {
        x: -100.0,
        y: -50.0,
    };
    let p3 = Vec2 { x: 34.0, y: -50.0 };
    let p4 = Vec2 { x: 200.0, y: 60.0 };
    let p5 = Vec2 { x: -70.0, y: 27.0 };
    let e1 = add_node(&mut commands, p1, &mut graph.0);
    let e2 = add_node(&mut commands, p2, &mut graph.0);
    let e3 = add_node(&mut commands, p3, &mut graph.0);
    let e4 = add_node(&mut commands, p4, &mut graph.0);
    let e5 = add_node(&mut commands, p5, &mut graph.0);
    add_edge(&mut commands, e1, p1, e2, p2, &mut graph.0);
    add_edge(&mut commands, e2, p2, e3, p3, &mut graph.0);
    add_edge(&mut commands, e5, p5, e3, p3, &mut graph.0);
    let t5 = add_edge(&mut commands, e1, p1, e5, p5, &mut graph.0);

    add_train(&mut commands, t5, 0.5);
}

#[derive(Resource)]
pub struct TrackGraph(GraphMap<Entity, f32, Undirected>);

#[derive(Component)]
pub struct Position(Vec2);
#[derive(Component)]
pub struct Velocity {
    value: f32,
    range: num::Range,
}
impl Default for Velocity {
    fn default() -> Self {
        Velocity {
            value: 0.,
            range: num::Range {
                left: 0.,
                right: 1.0,
            },
        }
    }
}
#[derive(Component)]
pub struct Acceleration {
    value: f32,
    range: num::Range,
}
impl Default for Acceleration {
    fn default() -> Self {
        Acceleration {
            value: 0.,
            range: num::Range {
                left: 0.,
                right: 1.0,
            },
        }
    }
}
#[derive(Component)]
pub struct LinearPosition {
    track_id: Entity,
    value: f32,
}

#[derive(Component)]
pub struct TrackNode;

#[derive(Component)]
pub struct Track {
    e1: Entity,
    p1: Vec2,
    e2: Entity,
    p2: Vec2,
}

#[derive(Component)]
pub struct Train;

fn add_node(
    commands: &mut Commands,
    position: Vec2,
    graph: &mut GraphMap<Entity, f32, Undirected>,
) -> Entity {
    let sprite = GeometryBuilder::build_as(
        &shapes::Circle {
            radius: 5.0,
            center: Vec2::splat(0.0),
        },
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::RED),
            outline_mode: StrokeMode::new(Color::RED, 2.0),
        },
        Transform::from_translation(position.extend(layer::CURSOR)),
    );
    let entity = commands.spawn((sprite, TrackNode, Position(position))).id();
    graph.add_node(entity);
    entity
}

fn remove_node(
    commands: &mut Commands,
    entity: Entity,
    graph: &mut GraphMap<Entity, f32, Undirected>,
) {
    graph.remove_node(entity);
    commands.entity(entity).despawn();
}

fn add_edge(
    commands: &mut Commands,
    entity1: Entity,
    position1: Vec2,
    entity2: Entity,
    position2: Vec2,
    graph: &mut GraphMap<Entity, f32, Undirected>,
) -> Entity {
    graph.add_edge(entity1, entity2, position1.distance(position2));

    commands
        .spawn((
            Track {
                e1: entity1,
                p1: position1,
                e2: entity2,
                p2: position2,
            },
            GeometryBuilder::build_as(
                &shapes::Line(position1, position2),
                DrawMode::Stroke(StrokeMode::new(ORANGE, 2.0)),
                Transform::from_xyz(0.0, 0.0, layer::TRACKS),
            ),
        ))
        .id()
}

fn add_train(commands: &mut Commands, track_id: Entity, lin_pos: f32) -> Entity {
    let sprite = GeometryBuilder::build_as(
        &shapes::Circle {
            radius: 3.0,
            center: Vec2::splat(0.0),
        },
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::GREEN),
            outline_mode: StrokeMode::new(Color::GREEN, 2.0),
        },
        Transform::from_xyz(0.0, 0.0, layer::TRAINS),
    );
    let entity = commands
        .spawn((
            sprite,
            Train,
            LinearPosition {
                track_id: track_id,
                value: lin_pos,
            },
        ))
        .id();
    entity
}
