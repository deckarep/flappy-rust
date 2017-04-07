# Flappy Rust

Flappy Rust is a *mostly* complete clone of Flappy Gopher which is a clone of the famous Flappy Bird game developed in Rust with bindings for SDL2.

This repo was built significantly faster by referencing Francesc Campoy's version originally based in Go: https://github.com/campoy/flappy-gopher

# Why did you write a clone of a clone?

Because I'm home all day after being layed off (along with 70% of the company) on Monday 4/3/2017 and decided to write some code in Rust since it's been on my TODO list AND because I need an outlet for my negative energy.

# Screen Capture from OSX

![Flappy Rust](https://github.com/deckarep/flappy-rust/raw/master/flappy-rust.gif)

# Thoughts on the Go version vs the Rust version

* The Rust version is not quite complete...it doesn't yet have collision detection on the pipes
* Also, I cheated a bit (due to lazy-ness) and simplifed the Scene object and event loop.
* I also pulled this version off in a single OS thread, no need to add mutexes like Francesc's version.
* The Go version needs it's main goroutine pinned to an OS thread because Go's runtime is allowed to move goroutines to different threads.  If this wasn't done then you'd have the SDL2 event-loop trying to mutate state while Go's runtime mutates state on a competing thread. For more information watch Francesc's video series called `Just for Func` where he mentions why this is important: https://www.youtube.com/channel/UC_BzFbxG2za3bp5NRRRXJSw
* By sticking to one thread, the Rust version needs no synchronization and furthermore, I can guarantee the full correctness of the application thanks to Rust's super advanced compiler. This means, even if I introduced more threads to; for example, generate the pipes similar to Francesc's version...the compiler will absolutely stop me from having data-races by refusing to compile. This means I get to sleep better at night than Francesc.
* I didn't go out of my way to optimize anything yet and I'm vaguely familiar with SDL2 since most of my work back in the day was with SDL1.
* Also, before anyone starts complaining about the code...I hacked on this in a fit of rage over a few hours...it was just for fun.
* Shoutout to Francesc Campoy https://twitter.com/francesc?lang=en a developer advocate on the Go team. Check out his video series Just for Func if you love Go.


## Installation

Please reference the rust-sdl2 crate which has installation instructions for SDL2 based on your platform: https://github.com/AngryLawyer/rust-sdl2

Additionally, you'll need the following addtional dependencies:

sdl2_ttf
sdl2_image
sdl2_mixer
sdl2_gfx

## Images, fonts, and licenses

All the images used in this game are CC0 and obtained from [Clipart](https://openclipart.org/tags/flapping).
You can find atlernative birds in there, so you can mod the game!

The fonts are copied from https://github.com/deano2390/OpenFlappyBird.

This project is licensed under Apache v2 license. Read more in the [LICENSE](LICENSE) file.