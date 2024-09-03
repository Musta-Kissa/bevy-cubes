use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, VertexAttributeValues},
    render_asset::RenderAssetUsages,
    render_resource::PrimitiveTopology,
};
use bevy::math::f32::Vec3;
use std::slice::Iter;

#[derive(Clone,Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down
}

impl Direction {
    pub fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction;6] = [Direction::North, 
                                            Direction::South, 
                                            Direction::East, 
                                            Direction::West, 
                                            Direction::Up, 
                                            Direction::Down];
        DIRECTIONS.iter()
    }
}

pub fn new_quad(dir: Direction, pos: Vec3) -> [[f32;3];4] {
    //Down -y
    match dir {
        Direction::North => 
            [
            [pos.x+1.,pos.y+0.,pos.z+0.],
            [pos.x+1.,pos.y+1.,pos.z+0.],
            [pos.x+1.,pos.y+1.,pos.z+1.],
            [pos.x+1.,pos.y+0.,pos.z+1.],
            ],
        Direction::South => 
            [
            [pos.x+0.,pos.y+0.,pos.z+0.],
            [pos.x+0.,pos.y+0.,pos.z+1.],
            [pos.x+0.,pos.y+1.,pos.z+1.],
            [pos.x+0.,pos.y+1.,pos.z+0.],
            ],
        Direction::East => 
            [
            [pos.x+1.,pos.y+0.,pos.z+0.],
            [pos.x+0.,pos.y+0.,pos.z+0.],
            [pos.x+0.,pos.y+1.,pos.z+0.],
            [pos.x+1.,pos.y+1.,pos.z+0.],
            ],
        Direction::West => 
            [
            [pos.x+1.,pos.y+0.,pos.z+1.],
            [pos.x+1.,pos.y+1.,pos.z+1.],
            [pos.x+0.,pos.y+1.,pos.z+1.],
            [pos.x+0.,pos.y+0.,pos.z+1.],
            ],
        Direction::Up => 
            [
            [pos.x+0.,pos.y+1.,pos.z+0.],
            [pos.x+0.,pos.y+1.,pos.z+1.],
            [pos.x+1.,pos.y+1.,pos.z+1.],
            [pos.x+1.,pos.y+1.,pos.z+0.],
            ],
        Direction::Down => 
            [
            [pos.x+0.,pos.y+0.,pos.z+0.],
            [pos.x+1.,pos.y+0.,pos.z+0.],
            [pos.x+1.,pos.y+0.,pos.z+1.],
            [pos.x+0.,pos.y+0.,pos.z+1.],
            ],
    }
}

pub fn gen_indeces(vert_len:usize) -> Indices {
        let mut indices: Vec<u32> = Vec::new();
        indices.reserve_exact(vert_len);
        for i in 0..(vert_len as u32)/4 {
            indices.extend([0+4*i,1+4*i,2+4*i , 2+4*i,3+4*i,0+4*i]); 
        }
        Indices::U32(indices)
}
