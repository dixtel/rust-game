use crate::utils::Position;
use crate::world::blocks::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn parse_map(map: &str, blocks: &mut Vec<Rc<RefCell<dyn Block>>>) {
    let file = std::fs::read_to_string(format!("maps/{}.txt", map));
    match file {
        Ok(file) => {
            let mut player_find_flag = false;
            for (y, elem) in file.split("\n").into_iter().enumerate() {
                let raw_blocks: Vec<BlockType> = elem
                    .split(" ")
                    .map(|x| x.to_string().parse().unwrap())
                    .collect();

                for (x, block) in raw_blocks.iter().enumerate() {
                    match block {
                        BlockType::Player => {
                            player_find_flag = match player_find_flag {
                                true => panic!("Player initialized twice"),
                                false => true,
                            };

                            blocks.push(Rc::new(RefCell::new(player::Player::new(Position::new(
                                (x * BLOCK_SIZE) as f64,
                                (y * BLOCK_SIZE) as f64,
                            )))));
                        }
                        BlockType::Wall => {
                            blocks.push(Rc::new(RefCell::new(wall::Wall::new(Position::new(
                                (x * BLOCK_SIZE) as f64,
                                (y * BLOCK_SIZE) as f64,
                            )))));
                        }
                        BlockType::None => {}
                    }
                }
            }
        }
        Err(err) => panic!(err),
    }
}
