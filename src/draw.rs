use pos::Pos;
use tetromino::TetColor;

use sdl2::pixels::Color;
use sdl2::pixels::Color::RGB;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::render::Renderer;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;
use game_over::GameOver;
use score::OFFSET;
use game_over::HighScores;
use board::Board;
use board::HIDE_ROWS;
use board::HEIGHT;
use board::WIDTH;
use tetromino::Rotation;
use tetromino;
use tetromino::Tetromino;
use game::Game;
use board;
use piece::Piece;
use score::Score;

const INNER_BLOCK_SIZE: u8 = 22;
const BLOCK_BORDER: u8 = 1;
pub const BLOCK_SIZE: u8 = INNER_BLOCK_SIZE + BLOCK_BORDER * 2;

const BORDER_COLOR: Color = Color::RGB(100, 100, 100);

pub struct Drawer<'a> {
    renderer: Renderer<'a>,
    font: Font<'a, 'a>,
}

impl Into<Color> for TetColor {
    fn into(self) -> Color {
        match self {
            TetColor::O => RGB(255, 255, 0),
            TetColor::I => RGB(0, 255, 255),
            TetColor::J => RGB(0, 0, 255),
            TetColor::L => RGB(255, 165, 0),
            TetColor::S => RGB(0, 255, 0),
            TetColor::T => RGB(255, 0, 255),
            TetColor::Z => RGB(255, 0, 0),
        }
    }
}

impl<'a> Drawer<'a> {
    pub fn new(renderer: Renderer<'a>, font: Font<'a, 'a>) -> Self {
        Drawer { renderer, font }
    }

    pub fn draw_block<T: Into<Color>>(&mut self, pos: Pos, col: T) {
        let x = pos.x() as i16;
        let y = pos.y() as i16;

        self.renderer.set_draw_color(col.into());
        let _ = self.renderer.fill_rect(Rect::new(
            i32::from(x) * i32::from(BLOCK_SIZE) +
                i32::from(BLOCK_BORDER),
            i32::from(y) * i32::from(BLOCK_SIZE) +
                i32::from(BLOCK_BORDER),
            u32::from(BLOCK_SIZE) - u32::from(BLOCK_BORDER),
            u32::from(BLOCK_SIZE) - u32::from(BLOCK_BORDER),
        ));
    }

    pub fn draw_game_over(&mut self, game_over: &GameOver) {
        let mut text = self.text()
            .top()
            .offset(0, 50)
            .size(3)
            .draw("Game Over")
            .under()
            .offset(0, 10)
            .size(1)
            .draw("final score")
            .under()
            .size(3)
            .draw(&game_over.score.value.to_string());

        text = game_over.draw(text);

        if game_over.posting_hiscore() {
            text.size(1).draw("[ Enter Name and Press Enter ]");
        } else {
            text.size(1).draw("[ Press Enter ]");
        }
    }

    pub fn draw_board(&mut self, board: &Board) {
        self.set_viewport(*BOARD_BORDER_VIEW);
        self.draw_border(Pos::new(i16::from(WIDTH), i16::from(HEIGHT - HIDE_ROWS)));

        self.set_viewport(*BOARD_VIEW);

        for y in HIDE_ROWS..HEIGHT {
            for x in 0..WIDTH {
                if let Some(color) = board.grid[y as usize][x as usize] {
                    let y = y - HIDE_ROWS;
                    let cell_pos = Pos::new(i16::from(x), i16::from(y));
                    self.draw_block(cell_pos, color)
                }
            }
        }
    }

    pub fn draw_next(&mut self, next: &Tetromino) {
        self.set_viewport(*PREVIEW_VIEW);

        self.draw_border(Pos::new(
            i16::from(tetromino::WIDTH),
            i16::from(tetromino::HEIGHT),
        ));
        self.draw_tetromino(next, Rotation::default(), Pos::new(1, 1));
    }

    pub fn draw_game_score(&mut self, game: &Game) {
        self.set_viewport(*SCORE_VIEW);

        self.text()
            .draw("lines")
            .size(2)
            .left()
            .draw(&game.lines_cleared.to_string())
            .size(1)
            .left()
            .offset(0, PAD)
            .draw("score")
            .size(2)
            .left()
            .draw(&game.score.to_string());
    }

    pub fn draw_piece(&mut self, piece: &Piece) {
        self.draw_tetromino(
            piece.tetromino,
            piece.rot,
            piece.pos + Pos::new(0, -i16::from(HIDE_ROWS)),
        );
    }

