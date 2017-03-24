use pos::Pos;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

const INNER_BLOCK_SIZE: u8 = 14;
const BLOCK_BORDER: u8 = 1;
pub const BLOCK_SIZE: u8 = INNER_BLOCK_SIZE + BLOCK_BORDER * 2;

const BORDER_COLOR: Color = Color::RGB(100, 100, 100);

pub fn draw_block(renderer: &mut Renderer, pos: Pos, col: Color) {
    let x = pos.x() as i16;
    let y = pos.y() as i16;

    renderer.set_draw_color(col);
    let _ = renderer.fill_rect(Rect::new(x as i32 * BLOCK_SIZE as i32 + BLOCK_BORDER as i32,
                                         y as i32 * BLOCK_SIZE as i32 + BLOCK_BORDER as i32,
                                         BLOCK_SIZE as u32 - BLOCK_BORDER as u32,
                                         BLOCK_SIZE as u32 - BLOCK_BORDER as u32));
}

pub fn draw_border(renderer: &mut Renderer, size: Pos) {
    let size = size + Pos::new(1, 1);

    for y in 0..size.y() + 1 {
        draw_block(renderer, Pos::new(0, y), BORDER_COLOR);
        draw_block(renderer, Pos::new(size.x(), y), BORDER_COLOR);
    }

    for x in 1..size.x() {
        draw_block(renderer, Pos::new(x, size.y()), BORDER_COLOR);
        draw_block(renderer, Pos::new(x, 0), BORDER_COLOR);
    }
}
