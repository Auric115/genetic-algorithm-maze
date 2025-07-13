use crate::maze::{MAZE_MAP, MAZE_WIDTH, MAZE_HEIGHT, GOAL_X, GOAL_Y};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Path {
    pub moves: Vec<Direction>,
}

#[derive(Debug)]
pub struct AgentResult {
    pub end_x: usize,
    pub end_y: usize,
    pub reached_goal: bool,
    pub steps_taken: usize,
    pub final_distance: f64,
}

impl Path {
    pub fn new(moves: Vec<Direction>) -> Self {
        Self { moves }
    }

    pub fn simulate(&self, start_x: usize, start_y: usize) -> AgentResult {
        let mut x = start_x;
        let mut y = start_y;

        let mut steps = 0;

        for mv in &self.moves {
            let (new_x, new_y) = match mv {
                Direction::Up => (x, y.saturating_sub(1)),
                Direction::Down => (x, (y + 1).min(MAZE_HEIGHT - 1)),
                Direction::Left => (x.saturating_sub(1), y),
                Direction::Right => ((x + 1).min(MAZE_WIDTH - 1), y),
            };

            // Only move if the target is not a wall
            if MAZE_MAP[new_y][new_x] != 1 {
                x = new_x;
                y = new_y;
            }

            steps += 1;

            // Early exit if goal reached
            if x == GOAL_X && y == GOAL_Y {
                break;
            }
        }

        let dist = ((x as isize - GOAL_X as isize).pow(2)
            + (y as isize - GOAL_Y as isize).pow(2)) as f64;

        AgentResult {
            end_x: x,
            end_y: y,
            reached_goal: x == GOAL_X && y == GOAL_Y,
            steps_taken: steps,
            final_distance: dist.sqrt(),
        }
    }

    pub fn fitness(&self, start_x: usize, start_y: usize) -> f64 {
        let result = self.simulate(start_x, start_y);

        let distance_penalty = result.final_distance;
        let step_penalty = result.steps_taken as f64 * 0.1;

        let fitness = if result.reached_goal {
            1000.0 - step_penalty
        } else {
            1.0 / (1.0 + distance_penalty + step_penalty)
        };

        fitness
    }
}

