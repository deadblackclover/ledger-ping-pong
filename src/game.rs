use crate::{ball::Ball, paddle::Paddle};
use core::arch::asm;
use nanos_sdk::buttons::{ButtonEvent, ButtonsState};
use nanos_ui::ui;
use nanos_ui::{PADDING, SCREEN_HEIGHT, SCREEN_WIDTH};

enum Motion {
    Left,
    Right,
    None,
}

pub struct Game {
    player: Paddle,
    opponent: Paddle,
    ball: Ball,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player: Paddle::new(
                (SCREEN_WIDTH / 2) as i32,
                (SCREEN_HEIGHT - (PADDING * 2)) as i32,
                4,
            ),
            opponent: Paddle::new((SCREEN_WIDTH / 2) as i32, 0, 4),
            ball: Ball::new(
                (SCREEN_WIDTH / 2) as i32,
                ((SCREEN_HEIGHT - (PADDING * 2)) / 2) as i32,
            ),
        }
    }

    pub fn event_loop(&mut self) {
        let mut buttons = ButtonsState::new();

        let mut draw = |motion: Motion| -> bool {
            ui::clear_screen();

            // The player acts
            match motion {
                Motion::Left => self.player.left(),
                Motion::Right => self.player.right(),
                Motion::None => (),
            }
            self.player.paint();

            // The opponent acts
            {
                let (op_x, _, op_x1, _) = self.opponent.get_coordinates();
                let (b_x, _, b_x1, _) = self.ball.get_coordinates();

                if b_x1 <= op_x {
                    self.opponent.left();
                }

                if b_x >= op_x1 {
                    self.opponent.right();
                }
            }
            self.opponent.paint();

            // The ball acts
            self.ball.update();
            self.ball.paint();

            // If the paddle can bounce the ball, it bounces
            self.player.kick(&mut self.ball);
            self.opponent.kick(&mut self.ball);

            nanos_ui::screen_util::screen_update();

            // Check if the ball touched the edge of the screen
            self.ball.is_game_over()
        };

        draw(Motion::None);

        loop {
            match ui::get_event(&mut buttons) {
                Some(ButtonEvent::LeftButtonRelease) => {
                    let result = draw(Motion::Left);
                    if result {
                        break;
                    }
                }
                Some(ButtonEvent::RightButtonRelease) => {
                    let result = draw(Motion::Right);
                    if result {
                        break;
                    }
                }
                Some(ButtonEvent::BothButtonsRelease) => break,
                Some(_) | None => {
                    // Dirty hack
                    for _ in 0..100_000 {
                        unsafe {
                            asm!("nop");
                        }
                    }

                    let result = draw(Motion::None);
                    if result {
                        break;
                    }
                }
            }
        }
    }
}
