use nanos_ui::{
    bagls::{RectFull, SendToDisplay},
    PADDING, SCREEN_WIDTH,
};

use crate::ball::Ball;

pub struct Paddle {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    speed: i32,
}

impl Paddle {
    pub fn new(x: i32, y: i32, speed: i32) -> Self {
        Paddle {
            x,
            y,
            width: 15,
            height: 3,
            speed,
        }
    }

    pub fn get_coordinates(&self) -> (i32, i32, i32, i32) {
        (
            self.x,
            self.y,
            self.x + (self.width as i32),
            self.y + (self.height as i32),
        )
    }

    pub fn left(&mut self) {
        if self.x > 0 {
            self.x -= self.speed;
        }
    }

    pub fn right(&mut self) {
        if self.x + (self.width as i32) <= (SCREEN_WIDTH - PADDING * 2) as i32 {
            self.x += self.speed;
        }
    }

    pub fn kick(&mut self, ball: &mut Ball) {
        let (b_x, b_y, b_x1, b_y1) = ball.get_coordinates();
        let (x, y, x1, y1) = self.get_coordinates();

        if b_x1 > x && b_x < x1 {
            if b_y1 >= y && b_y1 - y <= 1 {
                ball.rebound(-1);
            }

            if b_y <= y1 && y1 - b_y <= 1 {
                ball.rebound(1);
            }
        }
    }

    pub fn paint(&self) {
        RectFull::new()
            .width(self.width)
            .height(self.height)
            .pos(self.x, self.y)
            .paint();
    }
}
