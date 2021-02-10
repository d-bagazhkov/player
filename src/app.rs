
use piston::input::{Button, Key, RenderArgs, UpdateArgs};
use animation::*;
use std::collections::HashMap;

type PlayerAction = &'static str;
const WALK: PlayerAction = "walk";
const IDLE: PlayerAction = "idle";

pub struct Player {
    position: [f64; 2],
    speed: f64,
    current_action: PlayerAction,
    animations: HashMap<String, Animation>
}

impl Player {
    pub fn new() -> Self {
        let player_ani = Animation::load_from_json("./assets/data/skeleton.animations.json");
        Player {
            position: [0.0, 0.0],
            speed: 5.0,
            current_action: IDLE,
            animations: player_ani
        }
    }
    pub fn move_up(&mut self) {
        self.position[1] -= self.speed;
    }
    pub fn move_down(&mut self) {
        self.position[1] += self.speed;
    }
    pub fn move_right(&mut self) {
        self.position[0] += self.speed;
        self.animations.get_mut(self.current_action).unwrap().horizontal_orientation(Orientation::Normal);
    }
    pub fn move_left(&mut self) {
        self.position[0] -= self.speed;
        self.animations.get_mut(self.current_action).unwrap().horizontal_orientation(Orientation::Flipped);
    }
    pub fn change_action(&mut self, action: PlayerAction) {
        self.current_action = action;
    }
    pub fn render(&mut self, transform: graphics::math::Matrix2d, gl: &mut opengl_graphics::GlGraphics) {
        use graphics::*;
        let [x, y] = self.position;
        self.animations.get_mut(self.current_action).unwrap()
            .render(transform.trans(x, y), gl);
    }
}

pub struct App {
    player: Player
}

impl App {
    pub fn new() -> Self {
        App {
            player: Player::new()
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
        if let &Button::Keyboard(key) = args {
            match key {
                Key::W => {
                    self.player.move_up();
                }
                Key::S => {
                    self.player.move_down();
                }
                Key::A => {
                    self.player.move_left();
                }
                Key::D => {
                    self.player.move_right();
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