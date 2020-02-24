#![warn(missing_docs)]
//! Simple board game as TUI

/// Contains fundamental game structs, game logic
pub mod game;
/// Contains the Terminal User Interface
mod gui;

use std::io;
use self::game::{board::*, Game, GameState::*};
use self::gui::{Gui, Event};

fn main() -> io::Result<()> {
	let res = run();
	log::info!("{:?}", res);
	res
}

fn run() -> io::Result<()> {
    // FIXME: when used outside of this dir, the log files are around everywhere!
    flexi_logger::Logger::with_env_or_str("info, quart::gui=debug")
        .log_to_file()
        .directory("logs")
        .create_symlink("log.log")
        .format(flexi_logger::default_format)
        .start()
        .unwrap();

    // game state
    let mut game = Game::new();
    log::debug!("Created game");

    let mut gui = gui::create_default()?;

    // initial drawing
    gui.draw(&game)?;

	// TODO: switch to gui.poll_event() and gui::Event
    // game loop
    while let Some(event) = gui.poll_event(&game) {
        match event {
            Event::Exit => break,
            Event::CursorUp    => game.move_cursor(0, -1),
            Event::CursorDown  => game.move_cursor(0, 1),
            Event::CursorLeft  => game.move_cursor(-1, 0),
            Event::CursorRight => game.move_cursor(1, 0),
            Event::CursorToX(x) => game.set_cursor_x(x),
            Event::CursorToY(y) => game.set_cursor_y(y),
            Event::Enter => game.enter(),
			Event::CursorToPos(pos) => game.set_cursor_pos(pos),
	        // _ => continue,
        }

        if game.check() {
	        log::info!("Game Over: {:?}", game.game_over_info);
            // GAME OVER
            // break;
        }

        // redraw boards and piece preview
        gui.draw(&game)?;
    }
    log::trace!("After game loop");

    std::mem::drop(gui);

	log::info!("End, {:?}", game);
    if game.state == GameOver {
        println!("+++ GAME OVER +++");
        println!("Player {} lost", game.player_turn);
        println!("{:?}", game.game_over_info);
    }

    Ok(())
}
