use pos::Pos;
use game::WINDOW_HEIGHT;
use game::WINDOW_WIDTH;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::render::Renderer;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;

const INNER_BLOCK_SIZE: u8 = 22;
const BLOCK_BORDER: u8 = 1;
pub const BLOCK_SIZE: u8 = INNER_BLOCK_SIZE + BLOCK_BORDER * 2;

const BORDER_COLOR: Color = Color::RGB(100, 100, 100);

pub struct Drawer<'a> {
    renderer: Renderer<'a>,
    font: Font<'a, 'a>,
}

impl<'a> Drawer<'a> {
    pub fn new<'b>(renderer: Renderer<'b>, font: Font<'b, 'b>) -> Drawer<'b> {
        Drawer {
            renderer: renderer,
            font: font,
        }
    }

    pub fn draw_block(&mut self, pos: Pos, col: Color) {
        let x = pos.x() as i16;
        let y = pos.y() as i16;

        self.renderer.set_draw_color(col);
        let _ =
            self.renderer.fill_rect(Rect::new(x as i32 * BLOCK_SIZE as i32 + BLOCK_BORDER as i32,
                                              y as i32 * BLOCK_SIZE as i32 + BLOCK_BORDER as i32,
                                              BLOCK_SIZE as u32 - BLOCK_BORDER as u32,
                                              BLOCK_SIZE as u32 - BLOCK_BORDER as u32));
    }

    pub fn draw_border(&mut self, size: Pos) {
        let size = size + Pos::new(1, 1);

        for y in 0..size.y() + 1 {
            self.draw_block(Pos::new(0, y), BORDER_COLOR);
            self.draw_block(Pos::new(size.x(), y), BORDER_COLOR);
        }

        for x in 1..size.x() {
            self.draw_block(Pos::new(x, size.y()), BORDER_COLOR);
            self.draw_block(Pos::new(x, 0), BORDER_COLOR);
        }
    }

    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, size: u32) -> Rect {
        self.draw_text_general(|w, h| Rect::new(x, y, w, h), text, size)
    }

    pub fn draw_text_centered(&mut self,
                              text: &str,
                              offset_x: i32,
                              offset_y: i32,
                              size: u32)
                              -> Rect {
        self.draw_text_general(|w, h| {
                                   let mut target = center_view(w, h);
                                   target.offset(offset_x, offset_y);
                                   target
                               },
                               text,
                               size)
    }

    pub fn draw_text_under(&mut self, text: &str, rect: &Rect, pad: u32, size: u32) -> Rect {
        self.draw_text_general(|w, h| {
                                   Rect::new(rect.center().x() - w as i32 / 2,
                                             rect.bottom() + pad as i32,
                                             w,
                                             h)
                               },
                               text,
                               size)
    }

    pub fn draw_text_general<F>(&mut self, func: F, text: &str, size: u32) -> Rect
        where F: Fn(u32, u32) -> Rect
    {
        let surface = self.font
            .render(text)
            .solid(Color::RGB(255, 255, 255))
            .unwrap();
        let texture = self.renderer.create_texture_from_surface(&surface).unwrap();

        let TextureQuery { width, height, .. } = texture.query();

        let target = func(width * size, height * size);

        self.renderer.copy(&texture, None, Some(target)).unwrap();

        target
    }

    pub fn set_viewport(&mut self, rect: Rect) {
        self.renderer.set_viewport(Some(rect));
    }

    pub fn clear(&mut self) {
        self.renderer.set_viewport(None);
        self.renderer.set_draw_color(Color::RGB(32, 48, 32));
        self.renderer.clear();
    }

    pub fn present(&mut self) {
        self.renderer.present();
    }
}

fn center_view(width: u32, height: u32) -> Rect {
    let center_x = (WINDOW_WIDTH / 2) as i32;
    let center_y = (WINDOW_HEIGHT / 2) as i32;

    Rect::from_center(Point::new(center_x, center_y), width, height)
}
