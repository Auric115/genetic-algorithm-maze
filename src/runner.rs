use crate::maze::Maze;

pub struct Runner {
    pub pos: (usize, usize),
}

impl Runner {
    pub fn new(start_pos: (usize, usize)) -> Self {
        Self { pos: start_pos }
    }

    pub fn try_move(&mut self, dx: isize, dy: isize, maze: &Maze) {
        let new_x = self.pos.0 as isize + dx;
        let new_y = self.pos.1 as isize + dy;

        if let Some(c) = maze.get_cell_at(new_x, new_y) {
            if c != '#' {
                self.pos = (new_x as usize, new_y as usize);
            }
        }
    }
}
