use crate::utils::*;
use crate::world::blocks::*;
use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

pub struct Wall {
    pos: Position,
    color: Color,
}

impl Block for Wall {
    fn event(&mut self, _event: &Event) {}
    fn update(&mut self) {}
    fn draw(&self, context: &Context, g2d: &mut G2d, viewpoint: &Viewpoint) {
        let vp = viewpoint.get_position();
        rectangle(
            self.color,
            [
                self.pos.x - vp.x,
                self.pos.y - vp.y,
                BLOCK_SIZE as f64,
                BLOCK_SIZE as f64,
            ],
            context.transform,
            g2d,
        )
    }
}

impl PhysicBody for Wall {
    fn collision(&mut self, _side: Option<&CollisionSide>) {}

    fn get_type(&self) -> BlockType {
        BlockType::Wall
    }

    fn get_position(&self) -> Position {
        self.pos
    }

    fn get_center(&self) -> Position {
        Position::new(
            self.pos.x + BLOCK_SIZE as f64 / 2.0,
            self.pos.y + BLOCK_SIZE as f64 / 2.0,
        )
    }

    fn get_corners(&self) -> [Position; 4] {
        return [
            self.pos,
            Position::new(self.pos.x + BLOCK_SIZE as f64, self.pos.y),
            Position::new(self.pos.x + BLOCK_SIZE as f64, self.pos.y),
            Position::new(self.pos.x + BLOCK_SIZE as f64, self.pos.y),
        ];
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.pos.x = x;
        self.pos.y = y;
    }

    fn set_velocity(&mut self, _new_speed: Vector) {}
    fn get_physic_body_type(&self) -> PhysicBodyType {
        PhysicBodyType::Static
    }
}

impl Wall {
    pub fn new(pos: Position) -> Wall {
        Wall {
            pos,
            color: [1.0, 0.0, 1.0, 1.0],
        }
    }
}
