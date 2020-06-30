pub mod player;
pub mod wall;

use crate::collision::CollisionSide;
use crate::utils::{Position, Vector};
use crate::viewpoint::Viewpoint;
use piston_window::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BlockType {
    None,
    Player,
    Wall,
}

impl std::str::FromStr for BlockType {
    type Err = String;
    fn from_str(s: &str) -> Result<BlockType, Self::Err> {
        let block = s.to_string().parse::<i32>().unwrap();
        if block == BlockType::None as i32 {
            return Ok(BlockType::None);
        } else if block == BlockType::Player as i32 {
            return Ok(BlockType::Player);
        } else if block == BlockType::Wall as i32 {
            return Ok(BlockType::Wall);
        }
        Err(format!("Cannot convert {} to BlockType", s))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PhysicBodyType {
    Static,
    RightBody,
}

pub const BLOCK_SIZE: usize = 32;

pub trait Block: PhysicBody {
    fn event(&mut self, event: &Event);
    fn update(&mut self);
    fn draw(&self, context: &Context, g2d: &mut G2d, viewpoint: &Viewpoint);
}

pub trait PhysicBody {
    fn collision(&mut self, side: Option<&CollisionSide>);

    fn get_type(&self) -> BlockType;
    fn get_position(&self) -> Position;
    fn get_center(&self) -> Position;
    fn get_corners(&self) -> [Position; 4];
    fn get_physic_body_type(&self) -> PhysicBodyType;

    fn set_position(&mut self, x: f64, y: f64);
    fn set_velocity(&mut self, new_speed: Vector);
}
