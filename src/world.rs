use crate::chunk::*;
use crate::quad::{new_quad, Direction};
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Resource, Default)]
pub struct VoxelWorld {
    pub chunks: HashMap<IVec3, Arc<Chunk>>,
    pub quads: u64,
}

pub struct VoxelWorldPlugin;
impl Plugin for VoxelWorldPlugin {
    fn build(&self, app: &mut App) {}
}

impl VoxelWorld {
    pub fn new() -> Self {
        VoxelWorld {
            chunks: HashMap::<IVec3, Arc<Chunk>>::new(),
            ..default()
        }
    }
    pub fn add_chunk(&mut self, pos: IVec3, chunk: Chunk) {
        self.chunks.insert(pos, chunk.into());
    }
    pub fn get_chunk(&self, pos: IVec3) -> Option<Arc<Chunk>> {
        match self.chunks.get(&pos) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }
    pub fn get_voxel_neighbours(&self, chunk_data: &ChunkData, voxel_pos: IVec3) -> Vec<Direction> {
        //North == +x
        let chunk_pos = chunk_data.pos;

        let mut directions: Vec<Direction> = Vec::new();
        if voxel_pos.x == 0 {
            if let Some(chunk) = self.get_chunk(chunk_pos - IVec3::X) {
                if !chunk.data.get(31, voxel_pos.y, voxel_pos.z) {
                    directions.push(Direction::South); 
                }
            }else{
                    directions.push(Direction::South); 
            }
        } else if voxel_pos.x == CHUNK_SIZE - 1 {
            if let Some(chunk) = self.get_chunk(chunk_pos + IVec3::X) {
                if !chunk.data.get(0, voxel_pos.y, voxel_pos.z) {
                    directions.push(Direction::North);
                }
            }else{
                    directions.push(Direction::North);
            }
        }
        if voxel_pos.x != 0 && !chunk_data.get(voxel_pos.x - 1, voxel_pos.y, voxel_pos.z) {
            directions.push(Direction::South)
        }
        if voxel_pos.x != CHUNK_SIZE - 1
            && !chunk_data.get(voxel_pos.x + 1, voxel_pos.y, voxel_pos.z)
        {
            directions.push(Direction::North)
        }

        if voxel_pos.y == 0 {
            if let Some(chunk) = self.get_chunk(chunk_pos - IVec3::Y) {
                if !chunk.data.get(voxel_pos.x, 31, voxel_pos.z) {
                    directions.push(Direction::Down) //Chage
                }
            }else{
                    directions.push(Direction::Down) //Chage
                }
        } else if voxel_pos.y == CHUNK_SIZE - 1 {
            if let Some(chunk) = self.get_chunk(chunk_pos + IVec3::Y) {
                if !chunk.data.get(voxel_pos.x, 0, voxel_pos.z) {
                    directions.push(Direction::Up) //Chage
                }
            }else{
                    directions.push(Direction::Up) //Chage
                }
        }
        if voxel_pos.y != 0 && !chunk_data.get(voxel_pos.x, voxel_pos.y - 1, voxel_pos.z) {
            directions.push(Direction::Down)
        }
        if voxel_pos.y != CHUNK_SIZE - 1
            && !chunk_data.get(voxel_pos.x, voxel_pos.y + 1, voxel_pos.z)
        {
            directions.push(Direction::Up)
        }

        if voxel_pos.z == 0 {
            if let Some(chunk) = self.get_chunk(chunk_pos - IVec3::Z) {
                if !chunk.data.get(voxel_pos.x, voxel_pos.y, 31) {
                    directions.push(Direction::East) //Chage
                }
            }else{
                    directions.push(Direction::East) //Chage
                }
        } else if voxel_pos.z == CHUNK_SIZE - 1 {
            if let Some(chunk) = self.get_chunk(chunk_pos + IVec3::Z) {
                if !chunk.data.get(voxel_pos.x, voxel_pos.y, 0) {
                    directions.push(Direction::West) //Chage
                }
            }else{
                    directions.push(Direction::West) //Chage
                }
        }
        if voxel_pos.z != 0 && !chunk_data.get(voxel_pos.x, voxel_pos.y, voxel_pos.z - 1) {
            directions.push(Direction::East)
        }
        if voxel_pos.z != CHUNK_SIZE - 1
            && !chunk_data.get(voxel_pos.x, voxel_pos.y, voxel_pos.z + 1)
        {
            directions.push(Direction::West)
        }

        return directions;
    }
}

pub fn get_voxel_neighbours2(chunk_data: &ChunkData, voxel_pos: IVec3) -> Vec<Direction> {
    todo!()
}
