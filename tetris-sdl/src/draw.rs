use std::i16;
use std::i32;

use lazy_static::lazy_static;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;
use sdl2::video::Window;

use tetris::Board;
use tetris::Game;
use tetris::GameOver;
use tetris::HighScores;
use tetris::Piece;
use tetris::Pos;
use tetris::Rotation;
use tetris::Score;
use tetris::Shape;
use tetris::ShapeColor;
use tetris::State;

const INNER_BLOCK_SIZE: u8 = 22;
const BLOCK_BORDER: u8 = 1;
pub const BLOCK_SIZE: u8 = INNER_BLOCK_SIZE + BLOCK_BORDER * 2;
pub const SCORE_OFFSET: i32 = 100;

const BORDER_COLOR: Color = Color {
    r: 100,
    g: 100,
    b: 100,
    a: 255,
};

pub struct Drawer<'a> {
    canvas: Canvas<Window>,
    font: Font<'a, 'a>,
}

fn shape_color_to_rgb(color: ShapeColor) -> Color {
    match color {
        ShapeColor::O => Color::RGB(255, 255, 0),
        ShapeColor::I => Color::RGB(0, 255, 255),
        ShapeColor::J => Color::RGB(0, 0, 255),
        ShapeColor::L => Color::RGB(255, 165, 0),
        ShapeColor::S => Color::RGB(0, 255, 0),
        ShapeColor::T => Color::RGB(255, 0, 255),
        ShapeColor::Z => Color::RGB(255, 0, 0),
    }
}

impl<'a> Drawer<'a> {
    pub fn new(canvas: Canvas<Window>, font: Font<'a, 'a>) -> Self {
        Drawer { canvas, font }
    }

    pub fn draw_state(&mut self, state: &State) {
        match state {
            State::Title(_) => self.title_draw(),
            State::Play(game) => self.draw_game(game),
            State::Paused(_) => self.pause_draw(),
            State::GameOver(game_over) => self.draw_game_over(game_over),
        }
    }

    fn title_draw(&mut self) {
        self.text()
            .size(4)
            .centered()
            .draw("Tetris")
            .size(1)
            .under()
            .offset(0, 10)
            .draw("[ Press Enter ]");
    }

    fn pause_draw(&mut self) {
        self.text().centered().draw("Paused");
    }

    pub fn draw_block(&mut self, pos: Pos, col: Color) {
        let x = pos.x() as i16;
        let y = pos.y() as i16;

        self.canvas.set_draw_color(col);
        let _ = self.canvas.fill_rect(Rect::new(
            i32::from(x) * i32::from(BLOCK_SIZE) + i32::from(BLOCK_BORDER),
            i32::from(y) * i32::from(BLOCK_SIZE) + i32::from(BLOCK_BORDER),
            u32::from(BLOCK_SIZE) - u32::from(BLOCK_BORDER),
            u32::from(BLOCK_SIZE) - u32::from(BLOCK_BORDER),
        ));
    }

