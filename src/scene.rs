extern crate sdl2;

use std::path::Path;

use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::image::{LoadTexture};

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
        // Nothing to do for the background at this point sucka.
    }

    pub fn paint(&self, renderer:&mut Renderer){
        let mut current_texture = &self.bg;
        renderer.copy(&mut current_texture, None, None).expect("Background should have rendered.");
    }

    pub fn restart(&mut self){
       
    }
}