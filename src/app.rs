use speedy2d::{
    color::Color,
    dimen::{UVec2, Vec2},
    window::{
        KeyScancode, ModifiersState, MouseButton, MouseScrollDistance, VirtualKeyCode,
        WindowHandler, WindowHelper, WindowStartupInfo,
    },
    Graphics2D,
};

mod game;
use game::Game;

pub struct App {
    viewport_size: UVec2,

    current_frame: u64,
    mouse: Mouse,
    keyboard: Keyboard,
    is_fullscreen: bool,
    is_inputting_text: bool,

    game: Game,
}

impl App {
    pub fn new(viewport_size: UVec2, config: crate::config::Config) -> Self {
        Self {
            viewport_size,

            current_frame: 0,
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
            is_fullscreen: false,
            is_inputting_text: false,

            game: Game::new(config),
        }
    }

    pub fn game_loop(&mut self, _helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        if self.current_frame == 0 {
            self.setup(graphics);
        }

        self.input();

        self.update();

        self.draw(graphics);
        self.current_frame += 1;
    }

    pub fn setup(&mut self, graphics: &mut Graphics2D) {
        self.game.setup(graphics);
    }

    pub fn input(&mut self) {
        self.game
            .input(self.viewport_size, &self.mouse, &self.keyboard);
    }

    pub fn update(&mut self) {
        self.game.update();
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_gray(0.3));
        self.game.draw(graphics);
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

impl WindowHandler for App {
    fn on_start(&mut self, _helper: &mut WindowHelper<()>, info: WindowStartupInfo) {
        println!("{:?}", info.viewport_size_pixels());
        self.viewport_size = *info.viewport_size_pixels();
    }

    fn on_resize(&mut self, _helper: &mut WindowHelper<()>, size_pixels: UVec2) {
        println!("new size: {size_pixels:?}");
        self.viewport_size = size_pixels;
    }

    fn on_mouse_grab_status_changed(
        &mut self,
        _helper: &mut WindowHelper<()>,
        mouse_grabbed: bool,
    ) {
        if mouse_grabbed {
            println!("Mouse grabbed!");
        } else {
            println!("Mouse ungrabbed!");
        }
        self.mouse.grabbed = mouse_grabbed;
    }

    fn on_fullscreen_status_changed(&mut self, _helper: &mut WindowHelper<()>, fullscreen: bool) {
        if fullscreen {
            println!("App is now in fullscreen!");
        } else {
            println!("App is now windowed!");
        }
        self.is_fullscreen = fullscreen;
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        self.game_loop(helper, graphics);
    }

    fn on_mouse_move(&mut self, _helper: &mut WindowHelper<()>, position: Vec2) {
        self.mouse.position = position;
    }

    fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper<()>, button: MouseButton) {
        self.mouse.press(button);
    }

    fn on_mouse_button_up(&mut self, _helper: &mut WindowHelper<()>, button: MouseButton) {
        self.mouse.release(button);
    }

    fn on_mouse_wheel_scroll(
        &mut self,
        _helper: &mut WindowHelper<()>,
        distance: MouseScrollDistance,
    ) {
        match distance {
            MouseScrollDistance::Lines { x, y, z } => {
                if x != 0.0 {
                    eprintln!("Unsupported input: MouseScroll on X coordinate!");
                }
                if z != 0.0 {
                    eprintln!("Unsupported input: MouseScroll on Z coordinate!");
                }
                if y != 0.0 {
                    self.mouse.scroll_lines += y;
                }
            }
            other => eprintln!("Unsupported input: {other:?}"),
        }
    }

    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        _scancode: KeyScancode,
    ) {
        if let Some(key_code) = virtual_key_code {
            self.keyboard.press(key_code);
        }
    }

    fn on_key_up(
        &mut self,
        _helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        _scancode: KeyScancode,
    ) {
        if let Some(key_code) = virtual_key_code {
            self.keyboard.release(key_code);
        }
    }

    fn on_keyboard_char(&mut self, _helper: &mut WindowHelper<()>, unicode_codepoint: char) {
        if self.is_inputting_text {
            self.keyboard.buffer.push(unicode_codepoint);
        }
    }

    fn on_keyboard_modifiers_changed(
        &mut self,
        _helper: &mut WindowHelper<()>,
        state: ModifiersState,
    ) {
        self.keyboard.modifiers = state;
    }
}

pub struct Keyboard {
    buffer: Vec<char>,
    modifiers: ModifiersState,
    pressed: Vec<VirtualKeyCode>,
}

impl Keyboard {
    fn new() -> Self {
        Self {
            buffer: Vec::new(),
            modifiers: ModifiersState::default(),
            pressed: Vec::new(),
        }
    }

    pub fn press(&mut self, button: VirtualKeyCode) {
        if self.pressed.contains(&button) {
            println!("Pressed {button:?} without releasing it first!");
        } else {
            self.pressed.push(button);
        }
    }

    pub fn release(&mut self, button: VirtualKeyCode) {
        if self.pressed.contains(&button) {
            if let Some(idx) = self.pressed.iter().position(|b| b == &button) {
                self.pressed.remove(idx);
            }
        } else {
            println!("Released {button:?} without it being pressed!");
        }
    }
}

pub struct Mouse {
    position: Vec2,
    grabbed: bool,
    pressed: Vec<MouseButton>,
    scroll_lines: f64,
}

impl Mouse {
    pub const fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            grabbed: false,
            pressed: Vec::new(),
            scroll_lines: 0.0,
        }
    }

    pub fn press(&mut self, button: MouseButton) {
        if self.pressed.contains(&button) {
            println!("Pressed {button:?} without releasing it first!");
        } else {
            self.pressed.push(button);
        }
    }

    pub fn release(&mut self, button: MouseButton) {
        if self.pressed.contains(&button) {
            if let Some(idx) = self.pressed.iter().position(|b| b == &button) {
                self.pressed.remove(idx);
            }
        } else {
            println!("Released {button:?} without it being pressed!");
        }
    }
}
