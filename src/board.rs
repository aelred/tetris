use tile::draw_tile;
use pos::Pos;

use sdl2::pixels::Color;
use sdl2::render::Renderer;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 24;

pub struct Board([[Option<Color>; WIDTH]; HEIGHT]);

impl Board {
    pub fn new() -> Board {
        Board([[None; WIDTH]; HEIGHT])
    }

    pub fn touches(&self, pos: Pos) -> bool {
        pos.y >= 0 &&
        (pos.x < 0 || pos.x >= WIDTH as isize || pos.y >= HEIGHT as isize ||
         self.0[pos.y as usize][pos.x as usize].is_some())
    }

    pub fn fill(&mut self, pos: Pos, color: Color) {
        self.0[pos.y as usize][pos.x as usize] = Some(color);
    }

    pub fn check_clear(&mut self) {
        for y in 0..HEIGHT {
            let mut clear = true;

            'check_clear: for cell in self.0[y].iter() {
                if cell.is_none() {
                    clear = false;
                    break 'check_clear;
                }
            }

            if clear {
                for yy in (1..y + 1).rev() {
                    self.0[yy] = self.0[yy - 1];
                }

                for x in 0..WIDTH {
                    self.0[0][x] = None;
                }
            }
        }
    }

    pub fn draw(&self, renderer: &Renderer) {
        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match *cell {
                    Some(color) => {
                        draw_tile(&renderer,
                                  Pos {
                                      x: x as isize,
                                      y: y as isize,
                                  },
                                  color)
                    }
                    None => (),
                }
            }
        }
    }
}
