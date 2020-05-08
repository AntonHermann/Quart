#![warn(missing_docs)]
//! Simple board game as TUI

/// Contains the Terminal User Interface
pub mod gui;
/// Contains the User Interface State
pub mod ui_state;

use std::io;
use quart_lib::{board::*, Game, GameState::*};
use self::gui::{Gui, Event};
use self::ui_state::UiState;

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
        .directory(     concat!(env!("CARGO_MANIFEST_DIR"), "/logs"))
        .create_symlink(concat!(env!("CARGO_MANIFEST_DIR"), "/log.log"))
        .format(flexi_logger::default_format)
        .start()
        .unwrap();

    // game state
    let mut ui_state = UiState::new(Game::new());
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
    gui.draw(&ui_state)?;

	// TODO: switch to gui.poll_event() and gui::Event
    // game loop
    while let Some(event) = gui.poll_event(&ui_state) {
        match event {
			Event::Exit 			=> break,
			Event::CursorUp 		=> ui_state.move_cursor(0, -1),
			Event::CursorDown 		=> ui_state.move_cursor(0, 1),
			Event::CursorLeft 		=> ui_state.move_cursor(-1, 0),
			Event::CursorRight 		=> ui_state.move_cursor(1, 0),
			Event::CursorToX(x) 	=> ui_state.set_cursor_x(x),
			Event::CursorToY(y) 	=> ui_state.set_cursor_y(y),
			Event::Enter 			=> ui_state.enter(),
			Event::CursorToPos(pos) => ui_state.set_cursor_pos(pos),
        }

        if ui_state.game.check() {
	        log::info!("Game Over: {:?}", ui_state.game.game_over_info);
            // GAME OVER
            // break;
        }

        // redraw boards and piece preview
        gui.draw(&ui_state)?;
    }
    log::trace!("After game loop");

    std::mem::drop(gui);

	log::info!("End, {:?}", ui_state.game);
    if ui_state.game.state == GameOver {
        println!("+++ GAME OVER +++");
        println!("Player {} lost", ui_state.game.player_turn);
        println!("{:?}", ui_state.game.game_over_info);
    }

    Ok(())
}
