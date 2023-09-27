#![allow(dead_code)]

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::*;

pub struct Displayer {
    window: Window,
    opengl: OpenGL,
    gl: GlGraphics
}

impl Displayer {
    pub fn new() -> Displayer {
        Displayer {
            opengl : OpenGL::V3_2,
            window : WindowSettings::new("Bubulle", [200,200])
                .graphics_api(OpenGL::V3_2)
                .exit_on_esc(true)
                .build()
                .unwrap(),
            gl: GlGraphics::new(OpenGL::V3_2)
        }
    }

    pub fn render_window(&mut self) {
    
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }
    
            if let Some(args) = e.update_args() {
                self.update(&args);
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let mut cell = Ellipse::new(RED);
        let rotation : f64 = 0.0;

        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            ellipse(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        
    }

}