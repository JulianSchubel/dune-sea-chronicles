use crate::prelude::*;

#[derive(Debug)]
pub struct Player {
    /* Top left corner of player sprite */
    pub position: rect::Point, 
    /* Rectangulare section of texture representing the player */
    pub sprite: rect::Rect,
    /* The current animation frame for the player sprite */
    animation_frame: i32,
    animation_clock: time::Instant,
    animations_per_second: time::Duration,
    /* Orientation of the player sprite */
    pub orientation: Direction,
    /* The determines magnitude of the players displacement */
    pub movement_speed: i32,
    /* Determine player movement */
    moving: bool,
    moving_up: bool,
    moving_down: bool,
    moving_left: bool,
    moving_right: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: rect::Point::new(0, 0),
            sprite: rect::Rect::new(0, 0, 26, 36),
            animation_frame: 0,
            animation_clock: time::Instant::now(),
            animations_per_second: time::Duration::new(0, PLAYER_ANIMATIONS_PER_SECOND),
            orientation: Direction::Up,
            movement_speed: BASE_PLAYER_SPEED,
            moving: false,
            moving_up: false,
            moving_down: false,
            moving_left: false,
            moving_right: false,
        }
    }

    pub fn toggle_movement(self: &mut Self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.moving_up = !self.moving_up;
            },
            Direction::Down => {
                self.moving_down = !self.moving_down;
            },
            Direction::Left => {
                self.moving_left = !self.moving_left;
            },
            Direction::Right => {
                self.moving_right = !self.moving_right;
            }
        };
    }

    pub fn update_sprite(self: &mut Self) {
        let (sprite_width, sprite_height) = self.sprite.size();

        let spritesheet_x = sprite_width as i32 * self.animation_frame;
        let spritesheet_y = sprite_height as i32 * self.orientation_spritesheet_row();
        self.sprite = rect::Rect::new(spritesheet_x, spritesheet_y, sprite_width, sprite_height);
    }

    fn orientation_spritesheet_row(self: &mut Self) -> i32 {
        use self::Direction::*;

        match self.orientation {
            Up => 3,
            Down => 0,
            Left => 1,
            Right => 2,
        }
    }

    pub fn update_animation_frame(self: &mut Self) {
        let moving: bool = self.moving_up 
                || self.moving_down 
                || self.moving_left 
                || self.moving_right;
        while self.animation_clock.elapsed() > self.animations_per_second {
            if moving { 
                self.animation_frame = (self.animation_frame + 1) % 3
            } else {
                self.animation_frame = 0;
            };
            self.animation_clock = time::Instant::now();
        }
    }

    pub fn update_position(&mut self) {
        if self.moving_up {
            self.position.y -= BASE_PLAYER_SPEED;
        }

        if self.moving_down {
            self.position.y += BASE_PLAYER_SPEED 
        }

        if self.moving_left {
            self.position.x -= BASE_PLAYER_SPEED;
        }

        if self.moving_right {
            self.position.x += BASE_PLAYER_SPEED;
        }
    }
}
