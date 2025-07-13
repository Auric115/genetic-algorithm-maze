pub const MAZE_WIDTH: usize = 15;
pub const MAZE_HEIGHT: usize = 10;

pub const MAZE_MAP: [[u8; MAZE_WIDTH]; MAZE_HEIGHT] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1],
    [8, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1],
    [1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1],
    [1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 5],
    [1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

pub const START_X: usize = 14;
pub const START_Y: usize = 7;
pub const GOAL_X: usize = 0;
pub const GOAL_Y: usize = 2;

pub fn draw_maze(agent_pos: Option<(usize, usize)>) {
    for y in 0..MAZE_HEIGHT {
        for x in 0..MAZE_WIDTH {
            if let Some((ax, ay)) = agent_pos {
                if x == ax && y == ay {
                    print!("*");
                    continue;
                }
            }

            match MAZE_MAP[y][x] {
                0 => print!(" "),     // path
                1 => print!("#"),     // wall
                5 => print!("G"),     // goal
                8 => print!("S"),     // start
                _ => print!("?"),     // unknown
            }
        }
        println!();
    }
}

