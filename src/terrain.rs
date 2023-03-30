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
    core_pipeline::core_2d::*,
    ecs::query::WorldQuery,
    math,
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::*, render_asset::*, render_phase::*, render_resource::*, texture::BevyDefault,
        view::*, *,
    },
    sprite::*,
    utils::FloatOrd,
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

        app.add_plugin(TerrainMesh2dPlugin);

        app.add_startup_system(mesh_system);
        // app.add_startup_systems((generation_system, isolines_system));
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

    let step = 200.0;
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
            // let color: [f32; 3] = rng.gen();
            let z = terrain.z(pos);
            colors.push([0.2 + z * 0.8, 0.2, 1.0 - z * 0.8, 1.0]);
        }
    }

    let n2 = n - 1;
    let m2 = m - 1;

    for j in (0..m2) {
        for i in (0..n2) {
            let a = n * j + i;
            let b = n * (j + 1) + i;
            indices.extend_from_slice(&[a, a + 1, b]);
            indices.extend_from_slice(&[a + 1, b + 1, b]);
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.set_indices(Some(Indices::U32(indices)));

    commands.spawn((
        TerrainMesh2d::default(),
        Mesh2dHandle(meshes.add(mesh)),
        SpatialBundle::INHERITED_IDENTITY,
    ));

    println!("OK");
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

#[derive(Component, Default)]
pub struct TerrainMesh2d;

#[derive(Resource)]
pub struct TerrainMesh2dPipeline {
    mesh2d_pipeline: Mesh2dPipeline,
}

impl FromWorld for TerrainMesh2dPipeline {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh2d_pipeline: Mesh2dPipeline::from_world(world),
        }
    }
}

impl SpecializedRenderPipeline for TerrainMesh2dPipeline {
    type Key = Mesh2dPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let formats = vec![VertexFormat::Float32x3, VertexFormat::Float32x4];

        let vertex_layout =
            VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, formats);

        let format = match key.contains(Mesh2dPipelineKey::HDR) {
            true => ViewTarget::TEXTURE_FORMAT_HDR,
            false => TextureFormat::bevy_default(),
        };

        RenderPipelineDescriptor {
            label: Some("terrain_mesh_pipeline".into()),
            layout: vec![
                self.mesh2d_pipeline.view_layout.clone(),
                self.mesh2d_pipeline.mesh_layout.clone(),
            ],
            push_constant_ranges: Vec::new(),
            vertex: VertexState {
                shader: COLORED_MESH2D_SHADER_HANDLE.typed::<Shader>(),
                entry_point: "vertex".into(),
                shader_defs: Vec::new(),
                buffers: vec![vertex_layout],
            },
            primitive: PrimitiveState {
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
                topology: key.primitive_topology(),
                strip_index_format: None,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: key.msaa_samples(),
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(FragmentState {
                shader: COLORED_MESH2D_SHADER_HANDLE.typed::<Shader>(),
                shader_defs: Vec::new(),
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
        }
    }
}

type DrawColoredMesh2d = (
    // Set the pipeline
    SetItemPipeline,
    // Set the view uniform as bind group 0
    SetMesh2dViewBindGroup<0>,
    // Set the mesh uniform as bind group 1
    SetMesh2dBindGroup<1>,
    // Draw the mesh
    DrawMesh2d,
);

const COLORED_MESH2D_SHADER: &str = r"
// Import the standard 2d mesh uniforms and set their bind groups
#import bevy_sprite::mesh2d_types
#import bevy_sprite::mesh2d_view_bindings

@group(1) @binding(0)
var<uniform> mesh: Mesh2d;

// NOTE: Bindings must come before functions that use them!
#import bevy_sprite::mesh2d_functions

// The structure of the vertex buffer is as specified in `specialize()`
struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    // The vertex shader must set the on-screen position of the vertex
    @builtin(position) clip_position: vec4<f32>,
    // We pass the vertex color to the fragment shader in location 0
    @location(0) color: vec4<f32>,
};

/// Entry point for the vertex shader
@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    // Project the world position of the mesh into screen position
    out.clip_position = mesh2d_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    // Unpack the `u32` from the vertex buffer into the `vec4<f32>` used by the fragment shader
    // out.color = vec4<f32>((vec4<u32>(vertex.color) >> vec4<u32>(0u, 8u, 16u, 24u)) & vec4<u32>(255u)) / 255.0;
    out.color = vertex.color;
    return out;
}

