use macroquad::prelude::*;

pub struct Visualizer {
    pub grid: Vec<Vec<char>>,
    pub cell_size: f32,
}

impl Visualizer {
    pub fn new(grid: Vec<Vec<char>>, cell_size: f32) -> Self {
        Self { grid, cell_size }
    }

    pub fn draw_maze(&self) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                let color = match ch {
                    '#' => DARKGRAY,
                    'S' => GREEN,
                    'E' => RED,
                    ' ' => WHITE,
                    _ => WHITE,
                };
                draw_rectangle(
                    x as f32 * self.cell_size,
                    y as f32 * self.cell_size,
                    self.cell_size,
                    self.cell_size,
                    color,
                );
            }
        }
    }

    pub fn draw_runner(&self, position: Vec2, radius: f32) {
        draw_circle(position.x, position.y, radius, BLUE);
    }
}
