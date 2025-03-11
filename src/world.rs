use crate::input::Input;

pub struct World {
    pub input: Input,
    pub playing: bool,
}

impl World {
    pub fn new() -> World {
        World {
            input: Input::Wait(5),
            playing: true,
        }
    }

    pub fn tick(&mut self) {
        match self.input {
            Input::Quit => self.playing = false,
            Input::Wait(frames) => for _ in 0..frames {self.wait()},
        }
    }
    fn wait(&mut self) {}
}
