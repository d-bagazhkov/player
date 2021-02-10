use serde::{Serialize, Deserialize};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::cmp::{PartialEq, Eq};

#[derive(Serialize, Deserialize)]
pub struct Animation {
    pub filepath_animation: String,
    pub width: f64,
    pub height: f64,
    pub framerate: Option<u32>,
    pub frames: Vec<(f64, f64)>,
    pub horizontal_orientation: Orientation,
    pub vertical_orientation: Orientation,
    pub x_scale: f64,
    pub y_scale: f64,
    
    #[serde(skip_serializing, skip_deserializing)]
    pub current_frame: usize,
    #[serde(skip_serializing, skip_deserializing)]
    pub texture: Option<Texture>,
    #[serde(skip_serializing, skip_deserializing)]
    pub rect: [f64; 4],
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Orientation {
    Normal,
    Flipped,
}

impl std::fmt::Debug for Animation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tex = match &self.texture {Some(_) => Some("Texture"), None => None};
        f.debug_struct("Animation")
         .field("width", &self.width)
         .field("height", &self.height)
         .field("filepath_animation", &self.filepath_animation)
         .field("texture", &tex)
         .field("framerate", &self.framerate)
         .field("rect", &self.rect)
         .field("current_frame", &self.current_frame)
         .field("frames", &self.frames)
         .field("horizontal_orientation", &self.horizontal_orientation)
         .field("vertical_orientation", &self.vertical_orientation)
         .field("x_scale", &self.x_scale)
         .field("y_scale", &self.y_scale)
         .finish()
    }
} 

impl Animation {

    pub fn load_from_json(filepath: &'static str) -> HashMap<String, Animation> {
        let contents: String = read_to_string(filepath)
            .expect(&format!("Something went wrong reading the animation config file {}", filepath));
        let mut json: HashMap<String, Animation> = serde_json::from_str(&contents)
            .expect(&format!("Something went wrong deserializing string {}", contents));
        for (_, animation) in json.iter_mut() {
            let texture = Texture::from_path(&animation.filepath_animation, &TextureSettings::new())
                .expect(&format!("Something went wrong reading the texture file {}", &animation.filepath_animation));
            animation.texture = Some(texture);
            animation.rect = [0.0, 0.0, animation.width, animation.height];
        }

        json
    }


    fn update_animation_frame(&mut self) -> (f64, f64) {
        let frames = &self.frames;
        let framerate = self.framerate.unwrap_or(1);
        let frame_out_of_bounds = self.current_frame >= (framerate - 1) as usize ;
        let frame = self.current_frame.clone();

        self.current_frame = if frame_out_of_bounds {
            0
        } else {
            self.current_frame + 1
        };

        let frame_number = frame / ( framerate as usize  / frames.len() );
        let (x, y) = *frames.get(frame_number).unwrap_or(&(0.0, 0.0));

        (x, y)
    }
    
    pub fn render(&mut self, transform: graphics::math::Matrix2d, gl: &mut GlGraphics) {
        use graphics::*;
        let (fx, fy) = &self.update_animation_frame();
        let src_rect = [*fx, *fy, self.width, self.height];
        // let width_offset = self.width * 2.0;
        // let height_offset = self.height * 2.0;
        let mut t = transform.scale(self.x_scale, self.y_scale);
        if self.vertical_orientation == Orientation::Flipped {
            t = t.flip_v().trans(0.0, -self.height);
        }
        if self.horizontal_orientation == Orientation::Flipped {
            t = t.flip_h().trans(-self.width, 0.0);
        }
        match &self.texture {
            Some(tex) => {  
                Image::new()
                    .rect(self.rect)
                    .src_rect(src_rect)
                    .draw(tex, &Default::default(), t, gl);
            },
            None => {}
        }
    }

    pub fn horizontal_orientation(&mut self, o: Orientation) {
        self.horizontal_orientation = o;
    }
    #[allow(dead_code)]
    pub fn vertical_orientation(&mut self, o: Orientation) {
        self.vertical_orientation = o;
    }
}
