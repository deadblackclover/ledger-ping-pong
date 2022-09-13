use nanos_ui::bagls::{Displayable, Rect};

pub struct Paddle {
    x: i16,
    y: i16,
    width: u16,
    height: u16,
}

impl Paddle {
    pub fn new(x: i16, y: i16) -> Self {
        Paddle {
            x,
            y,
            width: 15,
            height: 3,
        }
    }

    pub fn left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.x < 128 - self.width as i16 {
            self.x += 1;
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
