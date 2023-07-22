use nanos_ui::{
    bagls::{RectFull, SendToDisplay},
    PADDING, SCREEN_HEIGHT, SCREEN_WIDTH,
};

pub struct Ball {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    width: u32,
    height: u32,
}

impl Ball {
    pub fn new(x: i32, y: i32) -> Self {
        Ball {
            x,
            y,
            dx: 1,
            dy: 1,
            width: 3,
            height: 3,
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

    pub fn rebound(&mut self, dy: i32) {
        if dy == -1 || dy == 1 {
            self.dy = dy;
        }
    }

    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;

        if self.x + self.width as i32 > (SCREEN_WIDTH - PADDING * 2) as i32 {
            self.dx = -1;
        }

        if self.x <= 0 {
            self.dx = 1;
        }
    }

    pub fn is_game_over(&self) -> bool {
        if self.y + self.height as i32 > SCREEN_HEIGHT as i32 {
            return true;
        }

        if self.y <= 0 {
            return true;
        }

        false
    }

    pub fn paint(&self) {
        RectFull::new()
            .width(self.width)
            .height(self.height)
            .pos(self.x, self.y)
            .paint();
    }
}
