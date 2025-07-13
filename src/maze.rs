use rand::seq::SliceRandom;
use rand::thread_rng;

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

    pub fn display(&self) {
        for row in &self.grid {
            let line: String = row.iter().collect();
            println!("{}", line);
        }
    }

    pub fn test_route(&self, route: Vec<u8>) -> f64 {
        let (mut x, mut y) = match self.start_pos {
            Some(pos) => pos,
            None => return 0.0,
        };

        let goal = match self.end_pos {
            Some(pos) => pos,
            None => return 0.0,
        };

        let mut steps = 0;
        let max_x = self.grid[0].len();
        let max_y = self.grid.len();

        for dir in route {
            let (dx, dy) = match dir {
                0 => (0, -1),  // Up
                1 => (1, 0),   // Right
                2 => (0, 1),   // Down
                3 => (-1, 0),  // Left
                _ => (0, 0),   // Invalid
            };

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < max_x as isize && ny >= 0 && ny < max_y as isize {
                let gx = nx as usize;
                let gy = ny as usize;
                if self.grid[gy][gx] != '#' {
                    x = gx;
                    y = gy;
                    steps += 1;

                    if (x, y) == goal {
                        // Reached the goal
                        return 1000.0 - steps as f64;
                    }
                }
            }
        }

        let dist = ((goal.0 as isize - x as isize).abs()
            + (goal.1 as isize - y as isize).abs()) as f64;

        1.0 / (1.0 + dist)
    }

}
