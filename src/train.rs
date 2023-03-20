use std::collections::HashMap;

use crate::{
    components::*,
    draw::*,
    in_game_components::*,
    input,
    layer::{self, CURSOR},
    num, ColorNames, GameColors, MouseState, ORANGE, PURPLE,
};

use bevy::{
    ecs::query::WorldQuery,
    math,
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};
use bevy_prototype_lyon::{
    entity::*,
    prelude::{tess::FillTessellator, *},
    shapes::Rectangle,
};

pub struct TrainPlugin;
impl Plugin for TrainPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system(run_buildings_system);
    }
}

pub fn get_train_sprite(
    pos: Vec2,
) -> (
    NodeComp,
    bevy_prototype_lyon::entity::ShapeBundle,
    bevy_prototype_lyon::draw::Fill,
    bevy_prototype_lyon::draw::Stroke,
) {
    (
        NodeComp,
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2 { x: 10.0, y: 5.0 },
                origin: RectangleOrigin::CustomCenter(Vec2::ZERO),
            }),
            transform: Transform::from_translation(pos.extend(layer::NODES)),
            ..default()
        },
        Fill::color(Color::Hsla {
            hue: 0.,
            saturation: 0.,
            lightness: 0.,
            alpha: 0.,
        }),
        Stroke::new(Color::PINK, 1.0),
    )
}
