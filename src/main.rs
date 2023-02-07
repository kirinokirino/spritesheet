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

fn main() {
    let window_size = UVec2::new(640, 640);
    let window_pixels = WindowSize::PhysicalPixels(window_size);
    let window = Window::new_with_options(
        "FLOATING",
        WindowCreationOptions::new_windowed(window_pixels, Some(WindowPosition::Center))
            .with_decorations(true)
            .with_transparent(false),
    )
    .expect("Wasn't able to create a window!");
    window.run_loop(App::new(window_size));
}
