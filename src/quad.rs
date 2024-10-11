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

pub fn new_quad(dir: Direction, pos: Vec3) -> [[f32;3];4] {
    //Down -y
    match dir {
        // Each face is written to have clockwise winding
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
            //[pos.x+0.9,pos.y+0.1,pos.z+0.9],
            //[pos.x+0.9,pos.y+0.9,pos.z+0.9],
            //[pos.x+0.1,pos.y+0.9,pos.z+0.9],
            //[pos.x+0.1,pos.y+0.1,pos.z+0.9],
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