    pub fn draw_game_over(&mut self, game_over: &GameOver) {
        let mut text = self
            .text()
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

    pub fn draw_game(&mut self, game: &Game) {
        self.draw_board(game.board());
        self.draw_piece(game.piece());
        self.draw_next(game.next_shape());
        self.draw_game_score(game);
    }

    fn draw_board(&mut self, board: &Board) {
        self.set_viewport(*BOARD_BORDER_VIEW);
        self.draw_border(Pos::new(
            i16::from(Board::WIDTH),
            i16::from(Board::VISIBLE_ROWS),
        ));

        self.set_viewport(*BOARD_VIEW);

        for y in Board::HIDE_ROWS..Board::HEIGHT {
            for x in 0..Board::WIDTH {
                if let Some(color) = board.grid()[y as usize][x as usize] {
                    let y = y - Board::HIDE_ROWS;
                    let cell_pos = Pos::new(i16::from(x), i16::from(y));
                    self.draw_block(cell_pos, shape_color_to_rgb(color))
                }
            }
        }
    }

    fn draw_next(&mut self, next: Shape) {
        self.set_viewport(*PREVIEW_VIEW);

        self.draw_border(Pos::new(i16::from(Shape::WIDTH), i16::from(Shape::HEIGHT)));
        self.draw_shape(next, Rotation::default(), Pos::new(1, 1));
    }

    fn draw_game_score(&mut self, game: &Game) {
        self.set_viewport(*SCORE_VIEW);

        self.text()
            .draw("lines")
            .size(2)
            .left()
            .draw(&game.lines_cleared().to_string())
            .size(1)
            .left()
            .offset(0, PAD)
            .draw("score")
            .size(2)
            .left()
            .draw(&game.score().to_string());
    }

    fn draw_piece(&mut self, piece: &Piece) {
        self.draw_shape(
            piece.shape,
            piece.rot,
            piece.pos + Pos::new(0, -i16::from(Board::HIDE_ROWS)),
        );
    }

    fn draw_shape(&mut self, shape: Shape, rot: Rotation, pos: Pos) {
        for block in shape.blocks(rot) {
            self.draw_block(pos + block, shape_color_to_rgb(shape.color));
        }
    }

    fn draw_border(&mut self, size: Pos) {
        let size = size + Pos::new(1, 1);

        for y in 0..=size.y() {
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
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let TextureQuery { width, height, .. } = texture.query();

        let target = text_pos.apply(width * size, height * size);

        self.canvas.copy(&texture, None, Some(target)).unwrap();

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
        self.canvas.set_viewport(Some(rect));
    }

    pub fn clear(&mut self) {
        self.canvas.set_viewport(None);
        self.canvas.set_draw_color(Color::RGB(32, 48, 32));
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}

pub struct TextDrawer<'a, 'b: 'a> {
    last_rect: Rect,
    pos: TextPos,
    size: u32,
    color: Color,
    drawer: &'a mut Drawer<'b>,
}

impl TextDrawer<'_, '_> {
    pub fn draw(mut self, text: &str) -> Self {
        self.last_rect = self
            .drawer
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
            TextPos::Top(rect) => Rect::new(
                rect.center().x() - width as i32 / 2,
                rect.y(),
                width,
                height,
            ),
            TextPos::Under(rect) => Rect::new(
                rect.center().x() - width as i32 / 2,
                rect.bottom(),
                width,
                height,
            ),
            TextPos::Left(rect) => Rect::new(rect.x(), rect.bottom(), width, height),
            TextPos::Offset(ref pos, x, y) => {
                let mut rect = pos.apply(width, height);
                rect.offset(x, y);
                rect
            }
        }
    }
}

trait Drawable {
    fn draw<'a, 'b>(&self, text: TextDrawer<'a, 'b>) -> TextDrawer<'a, 'b>;
}

impl Drawable for Score {
    fn draw<'a, 'b>(&self, text: TextDrawer<'a, 'b>) -> TextDrawer<'a, 'b> {
        let name = if self.name.is_empty() {
            " "
        } else {
            &self.name
        };

        text.offset(-SCORE_OFFSET, 0)
            .draw(name)
            .offset(SCORE_OFFSET * 2, 0)
            .draw(&self.value.to_string())
            .under()
            .offset(-SCORE_OFFSET, 10)
    }
}

impl Drawable for GameOver {
    fn draw<'a, 'b>(&self, text: TextDrawer<'a, 'b>) -> TextDrawer<'a, 'b> {
        match &self.hiscores {
            Some(HighScores {
                higher_scores,
                lower_scores,
                has_hiscore,
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

                text.under().offset(-SCORE_OFFSET, 10)
            }
            None => text
                .size(1)
                .under()
                .offset(0, 10)
                .draw("[ ERROR Failed to retrieve High Scores ]")
                .offset(0, 20),
        }
    }
}

lazy_static! {
    static ref PREVIEW_VIEW: Rect = Rect::new(PREVIEW_X, PREVIEW_Y, PREVIEW_WIDTH, PREVIEW_HEIGHT);
    static ref SCORE_VIEW: Rect = Rect::new(SCORE_X, PAD, PREVIEW_WIDTH, BOARD_HEIGHT);
    static ref BOARD_BORDER_VIEW: Rect = Rect::new(
        0,
        0,
        BOARD_WIDTH + BOARD_BORDER * 2,
        BOARD_HEIGHT + BOARD_BORDER * 2
    );
    static ref BOARD_VIEW: Rect = Rect::new(
        BOARD_BORDER as i32,
        BOARD_BORDER as i32,
        BOARD_WIDTH,
        BOARD_HEIGHT
    );
}

const BOARD_BORDER: u32 = BLOCK_SIZE as u32;
const BOARD_WIDTH: u32 = Board::WIDTH as u32 * BLOCK_SIZE as u32;
const BOARD_HEIGHT: u32 = Board::VISIBLE_ROWS as u32 * BLOCK_SIZE as u32;
const TOTAL_BOARD_HEIGHT: u32 = BOARD_HEIGHT + BOARD_BORDER * 2;

const PREVIEW_X: i32 = BOARD_WIDTH as i32 + BOARD_BORDER as i32;
const PREVIEW_Y: i32 = TOTAL_BOARD_HEIGHT as i32 - (Shape::HEIGHT + 2) as i32 * BLOCK_SIZE as i32;
const PREVIEW_WIDTH: u32 = (Shape::WIDTH + 2) as u32 * BLOCK_SIZE as u32;
const PREVIEW_HEIGHT: u32 = (Shape::HEIGHT + 2) as u32 * BLOCK_SIZE as u32;

const SCORE_X: i32 = PREVIEW_X + BOARD_BORDER as i32 + PAD;

const PAD: i32 = BLOCK_SIZE as i32;

pub const WINDOW_WIDTH: u32 = BOARD_WIDTH + BOARD_BORDER + PREVIEW_WIDTH;
pub const WINDOW_HEIGHT: u32 = TOTAL_BOARD_HEIGHT;
pub const WINDOW_RATIO: f32 = WINDOW_HEIGHT as f32 / WINDOW_WIDTH as f32;
