pub mod game_state;
pub mod geometry;
pub mod player;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub mod prelude {

    pub const DISPLAY_WIDTH: u32 = 800;
    pub const DISPLAY_HEIGHT: u32 = 600;
    pub const VIEWPORT_WIDTH: u32 = DISPLAY_WIDTH / 2;
    pub const VIEWPORT_HEIGHT: u32 = DISPLAY_HEIGHT / 2;
    pub const BASE_PLAYER_SPEED: i32 = 3;

    pub use sdl2::{*, render::TextureCreator, render::Texture};
    
    pub use std::time::Duration;
    pub use crate::game_state::*;
    pub use crate::player::*;
    pub use crate::geometry::*;
    pub use crate::Direction;
}
