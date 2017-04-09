extern crate sdl2;

use std::path::Path;
use std::vec::Vec;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::render::BlendMode;
use sdl2::image::{LoadTexture};

use rand::{thread_rng, Rng};

const GRAVITY:f64 =  0.2;
const NUM_PARTICLES:i32 = 15;

pub struct Particles{
    texture:Texture,
    particles:Vec<StarParticle>,
}

impl Particles{
    pub fn new(renderer:&mut Renderer)->Particles{
        let path = Path::new("res/imgs/star.png");
        let mut texture = renderer.load_texture(path).unwrap();
        texture.set_blend_mode(BlendMode::Add);

        let mut pieces:Vec<StarParticle> = Vec::new();
        for _ in 1..NUM_PARTICLES{
            pieces.push(StarParticle::new(30, 300))
        }

        Particles{
            texture:texture,
            particles:pieces,
        }
    }

    pub fn reset(&mut self, start_x:i32, start_y:i32){
        let mut pieces:Vec<StarParticle> = Vec::new();
        for _ in 1..NUM_PARTICLES{
            pieces.push(StarParticle::new(start_x, 600-start_y))
        }

        self.particles = pieces;
    }

    pub fn update(&mut self){
        let mut remaining_particles:Vec<StarParticle> = Vec::new();
        for p in &mut self.particles{
            p.update();
            if !p.dead{
                remaining_particles.push(p.clone());
            }
        }
        self.particles = remaining_particles;
    }

    pub fn paint(&self, renderer:&mut Renderer){
        let current_texture = &self.texture;
        for p in &self.particles{
            p.paint(renderer, current_texture);
        }
    }
}

#[derive(Copy)]
pub struct StarParticle{
    x:i32,
    xvel:i32,
    y:i32,
    dead:bool,
    speed:f64,
}

impl Clone for StarParticle {
    fn clone(&self) -> StarParticle { *self }
}

impl StarParticle{
    pub fn new(start_x:i32, start_y:i32)->StarParticle{
        StarParticle{
            x:start_x + thread_rng().gen_range(-15,15),
            y:start_y + thread_rng().gen_range(-5,5),
            xvel:thread_rng().gen_range(-5, 5),
            dead:false,
            speed:thread_rng().gen_range(1,4) as f64,
        }
    }

    pub fn reset(&mut self, start_x:i32, start_y:i32){
        self.x=start_x + thread_rng().gen_range(-15,15);
        self.y=start_y + thread_rng().gen_range(-5,5);
        self.xvel = thread_rng().gen_range(-5, 5);
        self.dead=false;
        self.speed=thread_rng().gen_range(1,5) as f64;
    }

    pub fn update(&mut self){
        self.x += self.xvel;
	    self.y += self.speed as i32;
	    if self.y < 0 {
		    self.dead = true;
	    }
	    self.speed += GRAVITY;
    }

    pub fn paint(&self, renderer:&mut Renderer, texture:&Texture){
        let rect = Rect::new(
            self.x, 
            self.y, 
            20, 
            20,
        );
        renderer.copy_ex(texture, None, Some(rect), 0.0, None, false, false).expect("Single star particle should have rendered.");
    }
}