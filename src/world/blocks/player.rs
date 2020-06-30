use crate::utils::{Position, Vector};
use crate::world::blocks::*;
use crate::MAX_FPS;
use piston_window::types::Color;
use piston_window::*;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash)]
enum PlayerEvent {
    Left,
    Right,
    Up,
}

pub struct Player {
    pos: Position,
    color: Color,

    // Events
    fixed_events: HashSet<PlayerEvent>,

    // Physics
    velocity: Vector,
    force: f64,
    friction: f64,
    gravity: f64,
    max_speed: f64,
    jump_force: f64,

    // Others
    is_jumped: bool,
    is_double_jumped: bool,
    is_jumped_in_air: bool,
    can_double_jump: bool,
    can_jump: bool,
    can_jump_in_air: bool,

    is_in_air: bool,
    is_on_the_ground: bool,
}

impl Block for Player {
    fn event(&mut self, event: &Event) {
        if let Event::Input(input, _) = event {
            if let Input::Button(button_args) = input {
                match button_args.button {
                    Button::Keyboard(Key::Left) => match button_args.state {
                        ButtonState::Press => {
                            self.fixed_events.insert(PlayerEvent::Left);
                        }
                        ButtonState::Release => {
                            self.fixed_events.remove(&PlayerEvent::Left);
                        }
                    },
                    Button::Keyboard(Key::Right) => match button_args.state {
                        ButtonState::Press => {
                            self.fixed_events.insert(PlayerEvent::Right);
                        }
                        ButtonState::Release => {
                            self.fixed_events.remove(&PlayerEvent::Right);
                        }
                    },
                    Button::Keyboard(Key::Up) => match button_args.state {
                        ButtonState::Press => {
                            self.fixed_events.insert(PlayerEvent::Up);
                        }
                        ButtonState::Release => {
                            self.fixed_events.remove(&PlayerEvent::Up);
                        }
                    },
                    _ => (),
                };
            }
        };
    }

    fn update(&mut self) {
        if self.fixed_events.contains(&PlayerEvent::Left) {
            self.velocity.x -= self.force;

            if self.velocity.x < -self.max_speed {
                self.velocity.x = -self.max_speed;
            }
        }

        if self.fixed_events.contains(&PlayerEvent::Right) {
            self.velocity.x += self.force;

            if self.velocity.x > self.max_speed {
                self.velocity.x = self.max_speed;
            }
        }

        if self.fixed_events.contains(&PlayerEvent::Left) == false
            && self.fixed_events.contains(&PlayerEvent::Right) == false
        {
            self.velocity.x *= self.friction;
        }

        if self.fixed_events.contains(&PlayerEvent::Up) {
            if self.is_in_air
                && self.can_jump_in_air
                && self.is_jumped == false
                && self.is_double_jumped == false
            {
                self.velocity.y = -self.jump_force;
                self.can_jump_in_air = false;
                self.is_jumped_in_air = true;
            } else if self.can_jump && self.is_jumped == false && self.is_in_air == false {
                self.velocity.y = -self.jump_force;
                self.is_jumped = true;
            } else if self.is_jumped && self.is_double_jumped == false && self.can_double_jump {
                self.velocity.y = -self.jump_force;
                self.can_double_jump = false;
                self.is_double_jumped = true;
            }
        } else if self.is_jumped {
            self.can_double_jump = true;
        }

        if self.is_on_the_ground == false {
            self.velocity.y += self.gravity;
        }

        self.pos += self.velocity;
    }

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

impl PhysicBody for Player {
    fn collision(&mut self, side: Option<&CollisionSide>) {
        match side {
            Some(CollisionSide::Left) => {
                if self.velocity.x < 0.0 {
                    self.velocity.x = 0.0;
                }
                self.is_on_the_ground = false;
            }
            Some(CollisionSide::Right) => {
                if self.velocity.x > 0.0 {
                    self.velocity.x = 0.0;
                }
                self.is_on_the_ground = false;
            }
            Some(CollisionSide::Up) => {
                if self.velocity.y < 0.0 {
                    self.velocity.y = 0.0;
                }
                self.is_on_the_ground = false;
            }
            Some(CollisionSide::Down) => {
                if self.velocity.y > 0.0 {
                    self.velocity.y = 0.0;
                }

                self.can_jump = true;
                self.can_double_jump = false;
                self.can_jump_in_air = false;

                self.is_jumped = false;
                self.is_double_jumped = false;
                self.is_jumped_in_air = false;
                self.is_in_air = false;
                self.is_on_the_ground = true;
            }
            None => {
                self.is_in_air = true;
                self.is_on_the_ground = false;

                if self.is_jumped_in_air == false {
                    self.can_jump_in_air = true;
                }
            }
        }
    }

    fn get_type(&self) -> BlockType {
        BlockType::Player
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
            Position::new(self.pos.x, self.pos.y + BLOCK_SIZE as f64),
            Position::new(
                self.pos.x + BLOCK_SIZE as f64,
                self.pos.y + BLOCK_SIZE as f64,
            ),
        ];
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.pos.x = x;
        self.pos.y = y;
    }

    fn set_velocity(&mut self, _new_speed: Vector) {}
    fn get_physic_body_type(&self) -> PhysicBodyType {
        PhysicBodyType::RightBody
    }
}

impl Player {
    pub fn new(pos: Position) -> Player {
        Player {
            pos,
            color: [1.0, 1.0, 1.0, 1.0],
            fixed_events: HashSet::new(),
            velocity: Vector::new(0.0, 0.0),
            force: 30.0 / MAX_FPS as f64,
            friction: 0.95,
            gravity: 10.0 / MAX_FPS as f64,
            max_speed: 200.0 / MAX_FPS as f64,
            jump_force: 300.0 / MAX_FPS as f64,
            is_jumped: false,
            is_double_jumped: false,
            is_jumped_in_air: false,
            can_double_jump: false,
            can_jump: true,
            can_jump_in_air: false,
            is_in_air: true,
            is_on_the_ground: false
        }
    }
}
