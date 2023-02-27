#![allow(unused_imports)]
#![allow(unused)]

use crate::{eisenbahn::*, mouse::*};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use std::collections::HashMap;

mod eisenbahn;
mod layer;
mod mouse;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Msaa { samples: 4 });

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: 1200.,
            height: 800.,
            title: String::from("Eisenbahn"),
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    }))
    .add_plugin(ShapePlugin)
    .add_startup_system(setup_system)
    .add_plugin(EisenbahnPlugin)
    .add_plugin(MousePlugin);

    app.init_resource::<GameColors>();

    app.run();
}

fn setup_system(
    mut commands: Commands,
    assett_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    // Camera
    let mut camera = Camera2dBundle::default();
    camera.transform.translation.y -= 10.0;
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
