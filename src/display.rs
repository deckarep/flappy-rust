extern crate sdl2;

use sdl2::render::Renderer;

// Displayable is any type that be updated and rendered to the screen.
pub trait Displayable {
    // update handles only updating the internal state of a Displayable object.
    fn update(&mut self);

    // paint handles the actual painting of the Displayable object against a Renderer.
    fn paint(&self, renderer: &mut Renderer);
}
