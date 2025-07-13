// maze.rs

use ::rand::seq::SliceRandom;
use ::rand::thread_rng;
use macroquad::prelude::*;

#[derive(Clone, Debug)]
struct Cell {
    x: usize,
    y: usize,
    neighbors: Vec<(usize, usize)>,
    wall: bool,
    open: bool,
}

impl Cell {
    fn new(x: usize, y: usize, is_wall: bool) -> Self {
        Self {
            x,
            y,
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
    dimension_x: usize,
    dimension_y: usize,
    cells: Vec<Vec<Cell>>,
    grid: Vec<Vec<char>>,
    start_pos: Option<(usize, usize)>,
    end_pos: Option<(usize, usize)>,
}

impl Maze {
    pub fn new(dimension_x: usize, dimension_y: usize) -> Self {
        let mut maze = Maze {
            dimension_x,
            dimension_y,
            cells: vec![vec![Cell::new(0, 0, true); dimension_y]; dimension_x],
            grid: vec![vec!['#'; dimension_x * 2 + 1]; dimension_y * 2 + 1],
            start_pos: None,
            end_pos: None,
        };
        maze.init_cells();
        maze.generate_maze();
        maze.update_grid();
        maze.place_start_end();
        maze
    }

    fn init_cells(&mut self) {
        for x in 0..self.dimension_x {
            for y in 0..self.dimension_y {
                self.cells[x][y] = Cell::new(x, y, true);
            }
        }
    }

    fn get_cell(&self, x: isize, y: isize) -> Option<&Cell> {
        if x >= 0 && y >= 0 && (x as usize) < self.dimension_x && (y as usize) < self.dimension_y {
            Some(&self.cells[x as usize][y as usize])
        } else {
            None
        }
    }

    fn get_cell_mut(&mut self, x: isize, y: isize) -> Option<&mut Cell> {
        if x >= 0 && y >= 0 && (x as usize) < self.dimension_x && (y as usize) < self.dimension_y {
            Some(&mut self.cells[x as usize][y as usize])
        } else {
            None
        }
    }

    pub fn get_grid(&self) -> Vec<Vec<char>> {
        self.grid.clone()
    }

    fn generate_maze(&mut self) {
        let mut rng = thread_rng();
        let mut stack = Vec::new();

        if let Some(start) = self.get_cell_mut(0, 0) {
            start.wall = false;
            start.open = false;
        }
        stack.push((0, 0));

        while let Some((x, y)) = stack.pop() {
            let mut neighbors = Vec::new();

            for &(dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if let Some(cell) = self.get_cell(nx, ny) {
                    if cell.wall && cell.open {
                        neighbors.push((nx as usize, ny as usize));
                    }
                }
            }

            if neighbors.is_empty() {
                continue;
            }

            stack.push((x, y));

            let &(nx, ny) = neighbors.choose(&mut rng).unwrap();

            {
                {
                    let current = self.get_cell_mut(x as isize, y as isize).unwrap();
                    current.add_neighbor((nx, ny));
                    current.wall = false;
                }

                let selected = self.get_cell_mut(nx as isize, ny as isize).unwrap();
                selected.add_neighbor((x, y));
                selected.wall = false;
                selected.open = false;
            }

            stack.push((nx, ny));
        }
    }

    fn update_grid(&mut self) {
        for row in self.grid.iter_mut() {
            for ch in row.iter_mut() {
                *ch = '#';
            }
        }

        for x in 0..self.dimension_x {
            for y in 0..self.dimension_y {
                let grid_x = x * 2 + 1;
                let grid_y = y * 2 + 1;

                if !self.cells[x][y].wall {
                    self.grid[grid_y][grid_x] = ' ';
                }

                for &(nx, ny) in &self.cells[x][y].neighbors {
                    let passage_x = (grid_x + (nx * 2 + 1)) / 2;
                    let passage_y = (grid_y + (ny * 2 + 1)) / 2;
                    self.grid[passage_y][passage_x] = ' ';
                }
            }
        }
    }

    fn place_start_end(&mut self) {
        for y in 0..self.dimension_y {
            let grid_x = self.dimension_x * 2;
            let grid_y = y * 2 + 1;

            if self.grid[grid_y][grid_x - 1] == ' ' {
                self.grid[grid_y][grid_x] = '*';
                self.start_pos = Some((grid_x, grid_y));
                break;
            }
        }

        for y in (0..self.dimension_y).rev() {
            let grid_x = 0;
            let grid_y = y * 2 + 1;

            if self.grid[grid_y][grid_x + 1] == ' ' {
                self.grid[grid_y][grid_x] = '~';
                self.end_pos = Some((grid_x, grid_y));
                break;
            }
        }
    }

    pub fn _display(&self) {
        for row in &self.grid {
            let line: String = row.iter().collect();
            println!("{}", line);
        }
    }

    pub fn test_route(&self, route: Vec<u8>) -> f64 {
        let mut pos = self.start_pos().unwrap();
        let mut visited = std::collections::HashSet::new();
        visited.insert(pos);
        let mut fitness:f64 = 0.0;

        for dir in route {
            let (dx, dy) = match dir {
                0 => (0, -1),
                1 => (1, 0),
                2 => (0, 1),
                3 => (-1, 0),
                _ => (0, 0),
            };

            let new_x = (pos.0 as isize + dx) as usize;
            let new_y = (pos.1 as isize + dy) as usize;

            if new_x >= self.grid[0].len() || new_y >= self.grid.len() || self.grid[new_y][new_x] == '#' {
                fitness -= 1.0;
                continue;
            }

            pos = (new_x, new_y);
            if visited.insert(pos) {
                fitness += 1.0;
            }

            if Some(pos) == self.end_pos() {
                fitness += 1000.0;
                break;
            }
        }

        if let Some(end) = self.end_pos() {
            let dx = (end.0 as isize - pos.0 as isize).abs() as f64;
            let dy = (end.1 as isize - pos.1 as isize).abs() as f64;
            fitness += 10.0 / (1.0 + dx + dy);
        }

        fitness
    }

    pub fn start_pos(&self) -> Option<(usize, usize)> {
        self.start_pos
    }

    pub fn end_pos(&self) -> Option<(usize, usize)> {
        self.end_pos
    }

    pub fn grid(&self) -> &Vec<Vec<char>> {
        &self.grid
    }

}
