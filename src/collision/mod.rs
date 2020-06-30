use crate::utils::*;
use crate::world::blocks::*;
use std::{cell::RefCell, rc::Rc};

pub enum CollisionSide {
    Left,
    Right,
    Up,
    Down,
}

pub struct Collision;

impl Collision {
    /// Check collision side for a
    fn check_collision_side(
        a: &Rc<RefCell<dyn Block>>,
        b: &Rc<RefCell<dyn Block>>,
        x_collision_first: &bool,
        y_collision_first: &bool,
    ) -> Option<CollisionSide> {
        if a.borrow()
            .get_corners()
            .iter()
            .any(|x| math::in_square(x, &b.borrow().get_position(), BLOCK_SIZE))
        {
            let vec: Vector = (b.borrow().get_position() - a.borrow().get_position()).to_vector();

            if vec.x * vec.x > vec.y * vec.y { 
                if *x_collision_first {
                    if vec.x > 0.0 {
                        return Some(CollisionSide::Right);
                    } else {
                        return Some(CollisionSide::Left);
                    }   
                }
            } else {
                if *y_collision_first {
                    if vec.y > 0.0 {
                        return Some(CollisionSide::Down);
                    } else {
                        return Some(CollisionSide::Up);
                    }
                }
            }
        }
        return None;
    }

    pub fn apply_collision(
        a: &Rc<RefCell<dyn Block>>,
        b: &Rc<RefCell<dyn Block>>,
        x_collision_first: bool,
        y_collision_first: bool,
    ) -> bool {
        let a_pos = { a.borrow().get_position() };

        let side = Collision::check_collision_side(&a, &b, &x_collision_first, &y_collision_first);
        let mut a_mut = a.borrow_mut();

        match &side {
            Some(CollisionSide::Left) => {
                a_mut.set_position(b.borrow().get_position().x + BLOCK_SIZE as f64, a_pos.y);
                a_mut.collision(Some(&CollisionSide::Left));
            }
            Some(CollisionSide::Right) => {
                a_mut.set_position(b.borrow().get_position().x - BLOCK_SIZE as f64, a_pos.y);
                a_mut.collision(Some(&CollisionSide::Right));
            }
            Some(CollisionSide::Up) => {
                a_mut.set_position(a_pos.x, b.borrow().get_position().y + BLOCK_SIZE as f64);
                a_mut.collision(Some(&CollisionSide::Up));
            }
            Some(CollisionSide::Down) => {
                a_mut.set_position(a_pos.x, b.borrow().get_position().y - BLOCK_SIZE as f64);
                a_mut.collision(Some(&CollisionSide::Down));
            }
            None => return false,
        };

        true
    }
}
