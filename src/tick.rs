use crate::World;
use crate::input::Input;

pub fn tick(world: &mut World) {
    match world.input {
        Input::Quit => world.playing = false,
        Input::Wait(frames) => for _ in 0..frames {wait()},
    }
}

fn wait() {}
