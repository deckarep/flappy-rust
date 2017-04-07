extern crate sdl2;

use std::fmt;
use std::path::Path;
use std::vec::Vec;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};


const GRAVITY:f64 =  0.2;
const JUMPSPEED:f64 =  8.0;

pub struct Bird{
    time:i32,
    x:i32,
    y:i32,
    w:i32,
    h:i32,
    speed:f64,
    dead:bool,

    texture_names:Vec<String>,
    textures:Vec<Texture>,
}

impl Bird{
    pub fn new(renderer:&mut Renderer)->Bird{
        // Must keep the names around as owned strings.
        let mut frame_names:Vec<String> = Vec::new();
        let mut frame_textures:Vec<Texture> = Vec::new();
        for n in 1..4{
            frame_names.push(format!("res/imgs/bird_frame_{}.png", n));
            let path = Path::new(&frame_names[n-1]);
            let mut texture = renderer.load_texture(path).unwrap();
            frame_textures.push(texture);
        }

        Bird{
            time:0,
            x:10,
            y:300,
            w:50,
            h:43,
            dead:false,
            speed:0.0,

            texture_names:frame_names,
            textures:frame_textures,
        }
    }

    pub fn update(&mut self){
        self.time+=1;
	    self.y -= self.speed as i32;
	    if self.y < 0 {
		    self.dead = true;
	    }
	    self.speed += GRAVITY;
    }

    pub fn paint(&self, renderer:&mut Renderer){
        let rect = Rect::new(10, 600 - self.y - self.h/2, self.w as u32, self.h as u32);
        let tex_len = self.textures.len() as i32;
        let i = (self.time / 10 % tex_len) as usize;

        let mut current_texture = &self.textures[i];
        renderer.copy(&mut current_texture, None, Some(rect)).expect("Bird should have rendered.");
    }

    pub fn restart(&mut self){
        self.y = 300;
	    self.speed = 0.0;
	    self.dead = false;
    }

    pub fn is_dead(&self) -> bool{
        return self.dead;
    }

    pub fn jump(&mut self){
        self.speed = -JUMPSPEED;
    }

    fn touch(){

    }
}