#![allow(unused_imports)]
#![allow(unused)]

use crate::{
    components::*, eisenbahn::*, gui::*, input::*, num::*, terrain::*, tools::*, tracks::*,
};

use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_prototype_lyon::prelude::*;

use std::{any::Any, collections::HashMap};

mod building;
mod components;
mod draw;
mod eisenbahn;
mod graph;
mod gui;
mod in_game_components;
mod input;
mod layer;
mod num;
mod terrain;
mod tools;
mod tracks;
mod train;

fn main() {
    let mut app = App::new();

    let window_size = MainWindowSize(Vec2::new(1200.0, 800.0));
    app.insert_resource(window_size);

    let window = Some(Window {
        resolution: WindowResolution::new(window_size.0.x, window_size.0.y),

        ..default()
    });

    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Msaa::default());

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: window,
        ..default()
    }))
    .add_plugin(ShapePlugin)
    .add_startup_system(setup_system)
    .add_plugin(EisenbahnPlugin);

    app.init_resource::<GameColors>();

    app.run();
}

fn setup_system(mut commands: Commands, assett_server: Res<AssetServer>) {
    // Camera
    let mut camera = Camera2dBundle::default();
    // camera.transform.translation.z -= 10.0;
    commands.spawn((MainCamera, camera));

    println!("Ready to play !")
}

#[derive(Component)]
struct MainCamera;

#[derive(Eq, Hash, PartialEq)]
pub enum ColorNames {
    BLUE,
    PURPLE,
}

#[derive(Resource)]

pub struct GameColors {
    pub map: HashMap<ColorNames, Color>,
}
impl Default for GameColors {
    fn default() -> Self {
        let mut map = HashMap::new();

        map.insert(
            ColorNames::BLUE,
            Color::rgb(65.0 / 255.0, 71.0 / 255.0, 112.0 / 255.0),
        );
        map.insert(
            ColorNames::PURPLE,
            Color::rgb(55.0 / 255.0, 34.0 / 255.0, 72.0 / 255.0),
        );
        GameColors { map: map }
    }
}

// Const
const BACKGROUND_COLOR: Color = Color::rgb(65.0 / 255.0, 71.0 / 255.0, 112.0 / 255.0);
const LIGHT_BLUE: Color = Color::rgb(91.0 / 255.0, 133.0 / 255.0, 170.0 / 255.0);
const ORANGE: Color = Color::rgb(244.0 / 255.0, 96.0 / 255.0, 54.0 / 255.0);
const PURPLE: Color = Color::rgb(55.0 / 255.0, 34.0 / 255.0, 72.0 / 255.0);
const DARK: Color = Color::rgb(23.0 / 255.0, 17.0 / 255.0, 35.0 / 255.0);

const HEIGHT: f32 = 800.0;
const WIDTH: f32 = 1200.0;
