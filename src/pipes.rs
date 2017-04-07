extern crate sdl2;

use std::path::Path;
use std::vec::Vec;

use rand::{thread_rng, Rng};

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::image::{LoadTexture};

use bird::Bird;

pub struct Pipes{
    speed:f64,
    texture:Texture,
    pipes:Vec<Pipe>,
}

impl Pipes{
    pub fn new(renderer:&mut Renderer)-> Pipes{
        // Must keep the names around as owned strings.
        let path = Path::new("res/imgs/pipe.png");
        let pipe_texture = renderer.load_texture(path).unwrap();

        Pipes{
            speed:4.0,
            texture:pipe_texture,
            pipes:vec!(Pipe::new()),
        }
    }

    pub fn paint(&self, renderer:&mut Renderer){
        let current_texture = &self.texture;
        for p in &self.pipes{
            p.paint(renderer, current_texture);
        }
    }

    pub fn update(&mut self){
        let mut remaining_pipes:Vec<Pipe> = Vec::new();
        for p in &mut self.pipes{
            p.x -= self.speed as i32;
            if p.x+p.w > 0{
                remaining_pipes.push(p.clone());
            }else{
                // Adds a new pipe when the last one is dead.
                remaining_pipes.push(Pipe::new());
            }
        }
        self.pipes = remaining_pipes;
    }

    pub fn restart(&mut self){
        self.pipes = Vec::new();
    }

    pub fn touch(&mut self, bird:&Bird){
        for p in &self.pipes{
            p.touch(bird);
        }
    }
}

#[derive(Copy)]
pub struct Pipe{
    x:i32,
    h:i32,
    w:i32,
    inverted:bool,
}

impl Clone for Pipe {
    fn clone(&self) -> Pipe { *self }
}

impl Pipe{
    pub fn new()->Pipe{
        let mut inverted = false;

        // Add some variation.
        if thread_rng().gen_range(0,10) > 5{
            inverted = true;
        }
        
        Pipe{
            x:800,
            h:100 + thread_rng().gen_range(0,300) as i32,
		    w:50,
            inverted:inverted,
        }
    }

    pub fn paint(&self, renderer:&mut Renderer, texture:&Texture){
        let mut rect = Rect::new(
            self.x, 
            600 - self.h, 
            self.w as u32, 
            self.h as u32
        );

        let mut flip = false;
        if self.inverted{
            rect.y = 0;
            flip = true;
        }

        renderer.copy_ex(texture, None, Some(rect), 0.0, None, false, flip).expect("Single pipe should have rendered.");
    }
    
    pub fn touch(&self, _bird:&Bird){
        // TODO handle touch code.
        // bird.touch(self);
    }
}

