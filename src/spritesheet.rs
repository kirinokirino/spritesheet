use speedy2d::{color::Color, dimen::Vec2, image::ImageHandle, shape::Rectangle, Graphics2D};

//sprite.draw(destination, graphics)

pub struct Spritesheet {
    image_handle: ImageHandle,
    pub width: u32,
    pub height: u32,
}

impl Spritesheet {
    pub const fn new(image_handle: ImageHandle, width: u32, height: u32) -> Self {
        Self {
            image_handle,
            width,
            height,
        }
    }

    pub fn draw_sprite(
        &self,
        dest: &Rectangle,
        sprite_x: u32,
        sprite_y: u32,
        graphics: &mut Graphics2D,
    ) {
        draw_sprite(
            dest,
            &self.image_handle,
            sprite_x,
            sprite_y,
            self.width,
            self.height,
            graphics,
        );
    }
}

pub fn draw_sprite(
    destination: &Rectangle,
    spritesheet: &ImageHandle,
    sprite_x: u32,
    sprite_y: u32,
    spritesheet_width: u32,
    spritesheet_height: u32,
    graphics: &mut Graphics2D,
) {
    let vertex_positions_clockwise = [
        *destination.top_left(),
        destination.top_right(),
        *destination.bottom_right(),
        destination.bottom_left(),
    ];
    let image_coords_normalized = [
        Vec2::new(
            sprite_x as f32 / spritesheet_width as f32,
            sprite_y as f32 / spritesheet_height as f32,
        ),
        Vec2::new(
            (sprite_x + 1) as f32 / spritesheet_width as f32,
            sprite_y as f32 / spritesheet_height as f32,
        ),
        Vec2::new(
            (sprite_x + 1) as f32 / spritesheet_width as f32,
            (sprite_y + 1) as f32 / spritesheet_height as f32,
        ),
        Vec2::new(
            sprite_x as f32 / spritesheet_width as f32,
            (sprite_y + 1) as f32 / spritesheet_height as f32,
        ),
    ];
    let vertex_colors = [Color::WHITE, Color::WHITE, Color::WHITE, Color::WHITE];
    graphics.draw_quad_image_tinted_four_color(
        vertex_positions_clockwise,
        vertex_colors,
        image_coords_normalized,
        spritesheet,
    );
}
