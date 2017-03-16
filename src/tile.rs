use pos::Pos;

use sdl2::pixels::Color;
use sdl2::render::Renderer;
use sdl2::gfx::primitives::DrawRenderer;

pub const INNER_TILE_SIZE: i16 = 14;
pub const TILE_BORDER: i16 = 1;
pub const TILE_SIZE: i16 = INNER_TILE_SIZE + TILE_BORDER * 2;

pub const HIDE_ROWS: u8 = 4;

const BORDER_COLOR: Color = Color::RGB(100, 100, 100);

pub fn draw_tile(renderer: &Renderer, pos: Pos, col: Color) {
    let x = pos.x() as i16;
    let y = pos.y() as i16;

    if y >= HIDE_ROWS as i16 {
        let y = y - HIDE_ROWS as i16;
        let _ = renderer.box_(x * TILE_SIZE + TILE_BORDER,
                              y * TILE_SIZE + TILE_BORDER,
                              (x + 1) * TILE_SIZE - TILE_BORDER,
                              (y + 1) * TILE_SIZE - TILE_BORDER,
                              col);
    }
}

pub fn draw_border(renderer: &Renderer, top_left: Pos, bottom_right: Pos) {
    for y in top_left.y()..bottom_right.y() {
        draw_tile(renderer, Pos::new(top_left.x() - 1, y), BORDER_COLOR);
        draw_tile(renderer, Pos::new(bottom_right.x(), y), BORDER_COLOR);
    }

    for x in top_left.x() - 1..bottom_right.x() + 1 {
        draw_tile(renderer, Pos::new(x, bottom_right.y()), BORDER_COLOR);
        draw_tile(renderer, Pos::new(x, top_left.y() - 1), BORDER_COLOR);
    }
}
