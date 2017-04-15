extern crate sdl2;

use std::path::Path;
use std::vec::Vec;
use std::rc::Rc;
use rand::{thread_rng, Rng};

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::render::BlendMode;
use sdl2::image::LoadTexture;

use display::Displayable;

const GRAVITY: f64 = 0.2;
const NUM_PARTICLES: i32 = 15;

pub struct Particles {
    texture: Rc<Texture>,
    particles: Vec<StarParticle>,
}

impl Particles {
    pub fn new(renderer: &mut Renderer) -> Particles {
        let path = Path::new("res/imgs/star.png");
        let mut texture = renderer.load_texture(path).unwrap();
        texture.set_blend_mode(BlendMode::Add);

        let rc_texture = Rc::new(texture);

        let mut pieces: Vec<StarParticle> = Vec::new();
        for _ in 1..NUM_PARTICLES {
            pieces.push(StarParticle::new(rc_texture.clone(), 30, 300))
        }

        Particles {
            texture: rc_texture,
            particles: pieces,
        }
    }

    pub fn reset(&mut self, start_x: i32, start_y: i32) {
        let mut pieces: Vec<StarParticle> = Vec::new();
        for _ in 1..NUM_PARTICLES {
            pieces.push(StarParticle::new(self.texture.clone(), start_x, 600 - start_y))
        }

        self.particles = pieces;
    }
}

impl Displayable for Particles {
    fn update(&mut self) {
        for p in &mut self.particles {
            p.update();
        }
    }

    fn paint(&self, renderer: &mut Renderer) {
        for p in &self.particles {
            p.paint(renderer);
        }
    }
}

pub struct StarParticle {
    x: i32,
    xvel: i32,
    y: i32,
    dead: bool,
    speed: f64,
    texture: Rc<Texture>,
}

impl StarParticle {
    pub fn new(texture: Rc<Texture>, start_x: i32, start_y: i32) -> StarParticle {
        StarParticle {
            x: start_x + thread_rng().gen_range(-15, 15),
            y: start_y + thread_rng().gen_range(-5, 5),
            xvel: thread_rng().gen_range(-5, 5),
            dead: false,
            speed: thread_rng().gen_range(1, 4) as f64,
            texture: texture,
        }
    }

    pub fn reset(&mut self, start_x: i32, start_y: i32) {
        self.x = start_x + thread_rng().gen_range(-15, 15);
        self.y = start_y + thread_rng().gen_range(-5, 5);
        self.xvel = thread_rng().gen_range(-5, 5);
        self.dead = false;
        self.speed = thread_rng().gen_range(1, 5) as f64;
    }
}

impl Displayable for StarParticle {
    fn update(&mut self) {
        self.x += self.xvel;
        self.y += self.speed as i32;
        if self.y < 0 {
            self.dead = true;
        }
        self.speed += GRAVITY;
    }

    fn paint(&self, renderer: &mut Renderer) {
        let rect = Rect::new(self.x, self.y, 20, 20);
        renderer
            .copy_ex(&self.texture, None, Some(rect), 0.0, None, false, false)
            .expect("Single star particle should have rendered.");
    }
}
