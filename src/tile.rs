use pos::Pos;

use sdl2::pixels::Color;
use sdl2::render::Renderer;
use sdl2::gfx::primitives::DrawRenderer;

pub const TILE_SIZE: i16 = 24;

pub fn draw_tile(renderer: &Renderer, pos: Pos, col: Color) {
    let x = pos.x() as i16;
    let y = pos.y() as i16;
    let _ = renderer.box_(x * TILE_SIZE + 1,
                          y * TILE_SIZE + 1,
                          (x + 1) * TILE_SIZE - 1,
                          (y + 1) * TILE_SIZE - 1,
                          col);
}
