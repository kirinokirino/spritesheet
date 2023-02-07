use speedy2d::{dimen::UVec2, Graphics2D};

use crate::app::{Keyboard, Mouse};
pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }
    pub fn input(&mut self, viewport_size: UVec2, mouse: &Mouse, keyboard: &Keyboard) {}
    pub fn update(&mut self) {}
    pub fn draw(&self, graphics: &mut Graphics2D) {}
}
