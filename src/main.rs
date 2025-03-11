mod world;
mod display;
mod input;

use world::World;

const TERMINAL_SIZE: (u32, u32) = (238, 59);
const BOTTOM_SPACE: u32 = 10;

fn main() {
    let mut world = World::new();
    display::display_all();

    while world.playing {
        input::input(&mut world);

        world.tick();

        display::display_all();
    }
}
