extern crate sdl2;

use std::path::Path;
use std::vec::Vec;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::image::LoadTexture;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use pipes::Pipe;
use display::Displayable;
use particles::Particles;

const GRAVITY: f64 = 0.2;
const JUMPSPEED: f64 = 8.0;

pub struct Bird {
    particles: Particles,
    time: i32,
    pub x: i32,
    pub y: i32,
    w: i32,
    h: i32,
    speed: f64,
    dead: bool,

    _texture_names: Vec<String>,
    textures: Vec<Texture>,
}

impl Bird {
    pub fn new(renderer: &Renderer) -> Bird {
        // Must keep the names around as owned strings.
        let mut frame_names: Vec<String> = Vec::new();
        let mut frame_textures: Vec<Texture> = Vec::new();

        for n in 1..4 {
            frame_names.push(format!("res/imgs/bird_frame_{}.png", n));
            let path = Path::new(&frame_names[n - 1]);
            let texture = renderer.load_texture(path).unwrap();
            frame_textures.push(texture);
        }

        Bird {
            particles: Particles::new(renderer),
            time: 0,
            x: 30,
            y: 300,
            w: 50,
            h: 43,
            dead: false,
            speed: 0.0,

            _texture_names: frame_names,
            textures: frame_textures,
        }
    }

    pub fn restart(&mut self) {
        self.y = 300;
        self.speed = 0.0;
        self.dead = false;
    }

    pub fn is_dead(&self) -> bool {
        return self.dead;
    }

    pub fn jump(&mut self) {
        self.speed = -JUMPSPEED;
    }

    pub fn touch(&mut self, p: &Pipe) {
        if p.x > self.x + self.w {
            // too far right
            return;
        }
        if p.x + p.w < self.x {
            // too far left
            return;
        }
        if !p.inverted && p.h < self.y - self.h / 2 {
            // pipe is too low
            return;
        }
        if p.inverted && 600 - p.h > self.y + self.h / 2 {
            // inverted pipe is too high
            return;
        }

        self.dead = true
    }
}

impl Displayable for Bird {
    fn on_key_down(&mut self, event: &Event) {
        match event {
            &Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                self.jump();
                self.particles.reset(self.x, self.y);
            }
            _ => {}
        }
    }

    fn update(&mut self) {
        self.time += 1;
        self.y -= self.speed as i32;
        if self.y < 0 {
            self.dead = true;
        }
        self.speed += GRAVITY;

        self.particles.update();
    }

    fn paint(&self, renderer: &mut Renderer) {
        let rect = Rect::new(self.x,
                             600 - self.y - self.h / 2,
                             self.w as u32,
                             self.h as u32);
        let tex_len = self.textures.len() as i32;
        let i = (self.time / 10 % tex_len) as usize;

        let mut current_texture = &self.textures[i];

        // Gives the bird a cool bouncing effect based on his speed.
        let degrees = (self.speed % 360.0) * 5.0;
        renderer
            .copy_ex(&mut current_texture,
                     None,
                     Some(rect),
                     degrees,
                     None,
                     false,
                     false)
            .expect("Bird should have rendered.");

        self.particles.paint(renderer);
    }
}
