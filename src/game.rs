use speedy2d::dimen::Vec2;
use speedy2d::image::{ImageFileFormat, ImageHandle, ImageSmoothingMode};
use speedy2d::shape::Rectangle;
use speedy2d::{dimen::UVec2, Graphics2D};

use crate::app::{Keyboard, Mouse};
use crate::config::Config;
use crate::spritesheet::Spritesheet;

pub struct Game {
    config: Config,
    images: Vec<ImageHandle>,
    spritesheets: Vec<Spritesheet>,

    viewport_size: UVec2,
}

impl Game {
    pub const fn new(config: Config) -> Self {
        let viewport_size = UVec2::new(config.window_width, config.window_height);
        Self {
            config,
            images: Vec::new(),
            spritesheets: Vec::new(),
            viewport_size,
        }
    }

    pub fn setup(&mut self, graphics: &mut Graphics2D) {
        let image_handle = graphics
            .create_image_from_file_path(
                Some(ImageFileFormat::PNG),
                ImageSmoothingMode::Linear,
                "spritesheet.png",
            )
            .unwrap();
        //self.images.push(spritesheet);
        self.spritesheets.push(Spritesheet::new(image_handle, 5, 6));
    }

    pub fn input(&mut self, viewport_size: UVec2, _mouse: &Mouse, _keyboard: &Keyboard) {
        self.viewport_size = viewport_size;
    }

    pub fn update(&mut self) {}

    pub fn draw(&self, graphics: &mut Graphics2D) {
        let spritesheet = self.spritesheets.first().unwrap();
        let pattern_size = 16;

        for x in 0..=self.viewport_size.x / pattern_size {
            for y in 0..=self.viewport_size.y / pattern_size {
                let sprite_x = fastrand::u32(0..spritesheet.width);
                let sprite_y = fastrand::u32(0..spritesheet.height);
                let top_left = UVec2::new(x * pattern_size, y * pattern_size).into_f32();
                let destination = Rectangle::new(
                    top_left,
                    top_left + Vec2::new(pattern_size as f32, pattern_size as f32),
                );
                spritesheet.draw_sprite(&destination, sprite_x, sprite_y, graphics);
            }
        }
    }
}
