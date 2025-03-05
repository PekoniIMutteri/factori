mod tick;
mod display;
mod input;

fn main() {
    let mut world = World::new();
    display::display_all();

    while world.playing {
        input::input(&mut world);

        tick::tick(&mut world);

        display::display_all();
    }
}

struct World {
    input: input::Input,
    playing: bool,
}

impl World {
    fn new() -> World {
        World {
            input: input::Input::Wait(5),
            playing: true,
        }
    }
}
