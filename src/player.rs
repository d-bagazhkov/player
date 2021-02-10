use animation::*;
use std::collections::HashMap;

type PlayerAction = &'static str;
pub const WALK: PlayerAction = "walk";
pub const IDLE: PlayerAction = "idle";

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