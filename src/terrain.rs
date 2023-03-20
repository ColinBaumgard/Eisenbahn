use std::collections::HashMap;

use crate::{
    components::*,
    draw::*,
    in_game_components::*,
    input,
    layer::{self, CURSOR},
    num, terrain, ColorNames, GameColors, MouseState, ORANGE, PURPLE,
};

use bevy::{
    ecs::query::WorldQuery,
    math,
    prelude::*,
    render::{mesh::*, render_resource::*, *},
    sprite::{Material2d, MaterialMesh2dBundle},
};
use bevy_prototype_lyon::{
    entity::*,
    prelude::{tess::FillTessellator, *},
    shapes::Rectangle,
};
use bracket_noise::prelude::*;
use optimization::*;
use rand::{thread_rng, Rng};

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system(grid_generation_system);

        app.insert_resource(Terrain::new(400.0));

        app.add_startup_systems((generation_system, mesh_system));
    }
}

fn mesh_system(
    mut commands: Commands,
    terrain: Res<Terrain>,
    window_size: Res<MainWindowSize>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();

    let step = 50.0;
    let (n, m) = (
        (window_size.0.x / step) as u32 + 1,
        (window_size.0.y / step) as u32 + 1,
    );

    let pos_middle = 0.5 * window_size.0;

    println!("{:?}, {:?}", n, m);

    let mut rng = thread_rng();

    for j in (0..m) {
        for i in (0..n) {
            let mut pos = Vec2::new(i as f32, j as f32) * step - pos_middle;
            vertices.push([pos.x, pos.y, 0.0]);
            let color: [f32; 3] = rng.gen();
            colors.push([color[0], color[1], color[2], 1.0]);
        }
    }

    let n2 = n - 1;
    let m2 = m - 1;

    for j in (0..m2) {
        for i in (0..n2) {
            let a = n * j + i;
            // println!("{:?}", a);
            indices.push(a);
            indices.push(a + 1);
            indices.push(a + n);

            indices.push(a + n + 1);
            indices.push(a + 1);
            indices.push(a + n);
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.set_indices(Some(Indices::U32(indices)));

    commands.spawn(ColorMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        material: materials.add(ColorMaterial::from(Color::YELLOW)),
        ..default()
    });

    println!("OK");
}

fn grid_generation_system(mut commands: Commands, window_size: Res<MainWindowSize>) {
    let grid_size = 50.0;
    let terrain_size = [6, 6];
    let mut path_builder = PathBuilder::new();

    for i in (0..=terrain_size[0]).rev() {
        let x = grid_size * i as f32;
        path_builder.move_to(Vec2::new(x, 0.0));
        path_builder.line_to(Vec2::new(x, terrain_size[1] as f32 * grid_size));
    }
    for i in (0..=terrain_size[1]).rev() {
        let y = grid_size * i as f32;
        path_builder.move_to(Vec2::new(0.0, y));
        path_builder.line_to(Vec2::new(terrain_size[0] as f32 * grid_size, y));
    }

    let mut path = path_builder.build();
    commands.spawn((
        ShapeBundle {
            path: path,
            transform: Transform::from_translation(Vec2::ZERO.extend(layer::INFO)),
            ..default()
        },
        Fill::color(Color::GRAY),
        Stroke::new(Color::WHITE, 1.0),
    ));
}

fn generation_system(mut commands: Commands, terrain: Res<Terrain>) {
    // let mut rng = RandomNumberGenerator::new();

    let d = 5;
    let factor = terrain.scale / d as f32;

    // for raw_x in (-d..=d).rev() {
    //     for raw_y in (-d..=d).rev() {
    //         let pos = Vec2::new(raw_x as f32, raw_y as f32) * factor;
    //         let z = terrain.z(pos) * 3.0 + 0.2;
    //         commands.spawn((
    //             ShapeBundle {
    //                 path: GeometryBuilder::build_as(&shapes::Circle {
    //                     radius: 3.0,
    //                     center: Vec2::ZERO,
    //                 }),
    //                 transform: Transform::from_translation(pos.extend(layer::INFO)),
    //                 ..default()
    //             },
    //             Fill::color(Color::Rgba {
    //                 red: 0.5 + z,
    //                 green: 0.0,
    //                 blue: 1.0 - z * 0.5,
    //                 alpha: 1.0,
    //             }),
    //             Stroke::new(Color::RED, 0.0),
    //         ));
    //     }
    // }
}

fn isolines_system(
    mut commands: Commands,
    terrain: Res<Terrain>,
    window_size: Res<MainWindowSize>,
) {
    let extrema: Vec<Vec2> = Vec::new();
    let n = 30;
    let factor_x = window_size.0.x / n as f32;
    let factor_y = window_size.0.y / n as f32;

    // let iso_z = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    let n_iso = 20;
    let step = 1.0 / n_iso as f32;

    let mut isolines = HashMap::new();
    for i in (0..=n_iso) {
        isolines.insert(i, Vec::<Vec2>::new());
    }

    for i in (-n..=n).rev() {
        for j in (-n..=n).rev() {
            let pos = Vec2::new(i as f32 * factor_x, j as f32 * factor_y);
            for i in (0..=n_iso) {
                // println!("{:?}", terrain.z(pos));
                let z = i as f32 * step;
                if let Some(p) = go_to_z_bisect(&terrain, pos, z, 100, factor_x, 0.001) {
                    // println!(" -> {:?} -> {:?}", z, terrain.z(p));
                    if let Some(path) =
                        follow_iso(&terrain, p, z, 1000, 10.0, isolines.get(&i).unwrap())
                    {
                        let color = Color::rgba(z / 0.5, 0.6 - z * 0.5, 0.5 - z * 0.5, 0.2);
                        // let color = Color::ORANGE;
                        commands.spawn((
                            ShapeBundle {
                                path: path,
                                transform: Transform::from_translation(
                                    Vec2::ZERO.extend(layer::ISOLINES),
                                ),
                                ..default()
                            },
                            Fill::color(color.clone()),
                            Stroke::new(color.clone(), 2.0),
                        ));
                        isolines.get_mut(&i).unwrap().push(p);
                    }
                }
            }
        }
    }
}

fn find_extrema(terrain: &Terrain, x_range: Vec2, y_range: Vec2, max: bool) -> Vec<Vec2> {
    let n = 10;

    let mut extrema_duplicate = Vec::new();
    let mut extrema = Vec::new();
    let step = match max {
        true => 1.0,
        false => -1.0,
    };

    for i in (0..=n).rev() {
        for j in (0..=n).rev() {
            let pos = Vec2::new(
                x_range.x + i as f32 * (x_range.y - x_range.x) / n as f32,
                y_range.x + j as f32 * (y_range.y - y_range.x) / n as f32,
            );
            if let Some(extre) = gradient_descent(&terrain, pos, 200, step) {
                extrema_duplicate.push(extre);
            }
        }
    }

    let mut duplicate = Vec::new();
    for i in (0..extrema_duplicate.len()) {
        if !duplicate.contains(&i) {
            for j in (i + 1..extrema_duplicate.len()) {
                if extrema_duplicate[i].distance(extrema_duplicate[j]) < 1.0 {
                    duplicate.push(j);
                }
            }
            extrema.push(extrema_duplicate[i])
        }
    }
    extrema
}

fn find_max(terrain: &Terrain, x_range: Vec2, y_range: Vec2) -> Option<Vec2> {
    let mut extrema = find_extrema(terrain, x_range, y_range, true);
    extrema.sort_by(|a, b| terrain.z(*a).total_cmp(&terrain.z(*b)));
    let mut values = Vec::new();
    for e in &extrema {
        values.push(terrain.z(*e));
    }
    extrema.last().copied()
}
fn find_min(terrain: &Terrain, x_range: Vec2, y_range: Vec2) -> Option<Vec2> {
    let mut extrema = find_extrema(terrain, x_range, y_range, false);
    extrema.sort_by(|a, b| terrain.z(*a).total_cmp(&terrain.z(*b)));
    let mut values = Vec::new();
    for e in &extrema {
        values.push(terrain.z(*e));
    }
    extrema.first().copied()
}

fn go_to_z(
    terrain: &Terrain,
    init_pos: Vec2,
    target_z: f32,
    max_iter: u32,
    tolerance: f32,
) -> Option<Vec2> {
    let mut pos = init_pos.clone();
    let max_step = 10.0;
    // println!("---------");
    let mut last_error = target_z - terrain.z(pos);
    for _ in (0..max_iter) {
        let error = target_z - terrain.z(pos);
        // println!("{:?}", error);
        if error.abs() < tolerance {
            return Some(pos);
        } else if last_error.abs() < error.abs() {
            return None;
        }
        let grad = terrain.grad(pos);
        let mut step = error * grad / grad.length_squared();
        if step.length() > max_step {
            step = step.normalize() * max_step;
        }
        pos += step;
        last_error = error;
    }
    None
}
fn go_to_z_bisect(
    terrain: &Terrain,
    init_pos: Vec2,
    target_z: f32,
    max_iter: u32,
    max_step: f32,
    tolerance: f32,
) -> Option<Vec2> {
    // println!("-----bisect---");
    let mut pos_a = init_pos.clone();
    let mut z_a = terrain.z(pos_a);
    let mut grad = terrain.grad(pos_a);

    let mut bounds = [pos_a, pos_a + Vec2::new(max_step, max_step)];
    let mut bounds_e = [
        terrain.z(bounds[0]) - target_z,
        terrain.z(bounds[1]) - target_z,
    ];
    // println!("{:?} : {:?}", bounds_e[0], bounds_e[1]);
    if bounds_e[0].signum() == bounds_e[1].signum() {
        // println!("NONE");
        return None;
    }

    // fn z_bounds(terrain: &Terrain, bounds: [Vec2; 2]) -> [f32; 2] {
    //     [terrain.z(bounds[0]), terrain.z(bounds[1])]
    // }

    for _ in (0..max_iter) {
        bounds_e = [
            terrain.z(bounds[0]) - target_z,
            terrain.z(bounds[1]) - target_z,
        ];
        // let factor = (bounds_e[0] / (bounds_e[1] - bounds_e[0]));
        let c = (bounds[0] + bounds[1]) / 2.0;
        let c_e = terrain.z(c) - target_z;
        // println!("{:?} : {:?} : {:?}", bounds_e[0], c_e, bounds_e[1]);
        if c_e.abs() < tolerance {
            // println!("OK");
            return Some(c);
        }
        if c_e.signum() == bounds_e[0].signum() {
            bounds[0] = c;
        } else {
            bounds[1] = c;
        }
    }

    None
}

fn gradient_descent(
    terrain: &Terrain,
    init_pos: Vec2,
    max_iteration: u32,
    step_size: f32,
) -> Option<Vec2> {
    let mut pos = init_pos.clone();
    let mut current_z = terrain.z(pos);
    let mut step = step_size;
    for _ in (0..max_iteration) {
        pos += terrain.grad(pos).normalize() * step;
        let next_z = terrain.z(pos);
        if step_size.signum() * next_z < step_size.signum() * current_z {
            step = step / 2.0;
        }
        if step.abs() < step_size.abs() / 10.0 {
            return Some(pos);
        }
        current_z = next_z;
    }
    None
}

fn follow_iso(
    terrain: &Terrain,
    init_pos: Vec2,
    target_z: f32,
    max_iteration: u32,
    step_size: f32,
    existing_iso: &Vec<Vec2>,
) -> Option<Path> {
    let mut path_builder = PathBuilder::new();

    path_builder.move_to(init_pos);
    let mut pos_l_prev = init_pos.clone();
    let mut pos_l = init_pos.clone();
    let mut pos_r_prev = init_pos.clone();
    let mut pos_r = init_pos.clone();
    for _ in (0..max_iteration).rev() {
        pos_l = iso_step(terrain, pos_l, target_z, step_size);
        pos_r = iso_step(terrain, pos_r, target_z, -step_size);
        if pos_r.distance(pos_l) < step_size {
            path_builder.move_to(pos_r_prev);
            path_builder.line_to(pos_l_prev);
            break;
        }
        for p in existing_iso {
            if p.distance(pos_l) < 2.0 * step_size || p.distance(pos_r) < 2.0 * step_size {
                return None;
            }
        }
        path_builder.move_to(pos_l_prev);
        path_builder.line_to(pos_l);
        path_builder.move_to(pos_r_prev);
        path_builder.line_to(pos_r);
        pos_l_prev = pos_l;
        pos_r_prev = pos_r;
    }
    Some(path_builder.build())
}

fn iso_step(terrain: &Terrain, pos: Vec2, target_z: f32, step_size: f32) -> Vec2 {
    let mut next_pos = pos
        + terrain
            .grad(pos)
            .normalize()
            .rotate(Vec2::new(num::HALF_PI.cos(), num::HALF_PI.sin()))
            * step_size;
    if let Some(p) = go_to_z(terrain, next_pos, target_z, 3, 0.0005) {
        return p;
    }
    return next_pos;
}

impl Terrain {
    pub fn new(scale: f32) -> Self {
        let mut rng = rand::thread_rng();
        let mut noise = FastNoise::seeded(rng.gen());
        noise.set_noise_type(NoiseType::PerlinFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(3);
        noise.set_fractal_gain(0.8);
        noise.set_fractal_lacunarity(2.0);
        noise.set_frequency(1.0);

        let mut t = Terrain {
            scale: scale,
            noise: noise,
            height_offset: 0.0,
            height_scale: 1.0,
        };
        t.auto_scale();
        t
    }

    pub fn auto_scale(&mut self) {
        self.height_offset = 0.0;
        self.height_scale = 1.0;
        let min = find_min(
            self,
            Vec2::new(-self.scale, self.scale),
            Vec2::new(-self.scale, self.scale),
        )
        .unwrap();
        let max = find_max(
            self,
            Vec2::new(-self.scale, self.scale),
            Vec2::new(-self.scale, self.scale),
        )
        .unwrap();
        let amplitude = self.z(max) - self.z(min);
        self.height_offset = -self.z(min);
        self.height_scale = 1.0 / amplitude;
    }

    pub fn z(&self, pos: Vec2) -> f32 {
        self.noise.get_noise(pos.x / self.scale, pos.y / self.scale) * self.height_scale
            + self.height_offset
    }
    pub fn grad(&self, pos: Vec2) -> Vec2 {
        let eps = 5.0;
        let dx = (self.z(Vec2::new(pos.x + eps, pos.y)) - self.z(Vec2::new(pos.x - eps, pos.y)));
        let dy = (self.z(Vec2::new(pos.x, pos.y + eps)) - self.z(Vec2::new(pos.x, pos.y - eps)));
        Vec2::new(dx, dy) / (2.0 * eps)
    }
}
