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
    pub fn new(renderer: Renderer<'a>, font: Font<'a, 'a>) -> Self {
        Drawer {
            renderer: renderer,
            font: font,
        }
    }

    pub fn draw_block(&mut self, pos: Pos, col: Color) {
        let x = pos.x() as i16;
        let y = pos.y() as i16;

        self.renderer.set_draw_color(col);
        let _ = self.renderer
            .fill_rect(Rect::new(x as i32 * BLOCK_SIZE as i32 + BLOCK_BORDER as i32,
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

    fn draw_text(&mut self, text_pos: &TextPos, text: &str, size: u32, color: Color) -> Rect {
        let surface = self.font.render(text).solid(color).unwrap();
        let texture = self.renderer.create_texture_from_surface(&surface).unwrap();

        let TextureQuery { width, height, .. } = texture.query();

        let target = text_pos.apply(width * size, height * size);

        self.renderer.copy(&texture, None, Some(target)).unwrap();

        target
    }

    pub fn text<'b>(&'b mut self) -> TextDrawer<'b, 'a> {
        TextDrawer {
            last_rect: Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT),
            pos: TextPos::At(0, 0),
            size: 1,
            color: Color::RGB(255, 255, 255),
            drawer: self,
        }
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

pub struct TextDrawer<'a, 'b: 'a> {
    last_rect: Rect,
    pos: TextPos,
    size: u32,
    color: Color,
    drawer: &'a mut Drawer<'b>,
}

impl<'a, 'b: 'a> TextDrawer<'a, 'b> {
    pub fn draw(mut self, text: &str) -> Self {
        self.last_rect = self.drawer
            .draw_text(&self.pos, text, self.size, self.color);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn reset_color(mut self) -> Self {
        self.color = Color::RGB(255, 255, 255);
        self
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }

    pub fn top(mut self) -> Self {
        self.pos = TextPos::Top(self.last_rect);
        self
    }

    pub fn under(mut self) -> Self {
        self.pos = TextPos::Under(self.last_rect);
        self
    }

    pub fn left(mut self) -> Self {
        self.pos = TextPos::Left(self.last_rect);
        self
    }

    pub fn centered(mut self) -> Self {
        self.pos = TextPos::Centered;
        self
    }

    pub fn offset(mut self, x: i32, y: i32) -> Self {
        self.pos = TextPos::Offset(Box::new(self.pos), x, y);
        self
    }
}

enum TextPos {
    At(i32, i32),
    Centered,
    Top(Rect),
    Under(Rect),
    Left(Rect),
    Offset(Box<TextPos>, i32, i32),
}

impl TextPos {
    fn apply(&self, width: u32, height: u32) -> Rect {
        match *self {
            TextPos::At(x, y) => Rect::new(x, y, width, height),
            TextPos::Centered => {
                let center_x = (WINDOW_WIDTH / 2) as i32;
                let center_y = (WINDOW_HEIGHT / 2) as i32;
                Rect::from_center(Point::new(center_x, center_y), width, height)
            }
            TextPos::Top(rect) => {
                Rect::new(rect.center().x() - width as i32 / 2,
                          rect.y(),
                          width,
                          height)
            }
            TextPos::Under(rect) => {
                Rect::new(rect.center().x() - width as i32 / 2,
                          rect.bottom(),
                          width,
                          height)
            }
            TextPos::Left(rect) => Rect::new(rect.x(), rect.bottom(), width, height),
            TextPos::Offset(ref pos, x, y) => {
                let mut rect = pos.apply(width, height);
                rect.offset(x, y);
                rect
            }
        }
    }
}
