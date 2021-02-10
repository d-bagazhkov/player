extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate serde;
extern crate serde_json;

use crate::piston::EventLoop;

use piston::Window;
use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{PressEvent, ReleaseEvent, RenderEvent, UpdateEvent};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use app::*;

mod animation;
mod app;
mod player;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Player", [640, 480])
            .graphics_api(opengl)
            .exit_on_esc(true)
            // .fullscreen(true)
            .build()
            .unwrap();

    let mut gl = GlGraphics::new(opengl);
    
    let mut app = App::new(window.size().into());

    let mut events = Events::new(EventSettings::new());
    events.set_ups(60);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, &mut gl);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        if let Some(p) = e.press_args() {
            app.press(&p);
        }
        if let Some(b) = e.release_args() {
            app.release(&b);
        }
    }
    
}
