use speedy2d::{dimen::UVec2, Graphics2D};

use crate::app::{Keyboard, Mouse};
use crate::config::Config;
pub struct Game {
    config: Config,
}

impl Game {
    pub const fn new(config: Config) -> Self {
        Self { config }
    }
    pub fn input(&mut self, viewport_size: UVec2, mouse: &Mouse, keyboard: &Keyboard) {}
    pub fn update(&mut self) {}
    pub fn draw(&self, graphics: &mut Graphics2D) {}
}
