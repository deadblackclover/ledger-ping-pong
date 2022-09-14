use nanos_ui::bagls::{Displayable, Rect};

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
        if self.x + (self.width as i16) < 128 {
            self.x += self.speed;
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
