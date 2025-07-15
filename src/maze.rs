use rand::seq::SliceRandom;
use rand::thread_rng;

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
            cells: vec![vec![Cell::new(true); dimension_y]; dimension_x],
            grid: vec![vec!['#'; dimension_x * 2 + 1]; dimension_y * 2 + 1],
            start_pos: None,
            end_pos: None,
        };
        maze.generate_maze();
        maze.update_grid();
        maze.place_start_end();
        maze
    }

    fn generate_maze(&mut self) {
        let mut rng = thread_rng();
        let mut stack = Vec::new();

        if let Some(start) = self.cells.get_mut(0).and_then(|row| row.get_mut(0)) {
            start.wall = false;
            start.open = false;
        }
        stack.push((0, 0));

        while let Some((x, y)) = stack.pop() {
            let mut neighbors = Vec::new();

            for &(dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && ny >= 0
                    && (nx as usize) < self.dimension_x
                    && (ny as usize) < self.dimension_y
                {
                    let cell = &self.cells[nx as usize][ny as usize];
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
                let current = &mut self.cells[x][y];
                current.add_neighbor((nx, ny));
                current.wall = false;
            }

            let selected = &mut self.cells[nx][ny];
            selected.add_neighbor((x, y));
            selected.wall = false;
            selected.open = false;

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
                self.grid[grid_y][grid_x] = 'S';
                self.start_pos = Some((grid_x, grid_y));
                break;
            }
        }

        for y in (0..self.dimension_y).rev() {
            let grid_x = 0;
            let grid_y = y * 2 + 1;

            if self.grid[grid_y][grid_x + 1] == ' ' {
                self.grid[grid_y][grid_x] = 'E';
                self.end_pos = Some((grid_x, grid_y));
                break;
            }
        }
    }

    pub fn grid(&self) -> &Vec<Vec<char>> {
        &self.grid
    }

    pub fn get_cell_at(&self, x: isize, y: isize) -> Option<char> {
        if x >= 0 && y >= 0 {
            let (x, y) = (x as usize, y as usize);
            if y < self.grid.len() && x < self.grid[0].len() {
                return Some(self.grid[y][x]);
            }
        }
        None
    }

    pub fn start_pos(&self) -> Option<(usize, usize)> {
        self.start_pos
    }

    pub fn end_pos(&self) -> Option<(usize, usize)> {
        self.end_pos
    }
}
