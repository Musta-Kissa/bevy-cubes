use std::collections::HashMap;
use std::sync::Arc;
use bevy::prelude::*;
use crate::chunk::*;
use crate::quad::{new_quad, Direction};

#[derive(Deref,DerefMut,Resource)]
pub struct VoxelWorld {
    pub chunks: HashMap<IVec3,Arc<Chunk>>
}

impl VoxelWorld {
    pub fn new() -> Self {
        VoxelWorld { chunks: HashMap::<IVec3,Arc<Chunk>>::new() }
    }
    fn add_chunk(&mut self, pos:IVec3,chunk:Chunk){
        self.chunks.insert(pos,chunk.into());
    }
    fn get_chunk(&self,pos:IVec3) -> Option<Arc<Chunk>> {
        match self.chunks.get(&pos) {
            Some(c) => Some(c.clone()),
            None => None
        }
    }
}

pub fn get_voxel_neighbours(chunk_data: &ChunkData, voxel_pos: IVec3) -> Vec<Direction> {
    let mut directions: Vec<Direction> = Vec::new();
    if voxel_pos.x == 0 {
        directions.push(Direction::South)
    } else if voxel_pos.x == CHUNK_SIZE - 1 {
        directions.push(Direction::North)
    } 
    if voxel_pos.x != 0 && !chunk_data.get(voxel_pos.x - 1,voxel_pos.y,voxel_pos.z) {
        directions.push(Direction::South)
    }
    if voxel_pos.x != CHUNK_SIZE-1 && !chunk_data.get(voxel_pos.x + 1,voxel_pos.y,voxel_pos.z) {
        directions.push(Direction::North)
    }
     
    if voxel_pos.y == 0 {
        directions.push(Direction::Down)
    } else if voxel_pos.y == CHUNK_SIZE - 1 {
        directions.push(Direction::Up)
    } 
    if voxel_pos.y != 0 && !chunk_data.get(voxel_pos.x,voxel_pos.y - 1,voxel_pos.z) {
        directions.push(Direction::Down)
    }
    if voxel_pos.y != CHUNK_SIZE-1 && !chunk_data.get(voxel_pos.x,voxel_pos.y + 1,voxel_pos.z) {
        directions.push(Direction::Up)
    }

    if voxel_pos.z == 0 {
        directions.push(Direction::East)
    } else if voxel_pos.z == CHUNK_SIZE - 1 {
        directions.push(Direction::West)
    } 
    if voxel_pos.z != 0 && !chunk_data.get(voxel_pos.x,voxel_pos.y,voxel_pos.z - 1) {
        directions.push(Direction::East)
    }
    if voxel_pos.z != CHUNK_SIZE-1 && !chunk_data.get(voxel_pos.x,voxel_pos.y,voxel_pos.z + 1) {
        directions.push(Direction::West)
    }

    return directions;
}
