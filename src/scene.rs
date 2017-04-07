extern crate sdl2;

use std::fmt;
use std::path::Path;
use std::vec::Vec;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};

pub struct Scene{
    bg:Texture,
}

impl Scene{
    pub fn new(renderer:&mut Renderer)->Scene{
        let path = Path::new("res/imgs/background.png");
        let texture = renderer.load_texture(path).unwrap();
        
        Scene{
            bg:texture,
        }
    }

    pub fn update(&mut self){
    }

    pub fn paint(&self, renderer:&mut Renderer){
        // let rect = Rect::new(10, 600 - self.y - self.h/2, self.w as u32, self.h as u32);
        // let tex_len = self.textures.len() as i32;
        // let i = (self.time / 10 % tex_len) as usize;

        let mut current_texture = &self.bg;
        renderer.copy(&mut current_texture, None, None).expect("Background should have rendered.");
    }

    pub fn restart(&mut self){
       
    }

    fn touch(){

    }
}