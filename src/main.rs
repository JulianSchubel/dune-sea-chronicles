#[allow(dead_code)]

use dune_sea_chronicles::prelude::*;

fn main() -> Result<(), String> {
    let mut state: State = State::init();
    while state.run {
        state.process_input();
        state.update();
        state.render();
        // Lock frame rate at 60 frames per second
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
