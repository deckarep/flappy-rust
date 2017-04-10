extern crate sdl2;

use std::path::Path;

use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::image::LoadTexture;

pub struct Scene {
    layer0: Texture,
    layer1: Texture,
    layer2: Texture,
    layer3: Texture,
    layer4: Texture,
}

impl Scene {
    pub fn new(renderer: &mut Renderer) -> Scene {
        Scene {
            layer0: renderer
                .load_texture(Path::new("res/imgs/layer_01_1920 x 1080.png"))
                .unwrap(),
            layer1: renderer
                .load_texture(Path::new("res/imgs/layer_02_1920 x 1080.png"))
                .unwrap(),
            layer2: renderer
                .load_texture(Path::new("res/imgs/layer_03_1920 x 1080.png"))
                .unwrap(),
            layer3: renderer
                .load_texture(Path::new("res/imgs/layer_04_1920 x 1080.png"))
                .unwrap(),
            layer4: renderer
                .load_texture(Path::new("res/imgs/layer_05_1920 x 1080.png"))
                .unwrap(),
        }
    }

    pub fn update(&mut self) {
        // Nothing to do for the background at this point sucka.
    }

    pub fn paint(&self, renderer: &mut Renderer) {
        let mut current_texture = &self.layer0;
        renderer
            .copy(&mut current_texture, None, None)
            .expect("Layer0 should have rendered.");

        let mut current_texture = &self.layer1;
        renderer
            .copy(&mut current_texture, None, None)
            .expect("Layer1 should have rendered.");

        let mut current_texture = &self.layer2;
        renderer
            .copy(&mut current_texture, None, None)
            .expect("Layer2 should have rendered.");

        let mut current_texture = &self.layer3;
        renderer
            .copy(&mut current_texture, None, None)
            .expect("Layer3 should have rendered.");

        let mut current_texture = &self.layer4;
        renderer
            .copy(&mut current_texture, None, None)
            .expect("Layer4 should have rendered.");
    }

    pub fn restart(&mut self) {}
}
