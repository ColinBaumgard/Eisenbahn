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

pub struct BuildingPlugin;
impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {}
}

impl Building {
    pub fn a_factory() -> Self {
        Building {
            inputs: vec![(String::from("B"), 5.0)],
            outputs: vec![(String::from("A"), 5.0)],
            storage: vec![(String::from("A"), 0.0), (String::from("B"), 0.0)],
            max_storage: vec![(String::from("A"), 100.0), (String::from("B"), 100.0)],
        }
    }
    pub fn b_factory() -> Self {
        Building {
            inputs: vec![(String::from("A"), 5.0)],
            outputs: vec![(String::from("B"), 5.0)],
            storage: vec![(String::from("A"), 0.0), (String::from("B"), 0.0)],
            max_storage: vec![(String::from("A"), 100.0), (String::from("B"), 100.0)],
        }
    }
}

pub fn get_info_bubble(
    pos: Vec2,
    building: &Building,
    font_handle: Handle<Font>,
) -> (
    InfoBubble,
    Text,
    bevy_prototype_lyon::entity::ShapeBundle,
    bevy_prototype_lyon::draw::Fill,
    bevy_prototype_lyon::draw::Stroke,
) {
    let arrow_diag = Vec2 { x: 10.0, y: 10.0 };
    let bubble = Vec2 { x: 40.0, y: 30.0 };
    let a = pos;
    let b = a + arrow_diag;
    let c = b + Vec2::new(bubble.x / 2.0 - arrow_diag.x, 0.);
    let d = c + Vec2::new(0., bubble.y);
    let e = d - Vec2::new(bubble.x, 0.);
    let f = e - Vec2::new(0., bubble.y);
    let g = f + Vec2::new(bubble.x / 2.0 - arrow_diag.x, 0.);

    let mut path_builder = PathBuilder::new();
    path_builder.move_to(pos);
    path_builder.line_to(a);
    path_builder.move_to(a);
    path_builder.line_to(b);
    path_builder.move_to(b);
    path_builder.line_to(c);
    path_builder.move_to(c);
    path_builder.line_to(d);
    path_builder.move_to(d);
    path_builder.line_to(e);
    path_builder.move_to(e);
    path_builder.line_to(f);
    path_builder.move_to(f);
    path_builder.line_to(g);
    path_builder.move_to(g);
    path_builder.line_to(a);
    path_builder.move_to(a);
    let mut path = path_builder.build();
    (
        InfoBubble,
        Text::from_section(
            "test",
            TextStyle {
                font: font_handle.clone(),
                font_size: 60.0,
                color: Color::WHITE,
            },
        ),
        ShapeBundle {
            path: path,
            transform: Transform::from_translation(Vec2::ZERO.extend(layer::INFO)),
            ..default()
        },
        Fill::color(Color::GRAY),
        Stroke::new(Color::WHITE, 1.0),
    )
}

pub fn get_building_sprite_bundle(
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
                extents: Vec2 { x: 10.0, y: 10.0 },
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
        Stroke::new(Color::GREEN, 1.0),
    )
}
