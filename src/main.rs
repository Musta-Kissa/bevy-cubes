use bevy::math::f32::Vec3;
use bevy::prelude::*;
use bevy::render::{
    mesh::Indices,
    render_asset::RenderAssetUsages,
    render_resource::{Face, PrimitiveTopology},
};
use bevy::window::{PresentMode, PrimaryWindow};
use bevy_cubes::chunk::*;
use bevy_flycam::prelude::*;
use bracket_noise::prelude::*;

#[derive(Component)]
struct MyMesh;

#[derive(Resource)]
struct FPS {
    buffer: [f32; 10],
    index: i64,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate, //NO V-Sync comment to turn on
                ..default()
            }),
            ..default()
        }))
        .insert_resource(FPS {
            buffer: [0.0; 10],
            index: 0,
        })
        .add_plugins(PlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00012, // default: 0.00012
            speed: 64.0,          // default: 12.0
        })
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_cubes)
        .add_systems(Startup, spawn_cardinal_lines)
        .add_systems(Update, fps)
        .run();
}

fn setup(
    mut commands: Commands,
    //mut materials: ResMut<Assets<StandardMaterial>>,
    //mut meshes: ResMut<Assets<Mesh>>,
) {
    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let light_transform = Transform::from_xyz(1024., 1024., 1024.).looking_at(Vec3::ZERO, Vec3::Y);

    // Light up the scene.
    commands.spawn(DirectionalLightBundle {
        transform: light_transform,
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 4000.,
            ..default()
        },
        ..default()
    });
}

fn create_line_mesh(start: Vec3, end: Vec3) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![start, end]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![end, end]);
    mesh.insert_indices(Indices::U32(vec![0, 1]));
    mesh
}

fn spawn_cubes(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for x in 0..4 {
        for y in 0..4 {
            for z in 0..4 {
                let chunk_data = gen_chunk_data(IVec3::new(x, y, z));
                let chunk = Chunk {
                    data: chunk_data,
                    position: IVec3::new(x, y, z),
                };
                let mesh = gen_mesh(&chunk);
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
    }
}

fn spawn_cardinal_lines(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let line_lenght = 32.0;
    // Create the line meshes
    let red_line_mesh = create_line_mesh(Vec3::ZERO, Vec3::X * line_lenght);
    let green_line_mesh = create_line_mesh(Vec3::ZERO, Vec3::Y * line_lenght);
    let blue_line_mesh = create_line_mesh(Vec3::ZERO, Vec3::Z * line_lenght);

    // Insert the meshes into assets
    let red_line_handle = meshes.add(red_line_mesh);
    let green_line_handle = meshes.add(green_line_mesh);
    let blue_line_handle = meshes.add(blue_line_mesh);

    // Spawn the lines
    commands.spawn(PbrBundle {
        mesh: red_line_handle,
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(2., 0., 0.),
            ..default()
        }),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: green_line_handle,
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0., 2., 0.),
            ..default()
        }),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: blue_line_handle,
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0., 0., 2.),
            ..default()
        }),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-2., -2., -2.),
        ..default()
    });
}

fn fps(
    time: Res<Time>,
    mut FPS: ResMut<FPS>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    FPS.index += 1;
    let i = FPS.index;
    let fps = 1. / time.delta_seconds();

    FPS.buffer[(i % 10) as usize] = fps;

    if i % 10 != 0 {
        return;
    }
    if let Ok(mut window) = window_query.get_single_mut() {
        let mut sum = 0.;
        for n in 0..10 {
            sum += FPS.buffer[n];
        }
        window.title = format!("FPS:{:0.2}", sum / 10.);
    }
}
