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

pub struct BuildingPlugin;
impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(run_buildings_system);
    }
}

fn run_buildings_system(mut q_buildings: Query<&mut Building>, time: Res<Time>) {
    let dt = time.raw_delta_seconds();
    for mut b in q_buildings.iter_mut() {
        let resources = b.resources.clone();
        let mut new_resources = HashMap::new();
        for (key, mut data) in resources {
            let mut new_data = data.clone();
            new_data.stock = data.stock + data.rate * dt;
            if new_data.stock > data.max_stock {
                return;
            }
            new_resources.insert(key, new_data);
        }
        b.resources = new_resources;
    }
}

impl Building {
    pub fn a_factory() -> Self {
        let res_a = ResourceData {
            stock: 0.,
            rate: 1.0,
            max_stock: 50.0,
        };
        let res_b = ResourceData {
            stock: 50.,
            rate: -1.0,
            max_stock: 50.0,
        };
        let mut resources = HashMap::new();
        resources.insert(String::from("A"), res_a);
        resources.insert(String::from("B"), res_b);
        Building {
            name: String::from("A Factory"),
            resources: resources,
        }
    }
    pub fn b_factory() -> Self {
        let res_a = ResourceData {
            stock: 50.,
            rate: -1.0,
            max_stock: 50.0,
        };
        let res_b = ResourceData {
            stock: 0.,
            rate: 1.0,
            max_stock: 50.0,
        };
        let mut resources = HashMap::new();
        resources.insert(String::from("A"), res_a);
        resources.insert(String::from("B"), res_b);
        Building {
            name: String::from("B Factory"),
            resources: resources,
        }
    }
}

pub fn get_info_bubble(
    pos: Vec2,
    building: &Building,
    font_handle: Handle<Font>,
) -> (
    InfoBubble,
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

    let text_pos = (e + f) * Vec2::new(0.5, 0.5);

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

    let text = Text::from_section(
        "test",
        TextStyle {
            font: font_handle.clone(),
            font_size: 60.0,
            color: Color::WHITE,
        },
    )
    .with_alignment(TextAlignment::Left);

    (
        InfoBubble,
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
