use crate::quad::{new_quad, Direction};
use crate::tools::ToUsize;
use crate::world::VoxelWorld;

use bevy::math::f32::Vec3;
use bevy::prelude::*;
use bevy::render::{
    mesh::Indices,
    render_asset::RenderAssetUsages,
    render_resource::{Face, PrimitiveTopology},
};
use bracket_noise::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub static QUAD_COUNT: AtomicUsize = AtomicUsize::new(0);

pub const CHUNK_SIZE: i32 = 32;
pub const SEED: u64 = 1111;

#[derive(Deref)]
pub struct ChunkData {
    #[deref]
    data: [[[bool; 32]; 32]; 32],
    pub pos: IVec3,
}

impl ChunkData {
    pub fn get<T>(&self, x: T, y: T, z: T) -> bool
    where
        T: ToUsize,
    {
        self.data[x.to_usize()][y.to_usize()][z.to_usize()]
    }
}

pub struct Chunk {
    pub position: IVec3,
    pub data: ChunkData,
}

#[derive(Deref)]
pub struct ChunkNeighbours {
    chunks: [Option<Arc<Chunk>>; 6],
}

impl ChunkNeighbours {
    pub fn new(voxel_world: &VoxelWorld, middle_chunk: IVec3) -> Self {
        let x = voxel_world.get_chunk(middle_chunk + IVec3::X);
        let neg_x = voxel_world.get_chunk(middle_chunk - IVec3::X);
        let y = voxel_world.get_chunk(middle_chunk + IVec3::Y);
        let neg_y = voxel_world.get_chunk(middle_chunk - IVec3::Y);
        let z = voxel_world.get_chunk(middle_chunk + IVec3::Z);
        let neg_z = voxel_world.get_chunk(middle_chunk - IVec3::Z);

        ChunkNeighbours {
            chunks: [x, neg_x, y, neg_y, z, neg_z],
        }
    }
    pub fn get(&self, pos:IVec3) -> &Option<Arc<Chunk>> {
        match pos {
           IVec3::X => &self[0],
           IVec3::NEG_X => &self[1],
           IVec3::Y => &self[2],
           IVec3::NEG_Y => &self[3],
           IVec3::Z => &self[4],
           IVec3::NEG_Z => &self[5],
            _ => &None
        }
    }
}

impl Chunk {
    pub fn gen_mesh(&self, world_data: &VoxelWorld) -> Mesh {
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut norm: Vec<Vec3> = Vec::new();
        let neighbours = ChunkNeighbours::new(world_data,self.position);

        for x in 0..32i32 {
            for y in 0..32i32 {
                for z in 0..32i32 {
                    if !self.data.get(x, y, z) {
                        continue;
                    }
                    for dir in world_data.get_voxel_neighbours(&self.data,&neighbours,IVec3::new(x, y, z)) {
                        vertices.extend(new_quad(
                            dir.clone(),
                            Vec3::new(
                                (self.position.x * CHUNK_SIZE + x) as f32,
                                (self.position.y * CHUNK_SIZE + y) as f32,
                                (self.position.z * CHUNK_SIZE + z) as f32,
                            ),
                        ));
                        // Quad num ++
                        QUAD_COUNT.fetch_add(1, Ordering::SeqCst);
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
}

pub fn gen_chunk(chunk_pos: IVec3) -> Chunk {
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

    Chunk {
        data: ChunkData {
            data: data,
            pos: chunk_pos,
        },
        position: chunk_pos,
    }
}
pub fn gen_chunk_flat(chunk_pos: IVec3) -> Chunk {
    let mut noise = FastNoise::new();
    noise.set_seed(SEED);
    noise.set_noise_type(NoiseType::Perlin);
    noise.set_frequency(6.);

    let mut data = [[[false; 32]; 32]; 32];

    for x in 0..32usize {
        for z in 0..32usize {
            let mut n = (noise.get_noise(
                ((chunk_pos.x * CHUNK_SIZE + x as i32) as f32) / 200.,
                ((chunk_pos.z * CHUNK_SIZE + z as i32) as f32) / 200.,
            ) + 1.)
                * 16.;

            n += (noise.get_noise(
                ((chunk_pos.x * CHUNK_SIZE + x as i32) as f32) / 1000.,
                ((chunk_pos.z * CHUNK_SIZE + z as i32) as f32) / 1000.,
            ) + 1.)
                * 16.
                * 4.;
            n -= 32.;

            for y in 0..32usize {
                //TODO Change this line
                if ((y as i32 + chunk_pos.y * 32) as f32) < n {
                    data[x][y][z] = true;
                } else {
                    data[x][y][z] = false;
                }
            }
        }
    }
    Chunk {
        data: ChunkData {
            data: data,
            pos: chunk_pos,
        },
        position: chunk_pos,
    }
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
