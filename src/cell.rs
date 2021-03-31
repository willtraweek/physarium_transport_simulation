pub enum Color {
    Red,
    Green,
    Blue,
    White,
    Black
}

pub struct Cell {
    color: Color,
    color_values: [f64;4]
}

///The basic building block of the organism
impl Cell {
    ///Inits a new cell to black.  This helps when creating the first screen
    pub fn new() -> Cell {
        Cell {
            color: Color::Black,
            color_values: [0.0, 0.0, 0.0, 0.0]
        }
    }

    ///Each color will seperate the cells in case of multiple colonies
    pub fn assign_color_values(&mut self) {
        self.color_values = match self.color {
            Color::Red => [1.0, 0.0, 0.0, 1.0],
            Color::Green => [0.0, 1.0, 0.0, 1.0],
            Color::Blue => [0.0, 0.0, 1.0, 1.0],
            Color::White => [1.0, 1.0, 1.0, 1.0],
            Color::Black => [0.0, 0.0, 0.0, 0.0]
        }
    }

    ///Alpha values help determine age of the cell
    pub fn get_alpha_value(&self) -> f64 {
        self.color_values[3]
    }

    ///This allows us to age cells
    pub fn set_alpha_value(&mut self, alpha_value: f64) {
        self.color_values[3] = alpha_value
    }
}