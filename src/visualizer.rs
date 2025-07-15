use macroquad::prelude::*;

pub struct Visualizer {
    pub cell_size: f32,
    pub maze_grid: Vec<Vec<char>>,
}

impl Visualizer {
    pub fn new(maze_grid: Vec<Vec<char>>) -> Self {
        let cell_size = 20.0;
        Self { cell_size, maze_grid }
    }

    pub fn draw_static_maze(&self) {
        for (y, row) in self.maze_grid.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                let color = match c {
                    '#' => DARKGRAY,
                    ' ' => WHITE,
                    'S' => GREEN,
                    'E' => RED,
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

    pub fn draw_runner(&self, pos: (usize, usize)) {
        let (x, y) = pos;
        let center_x = x as f32 * self.cell_size + self.cell_size / 2.0;
        let center_y = y as f32 * self.cell_size + self.cell_size / 2.0;
        let radius = self.cell_size * 0.4;

        draw_circle(center_x, center_y, radius, BLUE);
    }
}
