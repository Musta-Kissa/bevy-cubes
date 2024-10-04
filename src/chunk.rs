use bevy::math::f32::Vec3;
use bevy::prelude::*;
use bevy::render::{
    mesh::Indices,
    render_asset::RenderAssetUsages,
    render_resource::{Face, PrimitiveTopology},
};
use bracket_noise::prelude::*;

use crate::quad::{new_quad, Direction};

const CHUNK_SIZE: i32 = 32;
const SEED: u64 = 1111;

#[derive(Deref)]
pub struct ChunkData {
    data: [[[bool; 32]; 32]; 32],
}
pub struct Chunk {
    pub data: ChunkData,
    pub position: IVec3,
}

#[derive(Component)]
struct IsChunk;

fn get_neighbours(chunk_data: &ChunkData, voxel_pos: IVec3) -> Vec<Direction> {
    let mut directions: Vec<Direction> = Vec::new();
    if voxel_pos.x == 0 {
        directions.push(Direction::South)
    } else if voxel_pos.x == CHUNK_SIZE - 1 {
        directions.push(Direction::North)
    } 
    if voxel_pos.x != 0 && !chunk_data[(voxel_pos.x - 1) as usize][voxel_pos.y as usize][voxel_pos.z as usize] {
        directions.push(Direction::South)
    }
    if voxel_pos.x != CHUNK_SIZE-1 && !chunk_data[(voxel_pos.x + 1) as usize][voxel_pos.y as usize][voxel_pos.z as usize] {
        directions.push(Direction::North)
    }
     
    if voxel_pos.y == 0 {
        directions.push(Direction::Down)
    } else if voxel_pos.y == CHUNK_SIZE - 1 {
        directions.push(Direction::Up)
    } 
    if voxel_pos.y != 0 && !chunk_data[voxel_pos.x as usize][(voxel_pos.y - 1) as usize][voxel_pos.z as usize] {
        directions.push(Direction::Down)
    }
    if voxel_pos.y != CHUNK_SIZE-1 && !chunk_data[voxel_pos.x as usize][(voxel_pos.y + 1) as usize][voxel_pos.z as usize] {
        directions.push(Direction::Up)
    }

    if voxel_pos.z == 0 {
        directions.push(Direction::East)
    } else if voxel_pos.z == CHUNK_SIZE - 1 {
        directions.push(Direction::West)
    } 
    if voxel_pos.z != 0 && !chunk_data[voxel_pos.x as usize][voxel_pos.y as usize][(voxel_pos.z - 1) as usize] {
        directions.push(Direction::East)
    }
    if voxel_pos.z != CHUNK_SIZE-1 && !chunk_data[voxel_pos.x as usize][voxel_pos.y as usize][(voxel_pos.z + 1) as usize] {
        directions.push(Direction::West)
    }

    return directions;
}
pub fn gen_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut norm: Vec<Vec3> = Vec::new();

    for x in 0..32i32 {
        for y in 0..32i32 {
            for z in 0..32i32 {
                if !chunk.data[x as usize][y as usize][z as usize] {
                    continue;
                }
                for dir in get_neighbours(&chunk.data, IVec3::new(x, y, z)) {
                    vertices.extend(new_quad(dir.clone(), 
                                             Vec3::new(
                                                (chunk.position.x * CHUNK_SIZE + x) as f32,
                                                (chunk.position.y * CHUNK_SIZE + y) as f32,
                                                (chunk.position.z * CHUNK_SIZE + z) as f32)) );
                    let normal = match dir {
                        Direction::North => Vec3::X,
                        Direction::South => Vec3::NEG_X,
                        Direction::East => Vec3::NEG_Z,
                        Direction::West => Vec3::Z,
                        Direction::Up => Vec3::Y,
                        Direction::Down => Vec3::NEG_Y,
                    };
                    norm.extend(vec![normal; 4]);
                }
            }
        }
    }
    let indeces = gen_indeces(vertices.len());

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_indices(indeces)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, norm)
}

pub fn gen_chunk(chunk_pos: IVec3) -> ChunkData {
    let mut noise = FastNoise::new();
    noise.set_seed(SEED);
    noise.set_noise_type(NoiseType::Perlin);
    noise.set_frequency(6.);

    let mut data = [[[false; 32]; 32]; 32];

    for x in 0..32usize {
        for y in 0..32usize {
            for z in 0..32usize {
                let n = noise.get_noise3d(
                    ((chunk_pos.x * CHUNK_SIZE + x as i32) as f32) / 100.,
                    ((chunk_pos.y * CHUNK_SIZE + y as i32) as f32) / 100.,
                    ((chunk_pos.z * CHUNK_SIZE + z as i32) as f32) / 100.,
                );
                if n < 0. {
                    data[x][y][z] = false;
                } else {
                    data[x][y][z] = true;
                }
            }
        }
    }

    ChunkData { data: data }
}
pub fn gen_indeces(vert_len: usize) -> Indices {
    let mut indices: Vec<u32> = Vec::new();
    indices.reserve_exact(vert_len);
    //clockwise winding
    for i in 0..(vert_len as u32) / 4 {
        indices.extend([
            0 + 4 * i,
            1 + 4 * i,
            2 + 4 * i,
            2 + 4 * i,
            3 + 4 * i,
            0 + 4 * i,
        ]);
    }
    Indices::U32(indices)
}











