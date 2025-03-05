use super::World;

pub fn input(world: &mut World) {
    world.input = Input::Quit;
}

pub enum Input {
    Quit,
    Wait(u32),
}
