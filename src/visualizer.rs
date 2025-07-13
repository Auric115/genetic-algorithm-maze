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

    pub async fn draw_maze(&self) {
        clear_background(BLACK);

        for (y, row) in self.maze_grid.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                let color = match c {
                    '#' => DARKGRAY,
                    ' ' => WHITE,
                    '*' => GREEN,
                    '~' => RED,
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

        next_frame().await;
    }

    pub async fn draw_route(&self, path: &[(usize, usize)]) {
        for &(x, y) in path {
            draw_rectangle(
                x as f32 * self.cell_size,
                y as f32 * self.cell_size,
                self.cell_size,
                self.cell_size,
                BLUE,
            );
            next_frame().await;
        }
    }
}
