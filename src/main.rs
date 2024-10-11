use std::sync::atomic::Ordering;

use bevy::gizmos::aabb;
use bevy::math::f32::Vec3;
use bevy::prelude::*;
use bevy::reflect::List;
use bevy::render::primitives::Aabb;
use bevy::render::view::calculate_bounds;
use bevy::render::{
    mesh::Indices,
    render_asset::RenderAssetUsages,
    render_resource::{Face, PrimitiveTopology},
};
use bevy::transform::commands;
use bevy::window::PresentMode;
use bevy_flycam::prelude::*;
// Local imports
use bevy_cubes::chunk::*;
use bevy_cubes::fps::FpsPlugin;
use bevy_cubes::world::VoxelWorld;

#[derive(Component)]
struct MyMesh;

fn main() {
    App::new()
        .add_plugins(FpsPlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync, //NO V-Sync comment to turn on
                ..default()
            }),
            ..default()
        }))
        .add_plugins(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00012, // default: 0.00012
            //speed: 128.0,          // default: 12.0
            speed: 64.0, // default: 12.0
        })
        .insert_resource(VoxelWorld::new())
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_cubes)
        .add_systems(Startup, make_hitbox_mesh)
        .add_systems(PostStartup, print_debug)
        .add_systems(PostUpdate, update_hitbox)
        .run();
}

#[derive(Component)]
struct Hitbox;

fn make_hitbox_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let width = 8.0 / 2.;
    let hight = 24.0;
    for x in vec![-width, width] {
        for y in vec![0., hight] {
            for z in vec![-width, width] {
                println!("x:{} y:{} z:{}", x, y, z);
                vertices.push([x, y, z]);
            }
        }
    }
    //0 x:-1 y:-1 z:-1
    //1 x:-1 y:-1 z:1
    //2 x:-1 y:1 z:-1
    //3 x:-1 y:1 z:1
    //4 x:1 y:-1 z:-1
    //5 x:1 y:-1 z:1
    //6 x:1 y:1 z:-1
    //7 x:1 y:1 z:1

    let indeces = vec![
        0, 1, 1, 3, 3, 2, 2, 0, 4, 5, 5, 7, 7, 6, 6, 4, 0, 4, 2, 6, 3, 7, 1, 5,
    ];
    let mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_indices(Indices::U32(indeces));
    let hitbox = commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 1.0, 1.0),
                cull_mode: Some(Face::Back),
                perceptual_roughness: 0.745,
                unlit: true,
                ..default()
            }),
            ..default()
        },
        Hitbox,
    )).id();

    let head_hight = 20.0;
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    for x in vec![-width, width] {
        for z in vec![-width, width] {
            vertices.push([x, head_hight, z]);
        }
    }
    let indeces = vec![0, 1, 1, 3, 3, 2, 2, 0];
    let mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_indices(Indices::U32(indeces));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.0, 0.0),
                cull_mode: Some(Face::Back),
                perceptual_roughness: 0.745,
                unlit: true,
                ..default()
            }),
            ..default()
        },
    )).set_parent(hitbox);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let light_transform = Transform::from_xyz(1024., 1024., 1024.).looking_at(Vec3::ZERO, Vec3::Y);

    // Light up the scene.
    commands.spawn(DirectionalLightBundle {
        transform: light_transform,
        directional_light: DirectionalLight {
            shadows_enabled: false,
            illuminance: 4000.,
            ..default()
        },
        ..default()
    });
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(32., 32. * 3., 0.)
                .looking_at(Vec3::new(32. * 8., 32., 32. * 8.), Vec3::Y),
            ..default()
        },
        FlyCam,
    ));
}
fn update_hitbox(
    camera_query: Query<&Transform, With<Camera3d>>,
    mut cube_query: Query<&mut Transform, (With<Hitbox>, Without<Camera3d>)>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        if let Ok(mut cube_transform) = cube_query.get_single_mut() {
            let (mut yaw, _, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);
            cube_transform.translation =
                camera_transform.translation + Vec3::new(0.,-20.,0.);
                //camera_transform.translation + camera_transform.forward().with_y(0.) * 64.;
                //camera_transform.translation + camera_transform.forward() * 64.;
            //cube_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw);
        }
    }
}

fn spawn_cubes(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut voxel_world: ResMut<VoxelWorld>,
) {
    for x in 0..16 {
        for y in 0..3 {
            for z in 0..16 {
                let pos = IVec3::new(x, y, z);
                let chunk = gen_chunk_flat(pos);
                voxel_world.add_chunk(pos, chunk.into());
            }
        }
    }
    for (_, chunk) in &voxel_world.chunks {
        let mesh = chunk.gen_mesh(voxel_world.as_ref());
        let mesh_handle = meshes.add(mesh);

        commands.spawn((
            PbrBundle {
                mesh: mesh_handle,
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(0.8, 0.2, 0.2),
                    cull_mode: Some(Face::Back),
                    perceptual_roughness: 0.745,
                    ..default()
                }),
                ..default()
            },
            MyMesh,
        ));
    }
}

#[derive(Component)]
struct CardinalLine;
#[derive(Bundle)]
struct CardinalBundle {
    redline: PbrBundle,
    greenline: PbrBundle,
    blueline: PbrBundle,
    marker: CardinalLine,
}

fn print_debug() {
    println!("NUMBER OF QUADS:{}", QUAD_COUNT.load(Ordering::SeqCst));
}
