use bevy_flycam::prelude::*;
use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, VertexAttributeValues},
    render_asset::RenderAssetUsages,
    render_resource::PrimitiveTopology,
};
use bevy::math::f32::Vec3;

#[derive(Component)]
struct MyMesh;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .run();
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
    // Create and save a handle to the mesh.
    let cube_mesh_handle: Handle<Mesh> = meshes.add(create_mesh());

    // Render the mesh with the custom texture using a PbrBundle, add the marker.
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

    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let light_transform =
        Transform::from_xyz(2.8, 1.8, 2.8).looking_at(Vec3::NEG_X, Vec3::Y);

    // Light up the scene.
    commands.spawn(PointLightBundle {
        transform: light_transform,
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

fn create_mesh(cord: Vec3) -> Mesh {
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD) 
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION, 
            vec![
                //Clockwise winding
                //Down -y
                [0.,0.,0.],
                [1.,0.,0.],
                [1.,0.,1.],
                [0.,0.,1.],
                // Up +y
                [0.,1.,0.],
                [0.,1.,1.],
                [1.,1.,1.],
                [1.,1.,0.],
                // Front +x
                [1.,0.,0.],
                [1.,1.,0.],
                [1.,1.,1.],
                [1.,0.,1.],
                //// Back -x
                [0.,0.,0.],
                [0.,0.,1.],
                [0.,1.,1.],
                [0.,1.,0.],
                //// Left -z
                [1.,0.,0.],
                [0.,0.,0.],
                [0.,1.,0.],
                [1.,1.,0.],
                //// Right +z
                [1.,0.,1.],
                [1.,1.,1.],
                [0.,1.,1.],
                [0.,0.,1.],
                ])
        .with_inserted_indices(Indices::U32(vec![
                0,1,2 , 2,3,0,
                4,5,6 , 6,7,4,
                8,9,10 , 10,11,8,
                12,13,14, 14,15,12,
                16,17,18 , 18,19,16,
                20,21,22 , 22,23,20,
            ]))
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL, 
            vec![
                //Down -y
                Vec3::NEG_Y,
                Vec3::NEG_Y,
                Vec3::NEG_Y,
                Vec3::NEG_Y,
                // Up +y
                Vec3::Y,
                Vec3::Y,
                Vec3::Y,
                Vec3::Y,
                // Front +x
                Vec3::X,
                Vec3::X,
                Vec3::X,
                Vec3::X,
                // Back -x
                Vec3::NEG_X,
                Vec3::NEG_X,
                Vec3::NEG_X,
                Vec3::NEG_X,
                // Left -z
                Vec3::NEG_Z,
                Vec3::NEG_Z,
                Vec3::NEG_Z,
                Vec3::NEG_Z,
                // Right +z
                Vec3::Z,
                Vec3::Z,
                Vec3::Z,
                Vec3::Z,
            ])
}

fn gen_position() {
    for i in 0..2 {
        
    }
}

////fn create_mesh_wire() -> Mesh {
    ////
////}