    fn draw_tetromino(&mut self, tetromino: &Tetromino, rot: Rotation, pos: Pos) {
        for block in tetromino.blocks(rot) {
            self.draw_block(pos + block, tetromino.color);
        }
    }

    fn draw_border(&mut self, size: Pos) {
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
        self.last_rect = self.drawer.draw_text(
            &self.pos,
            text,
            self.size,
            self.color,
        );
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
                Rect::new(
                    rect.center().x() - width as i32 / 2,
                    rect.y(),
                    width,
                    height,
                )
            }
            TextPos::Under(rect) => {
                Rect::new(
                    rect.center().x() - width as i32 / 2,
                    rect.bottom(),
                    width,
                    height,
                )
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

impl Score {
    pub fn draw <'a, 'b> (&self, text: TextDrawer <'a, 'b>) -> TextDrawer <'a, 'b> {
        let name = if self.name.is_empty() {
            " "
        } else {
            & self.name
        };

        text.offset( - OFFSET, 0)
            .draw(name)
            .offset(OFFSET * 2, 0)
            .draw( & self.value.to_string())
            .under()
            .offset( - OFFSET, 10)
    }
}

impl GameOver {
    fn draw<'a, 'b>(&self, text: TextDrawer<'a, 'b>) -> TextDrawer<'a, 'b> {
        match self.hiscores {
            Some(HighScores {
                     ref higher_scores,
                     ref lower_scores,
                     ref has_hiscore,
                 }) => {
                let mut text = text.size(3).under().offset(0, 10).draw("High Scores");

                text = text.size(2).under().offset(0, 10);

                for score in higher_scores {
                    text = score.draw(text);
                }

                if *has_hiscore {
                    text = self
                        .score
                        .draw(text.color(Color::RGB(255, 255, 100)))
                        .reset_color();
                }

                for score in lower_scores {
                    text = score.draw(text);
                }

                text.under().offset(-OFFSET, 10)
            }
            None => {
                text.size(1)
                    .under()
                    .offset(0, 10)
                    .draw("[ ERROR Failed to retrieve High Scores ]")
                    .offset(0, 20)
            }
        }
    }
}


lazy_static! {
    static ref PREVIEW_VIEW: Rect = Rect::new(PREVIEW_X, PREVIEW_Y, PREVIEW_WIDTH, PREVIEW_HEIGHT);

    static ref SCORE_VIEW: Rect = Rect::new(SCORE_X, PAD, PREVIEW_WIDTH, BOARD_HEIGHT);

    static ref BOARD_BORDER_VIEW: Rect = Rect::new(0,
                                                   0,
                                                   BOARD_WIDTH + BOARD_BORDER * 2,
                                                   BOARD_HEIGHT + BOARD_BORDER * 2);

    static ref BOARD_VIEW: Rect = Rect::new(BOARD_BORDER as i32,
                                            BOARD_BORDER as i32,
                                            BOARD_WIDTH,
                                            BOARD_HEIGHT);
}

const BOARD_BORDER: u32 = BLOCK_SIZE as u32;
const BOARD_WIDTH: u32 = board::WIDTH as u32 * BLOCK_SIZE as u32;
const BOARD_HEIGHT: u32 = (board::HEIGHT as u32 - HIDE_ROWS as u32) * BLOCK_SIZE as u32;
const TOTAL_BOARD_HEIGHT: u32 = BOARD_HEIGHT + BOARD_BORDER * 2;

const PREVIEW_X: i32 = BOARD_WIDTH as i32 + BOARD_BORDER as i32;
const PREVIEW_Y: i32 = TOTAL_BOARD_HEIGHT as i32 -
    (tetromino::HEIGHT + 2) as i32 * BLOCK_SIZE as i32;
const PREVIEW_WIDTH: u32 = (tetromino::WIDTH + 2) as u32 * BLOCK_SIZE as u32;
const PREVIEW_HEIGHT: u32 = (tetromino::HEIGHT + 2) as u32 * BLOCK_SIZE as u32;

const SCORE_X: i32 = PREVIEW_X + BOARD_BORDER as i32 + PAD;

const PAD: i32 = BLOCK_SIZE as i32;

pub const WINDOW_WIDTH: u32 = BOARD_WIDTH + BOARD_BORDER + PREVIEW_WIDTH;
pub const WINDOW_HEIGHT: u32 = TOTAL_BOARD_HEIGHT;
pub const WINDOW_RATIO: f32 = WINDOW_HEIGHT as f32 / WINDOW_WIDTH as f32;
