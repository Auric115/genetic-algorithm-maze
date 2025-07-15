use macroquad::prelude::*;
use ::rand::seq::SliceRandom;
use ::rand::thread_rng;

#[derive(Clone, Debug)]
struct Cell {
    neighbors: Vec<(usize, usize)>,
    wall: bool,
    open: bool,
}

impl Cell {
    fn new(is_wall: bool) -> Self {
        Self {
            neighbors: Vec::new(),
            wall: is_wall,
            open: true,
        }
    }

    fn add_neighbor(&mut self, neighbor: (usize, usize)) {
        if !self.neighbors.contains(&neighbor) {
            self.neighbors.push(neighbor);
        }
    }
}

pub struct Maze {
    pub grid: Vec<Vec<char>>,
    cell_size: f32,
}

impl Maze {
    pub fn new(dimension_x: usize, dimension_y: usize, cell_size: f32) -> Self {
        let mut cells = vec![vec![Cell::new(true); dimension_y]; dimension_x];
        let mut grid = vec![vec!['#'; dimension_x * 2 + 1]; dimension_y * 2 + 1];

        let mut stack = Vec::new();
        let mut rng = thread_rng();

        cells[0][0].wall = false;
        cells[0][0].open = false;
        stack.push((0, 0));

        while let Some((x, y)) = stack.pop() {
            let mut neighbors = Vec::new();
            for &(dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && ny >= 0
                    && (nx as usize) < dimension_x
                    && (ny as usize) < dimension_y
                    && cells[nx as usize][ny as usize].wall
                    && cells[nx as usize][ny as usize].open
                {
                    neighbors.push((nx as usize, ny as usize));
                }
            }

            if neighbors.is_empty() {
                continue;
            }

            stack.push((x, y));
            let &(nx, ny) = neighbors.choose(&mut rng).unwrap();
            cells[x][y].add_neighbor((nx, ny));
            cells[x][y].wall = false;

            cells[nx][ny].add_neighbor((x, y));
            cells[nx][ny].wall = false;
            cells[nx][ny].open = false;

            stack.push((nx, ny));
        }

        for y in 0..dimension_y {
            for x in 0..dimension_x {
                let gx = x * 2 + 1;
                let gy = y * 2 + 1;
                if !cells[x][y].wall {
                    grid[gy][gx] = ' ';
                }
                for &(nx, ny) in &cells[x][y].neighbors {
                    let px = (gx + (nx * 2 + 1)) / 2;
                    let py = (gy + (ny * 2 + 1)) / 2;
                    grid[py][px] = ' ';
                }
            }
        }

        // Place start and end
        for y in 0..dimension_y {
            let gx = dimension_x * 2;
            let gy = y * 2 + 1;
            if grid[gy][gx - 1] == ' ' {
                grid[gy][gx] = 'S';
                break;
            }
        }

        for y in (0..dimension_y).rev() {
            let gx = 0;
            let gy = y * 2 + 1;
            if grid[gy][gx + 1] == ' ' {
                grid[gy][gx] = 'E';
                break;
            }
        }

        Self { grid, cell_size }
    }

    pub fn walls(&self) -> Vec<Rect> {
        let mut wall_rects = Vec::new();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                if ch == '#' {
                    wall_rects.push(Rect::new(
                        x as f32 * self.cell_size,
                        y as f32 * self.cell_size,
                        self.cell_size,
                        self.cell_size,
                    ));
                }
            }
        }
        wall_rects
    }

    pub fn bounds(&self) -> Rect {
        Rect::new(
            0.0,
            0.0,
            self.grid[0].len() as f32 * self.cell_size,
            self.grid.len() as f32 * self.cell_size,
        )
    }

    pub fn start_position(&self) -> Option<Vec2> {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                if ch == 'S' {
                    return Some(Vec2::new(
                        x as f32 * self.cell_size + self.cell_size / 2.0,
                        y as f32 * self.cell_size + self.cell_size / 2.0,
                    ));
                }
            }
        }
        None
    }

    pub fn end_position(&self) -> Option<Vec2> {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                if ch == 'E' {
                    return Some(Vec2::new(
                        x as f32 * self.cell_size + self.cell_size / 2.0,
                        y as f32 * self.cell_size + self.cell_size / 2.0,
                    ));
                }
            }
        }
        None
    }
}
