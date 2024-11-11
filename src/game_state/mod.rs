use sdl2::image::LoadTexture;

use crate::prelude::*;

pub struct State {
    pub run: bool,
    bg_colour_offset: u8,
    bg_colour_flag: bool,
    bg_colour: pixels::Color,
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    canvas: render::WindowCanvas,
    event_pump: EventPump,
    pub player: Player,
    pub time_per_frame: time::Duration,
    pub last_update_delta: time::Duration,
    clock: time::Instant,
    statistics: Statistics,
}

impl State {
    pub fn init() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let _image_context = image::init(image::InitFlag::PNG | image::InitFlag::JPG);

        let window = video_subsystem.window("Dune Sea Chronicles", DISPLAY_WIDTH, DISPLAY_HEIGHT)
            .position_centered()
            .build()
            .expect("Could not initialize video subsystem");

        let canvas = window.into_canvas()
            .build()
            .expect("Could not build canvas");

        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            run: true,
            bg_colour_offset: 0,
            bg_colour_flag: true,
            bg_colour: pixels::Color::RGB(0,0,0),
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            player: Player::new(),
            time_per_frame: time::Duration::new(0, FRAMES_PER_SECOND),
            last_update_delta: time::Duration::from_secs(0),
            clock: time::Instant::now(),
            statistics: Statistics::init(),
        }
    }

    /* start: start the game loop handling fix time steps */
    pub fn start(self: &mut Self) {
        while self.run {
            self.process_input();
            self.last_update_delta += self.clock.elapsed();
            /* gather time information to output frames / second */
            self.statistics.collect(self.last_update_delta);
            while self.last_update_delta.as_secs_f64() > self.time_per_frame.as_secs_f64() {
                self.update();
                self.render();
                self.last_update_delta -= self.time_per_frame;
            }
            self.last_update_delta += self.clock.elapsed();
            self.clock = time::Instant::now(); 

            // Lock frame rate at 60 frames per second
            //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub fn process_input(self: &mut Self) {
        // handle events
        for event in self.event_pump.poll_iter() {
            match event {
                event::Event::Quit {..} |
                event::Event::KeyDown { keycode: Some(keyboard::Keycode::Escape), .. } => {
                    self.run = false;
                },
                /* The repeat delay and the repeat rate to control repetitions can be
                 * configured on most systems. Match only on "repeat: false" to ignore repeated keys */
                event::Event::KeyDown { keycode: Some(keyboard::Keycode::W), repeat: false, .. } => {
                    self.player.toggle_movement(Direction::Up);
                    self.player.orientation = Direction::Up;
                },
                event::Event::KeyDown { keycode: Some(keyboard::Keycode::S), repeat: false, .. } => {
                    self.player.toggle_movement(Direction::Down);
                    self.player.orientation = Direction::Down;
                },
                event::Event::KeyDown { keycode: Some(keyboard::Keycode::A), repeat: false, .. } => {
                    self.player.toggle_movement(Direction::Left);
                    self.player.orientation = Direction::Left;
                },
                event::Event::KeyDown { keycode: Some(keyboard::Keycode::D), repeat: false, .. } => {
                    self.player.toggle_movement(Direction::Right);
                    self.player.orientation = Direction::Right;
                },
                event::Event::KeyUp { keycode: Some(keyboard::Keycode::W), repeat: false, ..} => {
                    self.player.toggle_movement(Direction::Up);
                },
                event::Event::KeyUp { keycode: Some(keyboard::Keycode::S), repeat: false, ..} => {
                    self.player.toggle_movement(Direction::Down);
                },
                event::Event::KeyUp { keycode: Some(keyboard::Keycode::A), repeat: false, ..} => {
                    self.player.toggle_movement(Direction::Left);
                },
                event::Event::KeyUp { keycode: Some(keyboard::Keycode::D), repeat: false, ..} => {
                    self.player.toggle_movement(Direction::Right);
                },
                _ => {}
            }
        }
        self.bg_colour = pixels::Color::RGB(64 + self.bg_colour_offset, 128, 192 - self.bg_colour_offset);
    }

    pub fn update(self: &mut Self) {
        /* update the player sprite, animation, and position */
        self.player.update_animation_frame();
        self.player.update_sprite();
        self.player.update_position();

        /* oscillate background bg_colour */
        if self.bg_colour_offset == 191 {
            self.bg_colour_flag = false;
        } else if self.bg_colour_offset == 0 {
            self.bg_colour_flag = true;
        }
        if self.bg_colour_flag {
            self.bg_colour_offset = (self.bg_colour_offset + 1) % 192;
        } else {
            self.bg_colour_offset -= 1;
        }
    }

    pub fn render(self: &mut Self) {
        self.canvas.set_draw_color(self.bg_colour);
        self.canvas.clear();

        let texture_creator = self.canvas.texture_creator(); 
        let player_texture = texture_creator.load_texture("assets/bardo.png").expect("Failed to load texture");
        let (width, height) = self.canvas.output_size().unwrap();
        let viewport_position = self.player.position + rect::Point::new(width as i32 / 2, height as i32 / 2);
        let viewport_rect = rect::Rect::from_center(viewport_position, self.player.sprite.width(), self.player.sprite.height());
        self.canvas.copy(&player_texture, self.player.sprite, viewport_rect).unwrap();

        self.canvas.present();
    }
}
