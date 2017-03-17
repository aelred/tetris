use pos::Pos;

use sdl2::pixels::Color;
use sdl2::render::Renderer;
use sdl2::gfx::primitives::DrawRenderer;

pub const INNER_TILE_SIZE: u8 = 14;
pub const TILE_BORDER: u8 = 1;
pub const TILE_SIZE: u8 = INNER_TILE_SIZE + TILE_BORDER * 2;

const BORDER_COLOR: Color = Color::RGB(100, 100, 100);

pub fn draw_tile(renderer: &Renderer, pos: Pos, col: Color) {
    let x = pos.x() as i16;
    let y = pos.y() as i16;

    let _ = renderer.box_(x * TILE_SIZE as i16 + TILE_BORDER as i16,
                          y * TILE_SIZE as i16 + TILE_BORDER as i16,
                          (x + 1) * TILE_SIZE as i16 - TILE_BORDER as i16,
                          (y + 1) * TILE_SIZE as i16 - TILE_BORDER as i16,
                          col);
}

pub fn draw_border(renderer: &Renderer, size: Pos) {
    let size = size + Pos::new(1, 1);

    for y in 0..size.y() + 1 {
        draw_tile(renderer, Pos::new(0, y), BORDER_COLOR);
        draw_tile(renderer, Pos::new(size.x(), y), BORDER_COLOR);
    }

    for x in 1..size.x() {
        draw_tile(renderer, Pos::new(x, size.y()), BORDER_COLOR);
        draw_tile(renderer, Pos::new(x, 0), BORDER_COLOR);
    }
}
