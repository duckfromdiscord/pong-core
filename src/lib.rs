use rand::prelude::*;
use rand::seq::SliceRandom;

pub struct PongGame {
    width: usize,
    height: usize,
    left_x: usize,
    left_y: usize,
    ball_x: usize,
    ball_y: usize,
    right_x: usize,
    right_y: usize,
    angle: usize,
}

impl PongGame {
    pub fn new() -> PongGame {
        PongGame {
            width: 128,
            height: 64,
            left_x: 3,
            left_y: 22,
            ball_x: 63,
            ball_y: 31,
            right_x: 124,
            right_y: 22,
            angle: 0
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

        if (self.left_x <= self.ball_x && self.ball_x <= self.left_x+6) && (self.left_y <= self.ball_y && self.ball_y <= self.left_y+20) {
            self.angle = rng.gen_range(4..=6);
        }

        if (self.right_x-6 <= self.ball_x && self.ball_x <= self.right_x) && (self.right_y <= self.ball_y && self.ball_y <= self.right_y+20) {
            self.angle = rng.gen_range(1..=3);
        }

        if self.ball_x >= 127 || self.ball_x <= 0 {
            // a score was made
            self.ball_x = 63;
            self.ball_y = 31;
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

        if self.ball_y >= 63 {
            self.ball_y = 63;
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