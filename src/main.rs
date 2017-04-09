extern crate sdl2;
extern crate rand;

pub mod bird;
pub mod scene;
pub mod pipes;
pub mod particles;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

use std::process;
use std::path::Path;
use std::time::Duration;
use std::thread;

use sdl2::pixels::Color;
use sdl2::image::{INIT_PNG, INIT_JPG};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::mixer::{INIT_OGG, AUDIO_S16LSB};

use scene::Scene;
use bird::Bird;
use pipes::Pipes;
use particles::Particles;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let _audio = sdl_context.audio().unwrap();
    let _mixer_context = sdl2::mixer::init(INIT_OGG).unwrap();
    let music = sdl2::mixer::Music::from_file(Path::new("res/audio/soundtrack.ogg")).unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let frequency = 44100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = 2; // Stereo
    //let chunk_size = 1024;
    let chunk_size = 512;
    let _ = sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
    sdl2::mixer::allocate_channels(0);

    let window = video_subsystem
        .window("Flappy Rust", 800, 600)
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
    thread::sleep(Duration::from_millis(3000));

    // Testing a bird
    let scene = Scene::new(&mut renderer);
    let mut pipes = Pipes::new(&mut renderer);
    let mut flappy = Bird::new(&mut renderer);
    let mut particles = Particles::new(&mut renderer);

    music.play(1);

    let mut main_loop = || {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    process::exit(0);
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    process::exit(0);
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    flappy.jump();
                    particles.reset(flappy.x, flappy.y);
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        thread::sleep(Duration::from_millis(10));
        renderer.clear();

        // Update and paint scene
        scene.paint(&mut renderer);

        // Update and paint pipes
        pipes.update();
        pipes.paint(&mut renderer);

        // Update paint bird.
        flappy.update();
        flappy.paint(&mut renderer);

        // Check for collisions on bird.
        pipes.touch(&mut flappy);

        // Update particles
        particles.update();
        particles.paint(&mut renderer);

        if flappy.is_dead() {
            end_game(&mut renderer);
            thread::sleep(Duration::from_millis(3000));
            flappy.restart();
            pipes = Pipes::new(&mut renderer);
        }

        renderer.present();
    };

    #[cfg(target_os = "emscripten")]
    use emscripten::emscripten;

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop {
        main_loop();
    }
}

fn draw_title(renderer: &mut Renderer) {
    renderer.clear();

    // Load a font
    let font_path = Path::new("res/fonts/Flappy.ttf");
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context.load_font(font_path, 50).unwrap();
    font.set_style(sdl2::ttf::STYLE_BOLD);

    // Render the surface
    let surface = font.render("Flappy Rust")
        .blended(Color::RGBA(255, 87, 0, 255))
        .unwrap();
    let mut texture = renderer.create_texture_from_surface(&surface).unwrap();

    renderer.set_draw_color(Color::RGBA(0, 217, 255, 255));
    renderer.clear();

    renderer
        .copy(&mut texture, None, Some(rect!(10, 10, 790, 590)))
        .unwrap();

    renderer.present();
}

fn end_game(renderer: &mut Renderer) {
    renderer.clear();

    // Load a font
    let font_path = Path::new("res/fonts/Flappy.ttf");
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context.load_font(font_path, 50).unwrap();
    font.set_style(sdl2::ttf::STYLE_BOLD);

    // Render the surface
    let surface = font.render("Game Over!")
        .blended(Color::RGBA(255, 87, 0, 255))
        .unwrap();
    let mut texture = renderer.create_texture_from_surface(&surface).unwrap();

    renderer.set_draw_color(Color::RGBA(0, 217, 255, 255));
    renderer.clear();

    renderer
        .copy(&mut texture, None, Some(rect!(10, 10, 790, 590)))
        .unwrap();

    renderer.present();
}
