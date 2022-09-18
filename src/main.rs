#![no_std]
#![no_main]

mod ball;
mod game;
mod paddle;

use game::Game;
use nanos_sdk::buttons::ButtonEvent;
use nanos_sdk::io;
use nanos_ui::ui;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

/// Basic nested menu. Will be subject
/// to simplifications in the future.
#[allow(clippy::needless_borrow)]
fn menu() {
    loop {
        match ui::Menu::new(&[&"Start game", &"Infos", &"Back", &"Exit App"]).show() {
            0 => game(),
            1 => loop {
                match ui::Menu::new(&[&"Copyright", &"Authors", &"Back"]).show() {
                    0 => ui::popup("2022 CipherDogs"),
                    1 => ui::popup("DEADBLACKCLOVER"),
                    _ => break,
                }
            },
            2 => return,
            3 => nanos_sdk::exit_app(0),
            _ => (),
        }
    }
}

/// Start game
fn game() {
    Game::new().event_loop();
    ui::popup("GAME OVER");
}

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = io::Comm::new();

    loop {
        // Draw some 'welcome' screen
        ui::SingleMessage::new("W e l c o m e").show();

        // Wait for either a specific button push to exit the app
        // or an APDU command
        match comm.next_event() {
            io::Event::Button(ButtonEvent::BothButtonsRelease) => menu(),
            io::Event::Command(ins) => match handle_apdu(&mut comm, ins) {
                Ok(()) => comm.reply_ok(),
                Err(sw) => comm.reply(sw),
            },
            _ => (),
        }
    }
}

#[repr(u8)]
enum Ins {
    Game,
    Menu,
    Exit,
}

impl From<u8> for Ins {
    fn from(ins: u8) -> Ins {
        match ins {
            3 => Ins::Game,
            4 => Ins::Menu,
            0xff => Ins::Exit,
            _ => panic!(),
        }
    }
}

use nanos_sdk::io::Reply;

fn handle_apdu(comm: &mut io::Comm, ins: Ins) -> Result<(), Reply> {
    if comm.rx == 0 {
        return Err(io::StatusWords::NothingReceived.into());
    }

    match ins {
        Ins::Game => game(),
        Ins::Menu => menu(),
        Ins::Exit => nanos_sdk::exit_app(0),
    }
    Ok(())
}
