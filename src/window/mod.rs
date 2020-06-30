use piston_window::*;

pub const WINDOW_WIDTH: u32 = 500;
pub const WINDOW_HEIGHT: u32 = 500;

pub struct Window {
    window: PistonWindow,
}

impl Window {
    pub fn new(title: &str) -> Window {
        let window = WindowSettings::new(title, [WINDOW_WIDTH, WINDOW_HEIGHT])
            .exit_on_esc(true)
            .build();

        match window {
            Ok(win) => {
                Window { window: win }
            },
            Err(err) => {
                panic!(err.to_string())
            }
        }
    }

    pub fn get(&mut self) -> &mut PistonWindow {
        &mut self.window
    }
}
