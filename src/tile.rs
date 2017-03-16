use pos::Pos;

use sdl2::pixels::Color;
use sdl2::render::Renderer;
use sdl2::gfx::primitives::DrawRenderer;

pub const INNER_TILE_SIZE: i16 = 14;
pub const TILE_BORDER: i16 = 1;
pub const TILE_SIZE: i16 = INNER_TILE_SIZE + TILE_BORDER * 2;

pub const HIDE_ROWS: i16 = 4;

pub fn draw_tile(renderer: &Renderer, pos: Pos, col: Color) {
    let x = pos.x() as i16;
    let y = pos.y() as i16;

    if y >= HIDE_ROWS {
        let y = y - HIDE_ROWS;
        let _ = renderer.box_(x * TILE_SIZE + TILE_BORDER,
                              y * TILE_SIZE + TILE_BORDER,
                              (x + 1) * TILE_SIZE - TILE_BORDER,
                              (y + 1) * TILE_SIZE - TILE_BORDER,
                              col);
    }
}
