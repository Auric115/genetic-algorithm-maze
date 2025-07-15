use macroquad::prelude::*;
use crate::maze::Maze;

pub struct Runner {
    pub position: Vec2,
    pub radius: f32,
    pub speed: f32,
}

impl Runner {
    pub fn new(position: Vec2, radius: f32, speed: f32) -> Self {
        Self {
            position,
            radius,
            speed,
        }
    }

    pub fn update(&mut self, dt: f32, maze: &Maze) {
        let mut movement = Vec2::ZERO;
        if is_key_down(KeyCode::Up) {
            movement.y -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            movement.y += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            movement.x -= 1.0;
        }
        if is_key_down(KeyCode::Right) {
            movement.x += 1.0;
        }

        if movement.length() > 0.0 {
            movement = movement.normalize() * self.speed * dt;
            let proposed = self.position + movement;

            let mut collided = false;
            for wall in maze.walls() {
                if circle_collides_rect(proposed, self.radius, wall) {
                    collided = true;
                    break;
                }
            }

            if !collided {
                self.position = proposed;
            }

            let bounds = maze.bounds();
            self.position.x = self.position.x.clamp(bounds.x + self.radius, bounds.x + bounds.w - self.radius);
            self.position.y = self.position.y.clamp(bounds.y + self.radius, bounds.y + bounds.h - self.radius);

        }

        // Print distances to nearest walls and goal
        // self.print_environment_info(maze);
    }

    fn print_environment_info(&self, maze: &Maze) {
        let walls = maze.walls();

        let mut up_dist = f32::MAX;
        let mut down_dist = f32::MAX;
        let mut left_dist = f32::MAX;
        let mut right_dist = f32::MAX;

        for wall in &walls {
            // Horizontal checks
            if self.position.y > wall.y + wall.h / 2.0 && self.position.x >= wall.x && self.position.x <= wall.x + wall.w {
                let dist = self.position.y - (wall.y + wall.h);
                if dist < up_dist {
                    up_dist = dist;
                }
            }

            if self.position.y < wall.y && self.position.x >= wall.x && self.position.x <= wall.x + wall.w {
                let dist = wall.y - self.position.y;
                if dist < down_dist {
                    down_dist = dist;
                }
            }

            // Vertical checks
            if self.position.x > wall.x + wall.w / 2.0 && self.position.y >= wall.y && self.position.y <= wall.y + wall.h {
                let dist = self.position.x - (wall.x + wall.w);
                if dist < left_dist {
                    left_dist = dist;
                }
            }

            if self.position.x < wall.x && self.position.y >= wall.y && self.position.y <= wall.y + wall.h {
                let dist = wall.x - self.position.x;
                if dist < right_dist {
                    right_dist = dist;
                }
            }
        }

        println!(
            "Wall Distances → Up: {:.1}, Down: {:.1}, Left: {:.1}, Right: {:.1}",
            up_dist, down_dist, left_dist, right_dist
        );

        if let Some(end) = maze.end_position() {
            let vec_to_end = end - self.position;
            let distance = vec_to_end.length();
            let angle_deg = vec_to_end.angle_between(Vec2::new(1.0, 0.0)).to_degrees();

            println!(
                "To End → Distance: {:.1}, Angle: {:.1}°",
                distance, angle_deg
            );
        }
    }
}

fn circle_collides_rect(circle: Vec2, radius: f32, rect: Rect) -> bool {
    let closest_x = circle.x.clamp(rect.x, rect.x + rect.w);
    let closest_y = circle.y.clamp(rect.y, rect.y + rect.h);
    let dx = circle.x - closest_x;
    let dy = circle.y - closest_y;
    dx * dx + dy * dy < radius * radius
}
