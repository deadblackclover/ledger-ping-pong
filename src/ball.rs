use nanos_ui::bagls::{Displayable, Rect};

use crate::paddle::Paddle;

pub struct Ball {
    x: i16,
    y: i16,
    dx: i16,
    dy: i16,
    width: u16,
    height: u16,
}

impl Ball {
    pub fn new(x: i16, y: i16) -> Self {
        Ball {
            x,
            y,
            dx: 1,
            dy: 1,
            width: 3,
            height: 3,
        }
    }

    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;

        if self.x + self.width as i16 > 128 {
            self.dx = -1;
        }

        if self.x < self.width as i16 {
            self.dx = 1;
        }

        if self.y + self.height as i16 > 32 {
            self.dy = -1;
        }

        if self.y < self.height as i16 {
            self.dy = 1;
        }
    }

    pub fn paint(&self) {
        Rect::new()
            .dims(self.width, self.height)
            .colors(0, 0x41ccb4u32)
            .fill(false)
            .pos(self.x, self.y)
            .paint();
    }
}
