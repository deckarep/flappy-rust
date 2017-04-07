extern crate sdl2;

mod bird;

use std::path::Path;
use std::time::Duration;
use std::thread;

use sdl2::pixels::Color;
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

use bird::Bird;



// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Flappy Bird", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();    

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // draw_title: Flappy Rust
    draw_title(&mut renderer);

    // sleep 1 second
    thread::sleep(Duration::from_millis(5000));

    // Testing a bird
    let mut flappy = Bird::new(&mut renderer);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..}  => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    flappy.jump();
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        thread::sleep(Duration::from_millis(10));
        renderer.clear();
        flappy.update();
        flappy.paint(&mut renderer);
        renderer.present();
    }
}

fn draw_title(renderer:&mut Renderer){
    renderer.clear();
    
    // Load a font
    let font_path = Path::new("res/fonts/Flappy.ttf");
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context.load_font(font_path, 50).unwrap();
    font.set_style(sdl2::ttf::STYLE_BOLD);

    // Render the surface
    let surface = font.render("Flappy Rust")
        .blended(Color::RGBA(255, 87, 0, 255)).unwrap();
    let mut texture = renderer.create_texture_from_surface(&surface).unwrap();

    renderer.set_draw_color(Color::RGBA(0, 217, 255, 255));
    renderer.clear();

    renderer.copy(&mut texture, None, Some(rect!(10,10,790,590))).unwrap();

        // Draw the Bird
    let bird_path = Path::new("res/imgs/bird_frame_1.png");
    let mut texture = renderer.load_texture(bird_path).unwrap();
    renderer.copy(&mut texture, None, None).expect("Render failed.");
    
    renderer.present();
}