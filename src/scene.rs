extern crate sdl2;

use std::path::Path;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::image::LoadTexture;
use sdl2::event::Event;
use std::cell::RefCell;
use std::rc::Rc;
use sdl2::keyboard::Keycode;

use display::Displayable;
use bird::Bird;
use pipes::Pipes;



pub struct Scene {
    // Internal state.
    paused: bool,

    // Objects.
    // https://www.reddit.com/r/rust/comments/4ij34q/how_to_use_rcrefcellt_properly/
    // I did this because I want to share assets to control individually within the scene.
    // And additionally control generically as Displayable objects.
    flappy: Rc<RefCell<Bird>>,
    pipes: Rc<RefCell<Pipes>>,
    //particles: Rc<RefCell<Particles>>,

    // Generic.
    children: Vec<Rc<RefCell<Displayable>>>,
    layer0: Texture,
    layer1: Texture,
    layer2: Texture,
    layer3: Texture,
    layer3_x: i32,
    layer3_x2: i32,
    layer4: Texture,
    layer4_x: i32,
    layer4_x2: i32,
}

// TODO: refactor this code since it's all copy pasta...but scrolling now works!
impl Scene {
    pub fn new(renderer: &Renderer) -> Scene {
        let flappy = Rc::new(RefCell::new(Bird::new(renderer)));
        let pipes = Rc::new(RefCell::new(Pipes::new(renderer)));

        let mut children: Vec<Rc<RefCell<Displayable>>> = Vec::new();
        children.push(flappy.clone());
        children.push(pipes.clone());

        let s = Scene {
            flappy: flappy.clone(),
            pipes: pipes.clone(),
            paused: false,
            children: children,
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
            layer3_x: 0,
            layer3_x2: 800,
            layer4: renderer
                .load_texture(Path::new("res/imgs/layer_05_1920 x 1080.png"))
                .unwrap(),
            layer4_x: 0,
            layer4_x2: 800,
        };

        return s;
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Displayable>>) {
        self.children.push(child);
    }

    pub fn restart(&mut self) {}
}

impl Displayable for Scene {
    fn on_key_down(&mut self, event: &Event) {
        match event {
            &Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                self.paused = !self.paused;
            }
            _ => {}
        }

        //TODO: allow cancel propagating events based on logic in parent.
        for child in &self.children {
            child.borrow_mut().on_key_down(event);
        }
    }

    fn on_key_up(&mut self, event: &Event) {
        // TODO: allow cancel propagating events based on logic in parent.
        // for child in &mut self.children {
        //     child.on_key_up(event);
        // }
    }

    fn update(&mut self) {
        if self.paused {
            return;
        }

        // Nothing to do for the background at this point sucka.
        self.layer3_x -= 1;
        self.layer3_x2 -= 1;

        if self.layer3_x < -800 {
            self.layer3_x = 800;
        }
        if self.layer3_x2 < -800 {
            self.layer3_x2 = 800;
        }

        self.layer4_x -= 1;
        self.layer4_x2 -= 1;

        if self.layer4_x < -800 {
            self.layer4_x = 800;
        }
        if self.layer4_x2 < -800 {
            self.layer4_x2 = self.layer4_x + 800 - 2;
        }

        //TODO: allow cancel propagating events based on logic in parent.
        for child in &self.children {
            child.borrow_mut().update();
        }
    }

    fn paint(&self, renderer: &mut Renderer) {
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

        let rect = Rect::new(self.layer3_x, 0, 800, 600);
        let mut current_texture = &self.layer3;
        renderer
            .copy(&mut current_texture, None, Some(rect))
            .expect("Layer3 should have rendered.");

        let rect = Rect::new(self.layer3_x2, 0, 800, 600);
        let mut current_texture = &self.layer3;
        renderer
            .copy(&mut current_texture, None, Some(rect))
            .expect("Layer3 should have rendered.");

        let rect = Rect::new(self.layer4_x, 0, 800, 600);
        let mut current_texture = &self.layer4;
        renderer
            .copy(&mut current_texture, None, Some(rect))
            .expect("Layer4 should have rendered.");

        let rect = Rect::new(self.layer4_x2, 0, 800, 600);
        let mut current_texture = &self.layer4;
        renderer
            .copy(&mut current_texture, None, Some(rect))
            .expect("Layer4 should have rendered.");

        for child in &self.children {
            child.borrow_mut().paint(renderer);
        }
    }
}
