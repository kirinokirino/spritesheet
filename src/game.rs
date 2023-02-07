use speedy2d::color::Color;
use speedy2d::dimen::{IVec2, Vec2};
use speedy2d::image::{ImageFileFormat, ImageHandle, ImageSmoothingMode};
use speedy2d::shape::Rectangle;
use speedy2d::{dimen::UVec2, Graphics2D};

use crate::app::{Keyboard, Mouse};
use crate::config::Config;
use crate::spritesheet::draw_sprite;

pub struct Game {
    config: Config,
    images: Vec<ImageHandle>,

    viewport_size: UVec2,
}

impl Game {
    pub const fn new(config: Config) -> Self {
        let viewport_size = UVec2::new(config.window_width, config.window_height);
        Self {
            config,
            images: Vec::new(),
            viewport_size,
        }
    }

    pub fn setup(&mut self, graphics: &mut Graphics2D) {
        let spritesheet = graphics
            .create_image_from_file_path(
                Some(ImageFileFormat::PNG),
                ImageSmoothingMode::Linear,
                "spritesheet.png",
            )
            .unwrap();
        self.images.push(spritesheet);
    }

    pub fn input(&mut self, viewport_size: UVec2, mouse: &Mouse, keyboard: &Keyboard) {
        self.viewport_size = viewport_size;
    }

    pub fn update(&mut self) {}

    pub fn draw(&self, graphics: &mut Graphics2D) {
        let spritesheet = self.images.first().unwrap();
        let pattern_size = 16;
        let spritesheet_width = 5u8;
        let spritesheet_height = 6u8;

        for x in 0..=self.viewport_size.x / pattern_size {
            for y in 0..=self.viewport_size.y / pattern_size {
                let sprite_x = fastrand::u8(0..spritesheet_width);
                let sprite_y = fastrand::u8(0..spritesheet_height);
                let top_left = UVec2::new(x * pattern_size, y * pattern_size).into_f32();
                let destination = Rectangle::new(
                    top_left,
                    top_left + Vec2::new(pattern_size as f32, pattern_size as f32),
                );
                draw_sprite(
                    &destination,
                    spritesheet,
                    sprite_x.into(),
                    sprite_y.into(),
                    spritesheet_width.into(),
                    spritesheet_height.into(),
                    graphics,
                );
            }
        }
    }
}
