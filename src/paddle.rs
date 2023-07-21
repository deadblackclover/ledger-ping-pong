use nanos_ui::{
    bagls::{RectFull, SendToDisplay},
    PADDING, SCREEN_WIDTH,
};

use crate::ball::Ball;

pub struct Paddle {
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    speed: i16,
}

impl Paddle {
    pub fn new(x: i16, y: i16, speed: i16) -> Self {
        Paddle {
            x,
            y,
            width: 15,
            height: 3,
            speed,
        }
    }

    pub fn get_coordinates(&self) -> (i16, i16, i16, i16) {
        (
            self.x,
            self.y,
            self.x + (self.width as i16),
            self.y + (self.height as i16),
        )
    }

    pub fn left(&mut self) {
        if self.x > 0 {
            self.x -= self.speed;
        }
    }

    pub fn right(&mut self) {
        if self.x + (self.width as i16) <= (SCREEN_WIDTH - PADDING * 2) as i16 {
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
            .width(self.width as u32)
            .height(self.height as u32)
            .pos(self.x as i32, self.y as i32)
            .paint();
    }
}
