#![warn(missing_docs)]
//! Simple board game as TUI

/// Contains the Terminal User Interface
pub mod gui;

use std::io;
use quart_lib::{board::*, Game, GameState::*};
use self::gui::{Gui, Event};

#[cfg(feature = "ai_enemy")]
use quart_ai_enemy::*;

fn main() -> io::Result<()> {
	let res = run();
	log::info!("{:?}", res);
	res
}

fn run() -> io::Result<()> {
    flexi_logger::Logger::with_env_or_str("info, quart::gui=debug")
        .log_to_file()
        .directory     (concat!(env!("CARGO_MANIFEST_DIR"), "/logs"))
        .create_symlink(concat!(env!("CARGO_MANIFEST_DIR"), "/log.log"))
        .format(flexi_logger::default_format)
        .start()
        .unwrap();

    // game state
    let mut game = Game::new();
    log::debug!("Created game");

	// FIXME: now we created an AiEnemy, but when does its `play()` get called?
	// It seems that we have mixed up Game logic and GUI pretty much,
	// so that the `Game` struct handles GUI stuff as well. This sucks ^^
	// Maybe a possibility to continue would be to extend the match arm for Event::Enter
	// and after calling `game.enter()`, check whether now it's the other players turn.
	// Then we let the `AiEnemy` do his move and so on.
	// Howewer I think that a redesign is the better idea, entirely splitting game logic and user interface
	#[cfg(feature = "ai_enemy")]
    let _ai_enemy = AiEnemy::new();

    let mut gui = gui::create_default()?;

    // initial drawing
    gui.draw(&game)?;

    // game event loop
    while let Some(event) = gui.poll_event(&game) {
        match event {
			Event::Exit 			=> break,
			Event::CursorUp 		=> game.move_cursor(0, -1),
			Event::CursorDown 		=> game.move_cursor(0, 1),
			Event::CursorLeft 		=> game.move_cursor(-1, 0),
			Event::CursorRight 		=> game.move_cursor(1, 0),
			Event::CursorToX(x) 	=> game.set_cursor_x(x),
			Event::CursorToY(y) 	=> game.set_cursor_y(y),
			Event::Enter 			=> game.enter(),
			Event::CursorToPos(pos) => game.set_cursor_pos(pos),
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
