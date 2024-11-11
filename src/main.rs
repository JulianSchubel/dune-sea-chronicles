#[allow(dead_code)]

use dune_sea_chronicles::prelude::*;

fn main() -> Result<(), String> {
    let mut game_state: State = State::init();
    game_state.start();
    Ok(())
}
