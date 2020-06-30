use crate::utils::Position;
use crate::window::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::world::blocks::{Block, BLOCK_SIZE};
use std::{cell::RefCell, rc::Rc};

pub struct Viewpoint {
    viewpoint: Rc<RefCell<dyn Block>>,
    width: u32,
    height: u32,
}

impl Viewpoint {
    pub fn new(to_follow: Rc<RefCell<dyn Block>>) -> Viewpoint {
        Viewpoint {
            viewpoint: to_follow,
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
        }
    }

    pub fn get_position(&self) -> Position {
        let pos = self.viewpoint.borrow().get_position();
        return Position::new(
            (pos.x + (BLOCK_SIZE / 2) as f64) - (self.width / 2) as f64,
            (pos.y + (BLOCK_SIZE / 2) as f64) - (self.height / 2) as f64,
        );
    }
}