// The input of the fragment shader must correspond to the output of the vertex shader for all `location`s
struct FragmentInput {
    // The color is interpolated between vertices by default
    @location(0) color: vec4<f32>,
};

/// Entry point for the fragment shader
@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    return in.color;
}
";

/// Handle to the custom shader with a unique random ID
pub const COLORED_MESH2D_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 13828845428412094821);

/// Plugin that renders [`ColoredMesh2d`]s
pub struct TerrainMesh2dPlugin;

impl Plugin for TerrainMesh2dPlugin {
    fn build(&self, app: &mut App) {
        // Load our custom shader
        let mut shaders = app.world.resource_mut::<Assets<Shader>>();
        shaders.set_untracked(
            COLORED_MESH2D_SHADER_HANDLE,
            Shader::from_wgsl(COLORED_MESH2D_SHADER),
        );

        // Register our custom draw function and pipeline, and add our render systems
        app.get_sub_app_mut(RenderApp)
            .unwrap()
            .add_render_command::<Transparent2d, DrawColoredMesh2d>()
            .init_resource::<TerrainMesh2dPipeline>()
            .init_resource::<SpecializedRenderPipelines<TerrainMesh2dPipeline>>()
            .add_system(extract_colored_mesh2d.in_schedule(ExtractSchedule))
            .add_system(queue_colored_mesh2d.in_set(RenderSet::Queue));
    }
}

/// Extract the [`ColoredMesh2d`] marker component into the render app
pub fn extract_colored_mesh2d(
    mut commands: Commands,
    mut previous_len: Local<usize>,
    // When extracting, you must use `Extract` to mark the `SystemParam`s
    // which should be taken from the main world.
    query: Extract<Query<(Entity, &ComputedVisibility), With<TerrainMesh2d>>>,
) {
    let mut values = Vec::with_capacity(*previous_len);
    for (entity, computed_visibility) in &query {
        if !computed_visibility.is_visible() {
            continue;
        }
        values.push((entity, TerrainMesh2d));
    }
    *previous_len = values.len();
    commands.insert_or_spawn_batch(values);
}

/// Queue the 2d meshes marked with [`ColoredMesh2d`] using our custom pipeline and draw function
#[allow(clippy::too_many_arguments)]
pub fn queue_colored_mesh2d(
    transparent_draw_functions: Res<DrawFunctions<Transparent2d>>,
    colored_mesh2d_pipeline: Res<TerrainMesh2dPipeline>,
    mut pipelines: ResMut<SpecializedRenderPipelines<TerrainMesh2dPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    msaa: Res<Msaa>,
    render_meshes: Res<RenderAssets<Mesh>>,
    colored_mesh2d: Query<(&Mesh2dHandle, &Mesh2dUniform), With<TerrainMesh2d>>,
    mut views: Query<(
        &VisibleEntities,
        &mut RenderPhase<Transparent2d>,
        &ExtractedView,
    )>,
) {
    if colored_mesh2d.is_empty() {
        return;
    }
    // Iterate each view (a camera is a view)
    for (visible_entities, mut transparent_phase, view) in &mut views {
        let draw_colored_mesh2d = transparent_draw_functions.read().id::<DrawColoredMesh2d>();

        let mesh_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples())
            | Mesh2dPipelineKey::from_hdr(view.hdr);

        // Queue all entities visible to that view
        for visible_entity in &visible_entities.entities {
            if let Ok((mesh2d_handle, mesh2d_uniform)) = colored_mesh2d.get(*visible_entity) {
                // Get our specialized pipeline
                let mut mesh2d_key = mesh_key;
                if let Some(mesh) = render_meshes.get(&mesh2d_handle.0) {
                    mesh2d_key |=
                        Mesh2dPipelineKey::from_primitive_topology(mesh.primitive_topology);
                }

                let pipeline_id =
                    pipelines.specialize(&pipeline_cache, &colored_mesh2d_pipeline, mesh2d_key);

                let mesh_z = mesh2d_uniform.transform.w_axis.z;
                transparent_phase.add(Transparent2d {
                    entity: *visible_entity,
                    draw_function: draw_colored_mesh2d,
                    pipeline: pipeline_id,
                    // The 2d render items are sorted according to their z value before rendering,
                    // in order to get correct transparency
                    sort_key: FloatOrd(mesh_z),
                    // This material is not batched
                    batch_range: None,
                });
            }
        }
    }
}

// -----------------------------------------------------------------------------

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
