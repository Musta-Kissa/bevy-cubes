use crate::chunk::*;
use crate::quad::Direction;
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Resource, Default)]
pub struct VoxelWorld {
    pub chunks: HashMap<IVec3, Arc<Chunk>>,
    pub quads: u64,
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

    //TODO Simplyfy (put middle chunkl in ChunkNeighbours and impl get_block(x,y,z) for ChunkNeighbours
    pub fn get_voxel_neighbours(
        &self,
        chunk_data: &ChunkData,
        neighbours: &ChunkNeighbours,
        voxel_pos: IVec3,
    ) -> Vec<Direction> {
        let mut directions: Vec<Direction> = Vec::new();
        if voxel_pos.x == 0 {
            if let Some(chunk) = neighbours.get(IVec3::NEG_X) {
                if !chunk.data.get(31, voxel_pos.y, voxel_pos.z) {
                    directions.push(Direction::South);
                }
            } else {
                directions.push(Direction::South);
            }
        } else if voxel_pos.x == CHUNK_SIZE - 1 {
            if let Some(chunk) = neighbours.get(IVec3::X) {
                if !chunk.data.get(0, voxel_pos.y, voxel_pos.z) {
                    directions.push(Direction::North);
                }
            } else {
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
            if let Some(chunk) = neighbours.get(IVec3::NEG_Y) {
                if !chunk.data.get(voxel_pos.x, 31, voxel_pos.z) {
                    directions.push(Direction::Down)
                }
            } else {
                //directions.push(Direction::Down) //Remove world floor
            }
        } else if voxel_pos.y == CHUNK_SIZE - 1 {
            if let Some(chunk) = neighbours.get(IVec3::Y) {
                if !chunk.data.get(voxel_pos.x, 0, voxel_pos.z) {
                    directions.push(Direction::Up)
                }
            } else {
                directions.push(Direction::Up)
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
            if let Some(chunk) = neighbours.get(IVec3::NEG_Z) {
                if !chunk.data.get(voxel_pos.x, voxel_pos.y, 31) {
                    directions.push(Direction::East)
                }
            } else {
                directions.push(Direction::East)
            }
        } else if voxel_pos.z == CHUNK_SIZE - 1 {
            if let Some(chunk) = neighbours.get(IVec3::Z) {
                if !chunk.data.get(voxel_pos.x, voxel_pos.y, 0) {
                    directions.push(Direction::West)
                }
            } else {
                directions.push(Direction::West)
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
