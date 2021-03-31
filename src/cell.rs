pub enum Color {
    Red,
    Green,
    Blue,
    White,
    Black
}

pub fn map_color(color: & Color) -> [f32; 4] {
    match color {
        Color::Black => [0.0, 0.0, 0.0, 0.0],
        Color::White => [1.0, 1.0, 1.0, 1.0],
        Color::Red => [1.0, 0.0, 0.0, 1.0],
        Color::Green => [0.0, 1.0, 0.0, 1.0],
        Color::Blue => [0.0, 0.0, 1.0, 1.0]
    }
}

pub struct Cell {
    color: Color,
    color_values: [f32;4]
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

    pub fn assign_color(&mut self, color: Color) {
        self.color = color;
        self.assign_color_values();
    }

    ///Each color will separate the cells in case of multiple colonies
    pub fn assign_color_values(&mut self) {
        self.color_values = map_color(& self.color)
    }

    ///Alpha values help determine age of the cell
    pub fn get_alpha_value(&self) -> f32 {
        self.color_values[3]
    }

    ///This allows us to age cells
    pub fn set_alpha_value(&mut self, alpha_value: f32) {
        self.color_values[3] = alpha_value
    }
}