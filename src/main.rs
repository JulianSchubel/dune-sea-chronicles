#[allow(dead_code)]

use dune_sea_chronicles::prelude::*;

fn main() -> Result<(), String> {
    let mut game_state: State = State::init();
    while game_state.run {
        game_state.start();
    }
    Ok(())
}
