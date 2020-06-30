pub mod blocks;
mod parser;

use blocks::*;
use parser::parse_map;
use std::cell::RefCell;
use std::rc::Rc;

pub struct World {
    blocks: Vec<Rc<RefCell<dyn Block>>>,
}

impl World {
    pub fn new() -> World {
        World { blocks: Vec::new() }
    }
    
    pub fn load_map(&mut self, name: &str) {
        parse_map(name, &mut self.blocks);
    }

    pub fn get_blocks(&self) -> &Vec<Rc<RefCell<dyn Block>>> {
        &self.blocks
    }
}
