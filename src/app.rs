use rand::Rng;
use ratatui::style::Color;
use rust_gpiozero::*;
use std::error;

use crate::pins::GpioPins;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct CircleSprite {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub left: Color,
    pub right: Color,
}

#[derive(Debug)]
pub struct CircleSpriteObj {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: Color,
}

#[derive(Debug)]
pub struct CircleSpriteAI {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub color: Color,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    /// To detect button's state
    pub is_pressed: bool,

    /// score counter
    pub score: i32,

    /// Players
    pub player_one: CircleSprite,
    pub player_two: CircleSprite,

    /// Game Piece
    pub movable_object: CircleSpriteObj,

    /// Obstacles
    pub positive_obstacle: CircleSpriteAI,
    pub negative_obstacle: CircleSpriteAI,
}

impl Default for App {
    fn default() -> Self {
        // spawns 2 rectanges, when contacted they disapper
        let x_pos: f64 = rand::thread_rng().gen();
        let y_pos: f64 = rand::thread_rng().gen();

        Self {
            running: true,
            is_pressed: false,
            score: 0,

            // Player one
            player_one: CircleSprite {
                x: 0.0,
                y: 0.0,
                radius: 50.0,
                left: Color::White,
                right: Color::White,
            },

            // Player two
            player_two: CircleSprite {
                x: 0.0,
                y: 0.0,
                radius: 50.0,
                left: Color::White,
                right: Color::White,
            },

            // Game object
            movable_object: CircleSpriteObj {
                x: 0.0,
                y: 0.0,
                radius: 20.0,
                color: Color::White,
            },

            positive_obstacle: CircleSpriteAI {
                x: -150.0,
                y: -50.0,
                width: 25.0,
                height: 25.0,
                color: Color::LightYellow,
            },

            negative_obstacle: CircleSpriteAI {
                x: -150.0,
                y: 50.0,
                width: 25.0,
                height: 25.0,
                color: Color::LightBlue,
            },
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.domain();
        self.interactions();

        // Score could be saved to a database
        if self.score == -10 {
            // if Button::pin(&Button::new(GpioPins::GameOver as u64))
            // .get_value()
            // .unwrap()
            // ==
            // {
            self.game_over();
            // }
        }

        if Button::pin(&Button::new(GpioPins::Power as u64))
            .get_value()
            .unwrap()
            == 0
        {
            self.remote_on();
        }

        if Button::pin(&Button::new(GpioPins::Up as u64))
            .get_value()
            .unwrap()
            == 0
        {
            // Up
            self.player_two.left = Color::Green;
            self.remote_up();
        } else {
            self.player_two.left = Color::White;
        }

        if Button::pin(&Button::new(GpioPins::Left as u64))
            .get_value()
            .unwrap()
            == 0
        {
            self.player_one.right = Color::Green;
            self.remote_left();
        } else {
            self.player_one.right = Color::White;
        }

        if Button::pin(&Button::new(GpioPins::Right as u64))
            .get_value()
            .unwrap()
            == 0
        {
            self.player_one.left = Color::Green;
            self.remote_right();
        } else {
            self.player_one.left = Color::White;
        }

        if Button::pin(&Button::new(GpioPins::Down as u64))
            .get_value()
            .unwrap()
            == 0
        {
            // Down
            self.player_two.right = Color::Green;
            self.remote_down();
        } else {
            self.player_two.right = Color::White;
        }

        if Button::pin(&Button::new(GpioPins::PowerOff as u64))
            .get_value()
            .unwrap()
            == 0
        {
            self.running = false;
            self.remote_power_off();
            self.remote_off();
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn remote_on(&mut self) {
        std::thread::Builder::new()
            .name("Front Light".to_string())
            .spawn(|| {
                GpioPins::activate(GpioPins::Power);
            })
            .ok();
    }

    pub fn point_gained(&mut self) {
        std::thread::Builder::new()
            .name("Point Gained".to_string())
            .spawn(|| {
                GpioPins::scoring(GpioPins::PointGain);
            })
            .ok();
    }

    pub fn point_removed(&mut self) {
        std::thread::Builder::new()
            .name("Front Lost".to_string())
            .spawn(|| {
                GpioPins::scoring(GpioPins::PointLoss);
            })
            .ok();
    }

    pub fn remote_off(&mut self) {
        std::thread::Builder::new()
            .name("Remote Off".to_string())
            .spawn(|| {
                GpioPins::deactivate(GpioPins::Power);
            })
            .ok();
    }

    pub fn remote_up(&mut self) {
        std::thread::Builder::new()
            .name("Moved Up".to_string())
            .spawn(|| {
                GpioPins::movements(GpioPins::Up);
            })
            .ok();

        self.movable_object.y += 10.0;
    }

    pub fn remote_left(&mut self) {
        std::thread::Builder::new()
            .name("Moved Left".to_string())
            .spawn(|| {
                GpioPins::movements(GpioPins::Left);
            })
            .ok();

        self.movable_object.x -= 10.0;
    }

    pub fn remote_right(&mut self) {
        std::thread::Builder::new()
            .name("Moved Right".to_string())
            .spawn(|| {
                GpioPins::movements(GpioPins::Right);
            })
            .ok();

        self.movable_object.x += 10.0;
    }

    pub fn remote_down(&mut self) {
        std::thread::Builder::new()
            .name("Moved Down".to_string())
            .spawn(|| {
                GpioPins::movements(GpioPins::Down);
            })
            .ok();

        self.movable_object.y -= 10.0;
    }

    pub fn remote_over(&mut self) {
        std::thread::Builder::new()
            .name("Game Over".to_string())
            .spawn(|| {
                GpioPins::last(GpioPins::GameOver);
            })
            .ok();
    }

    pub fn remote_power_off(&mut self) {
        std::thread::Builder::new()
            .name("Power Off".to_string())
            .spawn(|| {
                GpioPins::power_off(GpioPins::PowerOff);
            })
            .ok();
    }

    pub fn move_left(&mut self) {
        self.movable_object.x -= 10.0;
    }

    pub fn move_right(&mut self) {
        self.movable_object.x += 10.0;
    }

    pub fn move_up(&mut self) {
        self.movable_object.y += 10.0;
    }

    pub fn move_down(&mut self) {
        self.movable_object.y -= 10.0;
    }

    pub fn finalize_score(&mut self) {
        self.remote_over();
    }
    pub fn add_score(&mut self) {
        self.score += 1;
        self.point_gained();
    }

    pub fn sub_score(&mut self) {
        self.score -= 1;
        self.point_removed();
    }

    pub fn game_over(&mut self) {
        self.remote_over();
        // Resets stuff to original
        self.score = 0;
        self.movable_object.x = 0.0;
        self.movable_object.y = 0.0;

        self.positive_obstacle.x = -150.0;
        self.positive_obstacle.y = -50.0;

        self.negative_obstacle.x = -150.0;
        self.negative_obstacle.y = 50.0;
    }

    pub fn domain(&mut self) {
        // Movable Object going left
        if self.movable_object.x > 180.0 {
            self.movable_object.x = -180.0;
        }

        // Movable Object going up
        if self.movable_object.y > 90.0 {
            self.movable_object.y = -90.0;
        }

        // Movable Object going right
        if self.movable_object.x < -180.0 {
            self.movable_object.x = 180.0;
        }

        // Movable Object going left
        if self.movable_object.y < -90.0 {
            self.movable_object.y = 90.0;
        }
    }

    pub fn generate_objects(&mut self) {
        let x_pos: f64 = rand::thread_rng().gen();
        let y_pos: f64 = rand::thread_rng().gen();

        self.positive_obstacle.x = self.round_to_nearest(x_pos * 180.0, 10.0);
        self.positive_obstacle.y = self.round_to_nearest(y_pos * 70.0, 10.0);
        //
        self.negative_obstacle.x = self.round_to_nearest(x_pos * -160.0, 10.0);
        self.negative_obstacle.y = self.round_to_nearest(y_pos * -70.0, 10.0);
    }

    pub fn round_to_nearest(&mut self, value: f64, base: f64) -> f64 {
        ((value / base).round() * base).into()
    }

    pub fn interactions(&mut self) {
        // Positive: yellow, Negative: blue

        // You hit the yellow from right
        if self.movable_object.x == self.positive_obstacle.x + 40.0
            && self.movable_object.y == self.positive_obstacle.y + 10.0
        {
            self.add_score();
        }

        // You hit the blue from right
        if self.movable_object.x == self.negative_obstacle.x + 40.0
            && self.movable_object.y == self.negative_obstacle.y + 10.0
        {
            self.add_score();
        }

        // You hit the yellow from left
        if self.movable_object.x == self.positive_obstacle.x - 20.0
            && self.movable_object.y == self.positive_obstacle.y + 10.0
        {
            self.add_score();
        }

        // You hit the blue from left
        if self.movable_object.x == self.negative_obstacle.x - 20.0
            && self.movable_object.y == self.negative_obstacle.y + 10.0
        {
            self.add_score();
        }

        // You hit the yellow from bottom
        if self.movable_object.x == self.positive_obstacle.x + 20.0
            // alongside the x-axis
            || self.movable_object.x == self.negative_obstacle.x + 10.0
                && self.movable_object.y == self.positive_obstacle.y + 80.0
        {
            self.sub_score();
        }

        // You hit the blue from bottom
        if self.movable_object.x == self.negative_obstacle.x + 20.0
            // alongside the x-axis
            || self.movable_object.x == self.negative_obstacle.x + 10.0
                && self.movable_object.y == self.negative_obstacle.y - 120.0
        {
            self.sub_score();
        }
    }
}
