#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
#![windows_subsystem = "windows"]

use speedy2d::{
    dimen::UVec2,
    window::{WindowCreationOptions, WindowPosition, WindowSize},
    Window,
};

mod app;
use app::App;
mod config;
use config::Config;

mod game;
mod spritesheet;

fn main() {
    let config = Config::new("config.ini");
    let window_size = UVec2::new(config.window_width, config.window_height);
    let window_pixels = WindowSize::PhysicalPixels(window_size);
    let window = Window::new_with_options(
        &config.title,
        WindowCreationOptions::new_windowed(window_pixels, Some(WindowPosition::Center))
            .with_decorations(true)
            .with_transparent(false),
    )
    .expect("Wasn't able to create a window!");
    window.run_loop(App::new(window_size, config));
}
