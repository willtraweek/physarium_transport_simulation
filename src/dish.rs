use crate::cell::{Cell, Color};

use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::RenderArgs;

///The environment that the cells live in.  Each cell is 1 pixel.  Width and Height are in pixels/cells
pub struct Dish{
    cells: Vec<Vec<Cell>>,
    width: u32,
    height: u32,
    gl: GlGraphics
}

impl Dish {
    pub fn new(width: u32, height: u32, opengl: OpenGL) -> Dish {
        let dish = Dish {
            cells: vec![vec![Cell::new(); height as usize]; width as usize],
            width,
            height,
            gl: GlGraphics::new(opengl)
        };

        dish
    }

    pub fn render(&mut self, &args: &RenderArgs) {
        use graphics::*;
        let width = self.width;
        let height = self.height;
        let mut cells = self.cells.clone();

        self.gl.draw(args.viewport(), |c, gl | {
            clear(crate::cell::map_color(&Color::Black), gl);

            for x in 0..width {
                for y in 0..height {
                    let rect = rectangle::square(x as f64, y as f64, 1.0);
                    let color = cells[x as usize][y as usize].get_color_values();
                    rectangle(color, rect, c.transform, gl);
                }
            }
        });
    }
}