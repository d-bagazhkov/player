
use piston::input::{Button, Key, RenderArgs, UpdateArgs};
use player::*;

pub struct App {
    player: Player,
    window_size: (f64, f64)
}

impl App {
    pub fn new(window_size: (f64, f64)) -> Self {
        App {
            player: Player::new(),
            window_size
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut opengl_graphics::GlGraphics) {
        use graphics::*;
        
        const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 1.0];
       
        gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            self.player.render(c.transform, gl);
        })
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        
    }

    pub fn press(&mut self, args: &Button) {
        let mut moved = true;
        let (window_width, window_height) = self.window_size;
        if let &Button::Keyboard(key) = args {
            match key {
                Key::W => {
                    self.player.move_up();
                }
                Key::S => {
                    self.player.move_down(window_height);
                }
                Key::A => {
                    self.player.move_left();
                }
                Key::D => {
                    self.player.move_right(window_width);
                }
                _ => {
                    moved = false;
                }
            }
        }
        if moved {
            self.player.change_action(WALK);
        }
    }

    pub fn release(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::W => {
                    self.player.change_action(IDLE);
                }
                Key::S => {
                    self.player.change_action(IDLE);
                }
                Key::A => {
                    self.player.change_action(IDLE);
                }
                Key::D => {
                    self.player.change_action(IDLE);
                }
                _ => {}
            }
        }
    }
}