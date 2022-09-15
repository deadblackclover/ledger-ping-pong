use nanos_ui::bagls::{Displayable, Rect};

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

    pub fn get_coordinates(&self) -> (i16, i16, i16, i16) {
        (
            self.x,
            self.y,
            self.x + (self.width as i16),
            self.y + (self.height as i16),
        )
    }

    pub fn rebound(&mut self, dy: i16) {
        if dy == -1 || dy == 1 {
            self.dy = dy;
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
    }

    pub fn is_game_over(&self) -> bool {
        if self.y + self.height as i16 > 32 {
            return true;
        }

        if self.y < self.height as i16 {
            return true;
        }

        false
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
