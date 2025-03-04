mod tick;
mod display;
mod input;

fn main() {
    display::display_all();

    'MainLoop: loop {
        input::input();
        // TODO handle the quit case
        // to break out of the MainLoop

        tick::tick(1);

        display::display_all();
    }
}
