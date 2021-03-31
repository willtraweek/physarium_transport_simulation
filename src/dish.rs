use crate::cell::{Cell, Color};
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

///The environment that the cells live in.  Each cell is 1 pixel.  Width and Height are in pixels/cells
pub struct Dish{
    cells: Vec<Vec<Cell>>,
    width: u64,
    height: u64,
    gl: GlGraphics
}

impl Dish {
    pub fn new(width: u64, height: u64) -> Dish {
        Dish {
            cells: vec![vec![Cell::new(); width as usize]; height as usize],
            width,
            height
        }
    }

    pub fn render(&mut self, &args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl | {
            clear(Color::Black, gl);

            for x in 0..width {
                for y in 0..height {

                }
            }
        });
    }
}