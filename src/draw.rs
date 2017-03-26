use pos::Pos;
use game::WINDOW_HEIGHT;
use game::WINDOW_WIDTH;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;

const INNER_BLOCK_SIZE: u8 = 22;
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

pub fn draw_text(text: &str,
                 x: i32,
                 y: i32,
                 size: u32,
                 renderer: &mut Renderer,
                 font: &Font)
                 -> Rect {
    let surface = font.render(text).solid(Color::RGB(255, 255, 255)).unwrap();
    let texture = renderer.create_texture_from_surface(&surface).unwrap();

    let TextureQuery { width, height, .. } = texture.query();

    let target = Rect::new(x, y, width * size, height * size);

    renderer.copy(&texture, None, Some(target)).unwrap();

    target
}

pub fn draw_text_centered(text: &str,
                          offset_x: i32,
                          offset_y: i32,
                          size: u32,
                          renderer: &mut Renderer,
                          font: &Font)
                          -> Rect {
    let surface = font.render(text).solid(Color::RGB(255, 255, 255)).unwrap();
    let texture = renderer.create_texture_from_surface(&surface).unwrap();

    let TextureQuery { width, height, .. } = texture.query();

    let target = center_view(offset_x, offset_y, width * size, height * size);

    renderer.copy(&texture, None, Some(target)).unwrap();

    target
}

fn center_view(x: i32, y: i32, width: u32, height: u32) -> Rect {
    let center_x = (WINDOW_WIDTH / 2) as i32;
    let center_y = (WINDOW_HEIGHT / 2) as i32;

    Rect::new(center_x + x - width as i32 / 2,
              center_y + y - height as i32 / 2,
              width,
              height)
}
