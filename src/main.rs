use bevy_flycam::prelude::*;
use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, VertexAttributeValues},
    render_asset::RenderAssetUsages,
    render_resource::PrimitiveTopology,
};
use bevy::math::f32::Vec3;

use bevy_cubes::quad::{Direction,new_quad,gen_indeces};

#[derive(Component)]
struct MyMesh;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
      //  .add_systems(Startup, test)
        .add_systems(Startup, spawn_cubes)
        //.add_systems(Startup, lsfr)
        .add_systems(Startup, setup)
        .run();
}

fn test(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
){
    let mut vec:Vec<[f32;3]> = Vec::new();
    let mut norm:Vec<Vec3> = Vec::new();
    for dir in Direction::iter() {
        for corner in new_quad(dir.clone(), Vec3::new(1.,1.,1.)){
            vec.push(corner);
        }
        let normal = match dir {
                Direction::North => Vec3::X,
                Direction::South => Vec3::NEG_X,
                Direction::East => Vec3::NEG_Z,
                Direction::West => Vec3::Z,
                Direction::Up => Vec3::Y,
                Direction::Down => Vec3::NEG_Y
            };
        norm.extend(vec![normal;4]);
    }
    let indeces = gen_indeces(vec.len());
    println!("{:?}",vec);
    println!();
    println!("{:?}",indeces);
    println!();
    println!("{:?}",norm);

    let cube_mesh_handle = meshes.add(
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD) 
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,vec) 
        .with_inserted_indices(indeces)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, norm));

    commands.spawn((
        PbrBundle {
            mesh: cube_mesh_handle,
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.2, 0.2),
                ..default()
            }),
            ..default()
        },
        MyMesh,
    ));
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
      // Create the line meshes
    let red_line_mesh = create_line_mesh(Vec3::ZERO, Vec3::X * 2.0);
    let green_line_mesh = create_line_mesh(Vec3::ZERO, Vec3::Y * 2.0);
    let blue_line_mesh = create_line_mesh(Vec3::ZERO, Vec3::Z * 2.0);

    // Insert the meshes into assets
    let red_line_handle = meshes.add(red_line_mesh);
    let green_line_handle = meshes.add(green_line_mesh);
    let blue_line_handle = meshes.add(blue_line_mesh);

    // Spawn the lines
    commands.spawn(PbrBundle {
        mesh: red_line_handle,
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(1., 0., 0.),
            ..default()
        }),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: green_line_handle,
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0., 1., 0.),
            ..default()
        }),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: blue_line_handle,
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0., 0., 1.),
            ..default()
        }),
        ..default()
    });

    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let light_transform =
        Transform::from_xyz(64.,64.,64.,).looking_at(Vec3::new(-1.,-1.,-1.), Vec3::Y);

    // Light up the scene.
    commands.spawn(PointLightBundle {
        transform: light_transform,
        point_light: PointLight {
            range: 1000.,
            intensity: 1_000_000_000.,
            ..default()
        },
        ..default()
    });
}

fn create_line_mesh(start: Vec3, end: Vec3) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList,RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![start ,end]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![end,end]);
    mesh.insert_indices(Indices::U32(vec![0,1]));
    mesh
}

fn spawn_cubes(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
){
    let cube_list: Vec<[f32;3]> = gen_cube_list(6000);
    let mut vertices:Vec<[f32;3]> = Vec::new();
    let mut norm:Vec<Vec3> = Vec::new();
    
    for cube_pos in cube_list {
        for dir in Direction::iter() {
            for corner in new_quad(dir.clone(), cube_pos.into()){
                vertices.push(corner);
            }
            let normal = match dir {
                    Direction::North => Vec3::X,
                    Direction::South => Vec3::NEG_X,
                    Direction::East => Vec3::NEG_Z,
                    Direction::West => Vec3::Z,
                    Direction::Up => Vec3::Y,
                    Direction::Down => Vec3::NEG_Y
                };
            norm.extend(vec![normal;4]);
        }
    }
    let indeces = gen_indeces(vertices.len());
    
    let mesh_handle = meshes.add(
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD) 
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices) 
        .with_inserted_indices(indeces)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, norm));

    commands.spawn((
        PbrBundle {
            mesh: mesh_handle,
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.2, 0.2),
                ..default()
            }),
            ..default()
        },
        MyMesh,
    ));
}

fn gen_cube_list(cube_count:usize) -> Vec<[f32;3]> {
    let mut rand_32 = lsfr();
    let mut cube_list: Vec<[f32;3]> = Vec::new();
    for _ in 0..cube_count {
        let x = rand_32() as f32;
        let y = rand_32() as f32;
        let z = rand_32() as f32;
        cube_list.push([x,y,z]);
    }
    cube_list
}

fn lsfr() -> impl FnMut() -> u32 {
    let mut state = 1 << 15 | 1;
    let mut step = move || {
        let bit = (state ^ (state >> 1) ^ ( state >> 3) ^ (state >> 12)) & 1;
        state = ( state >> 1) | ( bit << 15);
        bit
    };
    let rand_32 = move || {
        let mut num = 0u32;
        for _ in 0..5 {
            num = num << 1;
            num |= step();
        }
        num
    };
    rand_32
}

////fn create_mesh_wire() -> Mesh {
    ////
////}
