extern crate piston_window;

mod collision;
mod utils;
mod viewpoint;
mod window;
mod world;

use collision::Collision;
use piston_window::*;
use viewpoint::Viewpoint;
use world::blocks::BlockType;

pub const MAX_FPS: u64 = 60;

struct Game {
    world: world::World,
    viewpoint: Viewpoint,
}

impl Game {
    pub fn new() -> Game {
        let mut world = world::World::new();
        world.load_map("map_1");

        let player = world
            .get_blocks()
            .iter()
            .find(|x| x.borrow().get_type() == BlockType::Player)
            .unwrap();

        let viewpoint = Viewpoint::new(player.clone());
        let game = Game { world, viewpoint };

        game
    }

    pub fn event(&self, event: &Event) {
        for block in self.world.get_blocks() {
            block.borrow_mut().event(event);
        }
    }

    pub fn update(&self) {
        for block in self.world.get_blocks() {
            block.borrow_mut().update();
        }

        if let Some(player) = self
            .world
            .get_blocks()
            .iter()
            .find(|&x| x.borrow().get_type() == world::blocks::BlockType::Player)
        {
            let mut is_collision = false;
            for b1 in self
                .world
                .get_blocks()
                .iter()
                .filter(|&x| x.borrow().get_type() != world::blocks::BlockType::Player)
            {
                if Collision::apply_collision(player, b1, true, false) {
                    is_collision = true;
                }
            }

            for b1 in self
                .world
                .get_blocks()
                .iter()
                .filter(|&x| x.borrow().get_type() != world::blocks::BlockType::Player)
            {
                if Collision::apply_collision(player, b1, false, true) {
                    is_collision = true;
                }
            }

            if is_collision == false {
                player.borrow_mut().collision(None);
            }
        }
    }

    pub fn render(&mut self, event: &Event, window: &mut PistonWindow) {
        window.draw_2d(event, |_c, g, _device| {
            clear([0.0, 0.0, 0.0, 0.5], g);
        });
        for block in self.world.get_blocks() {
            window.draw_2d(event, |c, g, _device| {
                block.borrow().draw(&c, g, &self.viewpoint);
            });
        }
    }
}

fn main() {
    let mut game = Game::new();
    let mut window = window::Window::new("Platformer");

    window.get().set_max_fps(MAX_FPS);

    while let Some(event) = window.get().next() {
        game.event(&event);
        event.render(|_arg| {
            game.update();
            game.render(&event, window.get());
        });
    }
}
