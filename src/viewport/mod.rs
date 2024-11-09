use crate::{prelude::*, geometry};

#[derive(Debug)]
pub struct Viewport {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Viewport {
    pub fn new(player_position: &Point) -> Self {
        Self {
            left_x: player_position.x - (VIEWPORT_WIDTH),
            right_x: player_position.x + (VIEWPORT_WIDTH),
            top_y: player_position.y - (VIEWPORT_HEIGHT),
            bottom_y: player_position.y + (VIEWPORT_HEIGHT),
        }
    }

    /* on_player_move: Update the viewport to be centered on the player as the player moves */
    pub fn on_player_move(&mut self, &player_position: rect::Point) {
        let HORIZONTAL_OFFSET: i32 = VIEWPORT_WIDTH as i32 / 2;
        let VERTICAL_OFFSET: i32 = VIEWPORT_HEIGHT as i32 / 2;
        self.left_x = player_position.x - HORIZONTAL_OFFSET;
        self.right_x = player_position.x + HORIZONTAL_OFFSET;
        self.top_y = player_position.y - VERTICAL_OFFSET;
        self.bottom_y = player_position.y + VERTICAL_OFFSET;
    }
}
