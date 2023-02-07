use speedy2d::color::Color;
use speedy2d::dimen::{IVec2, Vec2};
use speedy2d::image::{ImageFileFormat, ImageHandle, ImageSmoothingMode};
use speedy2d::shape::Rectangle;
use speedy2d::{dimen::UVec2, Graphics2D};

use crate::app::{Keyboard, Mouse};
use crate::config::Config;
pub struct Game {
    config: Config,
    images: Vec<ImageHandle>,
}

impl Game {
    pub const fn new(config: Config) -> Self {
        Self {
            config,
            images: Vec::new(),
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
    pub fn input(&mut self, viewport_size: UVec2, mouse: &Mouse, keyboard: &Keyboard) {}
    pub fn update(&mut self) {}
    pub fn draw(&self, graphics: &mut Graphics2D) {
        let image = self.images.first().unwrap();
        println!("image size: {:?}", image.size());
        let pattern_size = 256;
        let spritesheet_width = 5;
        let spritesheet_height = 6;
        for x in 0..=spritesheet_width {
            for y in 0..=spritesheet_height {
                let top_left = IVec2::new(x * pattern_size, y * pattern_size).into_f32();
                let bottom_right =
                    IVec2::new((x + 1) * pattern_size, (y + 1) * pattern_size).into_f32();
                let rect = Rectangle::new(top_left, bottom_right);
                let vertex_positions_clockwise = [
                    *rect.top_left(),
                    rect.top_right(),
                    *rect.bottom_right(),
                    rect.bottom_left(),
                ];
                let image_coords_normalized = [
                    Vec2::new(
                        x as f32 / spritesheet_width as f32,
                        y as f32 / spritesheet_height as f32,
                    ),
                    Vec2::new(
                        (x + 1) as f32 / spritesheet_width as f32,
                        y as f32 / spritesheet_height as f32,
                    ),
                    Vec2::new(
                        (x + 1) as f32 / spritesheet_width as f32,
                        (y + 1) as f32 / spritesheet_height as f32,
                    ),
                    Vec2::new(
                        x as f32 / spritesheet_width as f32,
                        (y + 1) as f32 / spritesheet_height as f32,
                    ),
                ];
                let vertex_colors = [Color::WHITE, Color::WHITE, Color::WHITE, Color::WHITE];
                graphics.draw_quad_image_tinted_four_color(
                    vertex_positions_clockwise,
                    vertex_colors,
                    image_coords_normalized,
                    image,
                );
            }
        }
    }
}
