use rand::prelude::*;
use rand::seq::SliceRandom;

pub struct PongGame {
    pub width: usize,
    pub height: usize,
    pub left_x: usize,
    pub left_y: usize,
    pub ball_x: usize,
    pub ball_y: usize,
    pub ball_start_y: usize,
    pub ball_start_x: usize,
    pub right_x: usize,
    pub right_y: usize,
    pub angle: usize,
    pub paddle_size_x: f32,
    pub paddle_size_y: f32,
    pub ball_radius: f32,
}

impl PongGame {
    pub fn new_size(width: usize, height: usize, left_x: usize, right_x: usize, paddle_starty: usize, ball_x: usize, ball_y: usize, paddle_size_x: f32, paddle_size_y: f32, ball_radius: f32) -> PongGame {
        PongGame {
            width,
            height,
            left_x,
            left_y: paddle_starty,
            ball_x,
            ball_y,
            ball_start_x: ball_x,
            ball_start_y: ball_y,
            right_x,
            right_y: paddle_starty,
            angle: 0,
            paddle_size_x,
            paddle_size_y,
            ball_radius,
        }
    }
    pub fn new() -> PongGame {
        PongGame {
            width: 128,
            height: 64,
            left_x: 3,
            left_y: 22,
            ball_x: 63,
            ball_y: 31,
            ball_start_x: 63,
            ball_start_y: 31,
            right_x: 124,
            right_y: 22,
            angle: 0,
            paddle_size_x: 6f32,
            paddle_size_y: 20f32,
            ball_radius: 3f32,
        }
    }

    pub fn pick_direction(&self) -> usize {
        let mut rng = rand::thread_rng();

        if rand::random() {
            // left
            rng.gen_range(1..=3)
        } else {
            // right
            rng.gen_range(4..=6)
        }
    }

    pub fn time_step(&mut self) {
        let mut rng = rand::thread_rng();

        if (self.left_x <= self.ball_x && self.ball_x <= self.left_x+(self.paddle_size_x as usize)) && (self.left_y <= self.ball_y && self.ball_y <= self.left_y+(self.paddle_size_y as usize)) {
            self.angle = rng.gen_range(4..=6);
        }

        if (self.right_x-(self.paddle_size_x as usize) <= self.ball_x && self.ball_x <= self.right_x) && (self.right_y <= self.ball_y && self.ball_y <= self.right_y+(self.paddle_size_y as usize)) {
            self.angle = rng.gen_range(1..=3);
        }

        if self.ball_x >= self.width-1 || self.ball_x <= 0 {
            // a score was made
            self.ball_x = self.ball_start_x;
            self.ball_y = self.ball_start_y;
        }


        if self.angle == 0 {
            self.angle = self.pick_direction();
        }

        if self.angle == 1 {
            self.ball_y -= 1;
            self.ball_x -= 1;
        }


        if self.angle == 2 {
            self.ball_x -= 1;
        }

        if self.angle == 3 {
            self.ball_y += 1;
            self.ball_x -= 1;
        }

        if self.angle == 4 {
            self.ball_y -= 1;
            self.ball_x += 1;
        }

        if self.angle == 5 {
            self.ball_x += 1;
        }

        if self.angle == 6 {
            self.ball_y += 1;
            self.ball_x += 1;
        }

        if self.ball_y >= self.height-1 {
            self.ball_y = self.height-1;
            if 1 <= self.angle && self.angle <= 3 {
                self.angle = 1;
            } else if 4 <= self.angle && self.angle <= 6 {
                self.angle = 4;
            } else {
                self.angle = *vec![1 as usize, 4 as usize].choose(&mut rng).unwrap();
            }
        }


        if self.ball_y <= 0 {
            self.ball_y = 0;
            if 1 <= self.angle && self.angle <= 3 {
                self.angle = 3;
            } else if 4 <= self.angle && self.angle <= 6 {
                self.angle = 6;
            } else {
                self.angle = *vec![3 as usize, 6 as usize].choose(&mut rng).unwrap();
            }
        }

    }
}