extern crate sdl2;

use std::fmt;

use sdl2::render::Renderer;
use sdl2::event::Event;

// Displayable is any type that be updated and rendered to the screen.
pub trait Displayable {
    // update handles only updating the internal state of a Displayable object.
    fn update(&mut self);

    // paint handles the actual painting of the Displayable object against a Renderer.
    fn paint(&self, renderer: &mut Renderer);

    // on_key_down handles a key down event with a default implmentation of noop.
    fn on_key_down(&mut self, event: &Event) {}

    // on_key_up handles a key up event with a default implmentation of noop.
    fn on_key_up(&mut self, event: &Event) {}
}

impl fmt::Debug for Displayable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This is a bird!")
    }
}
