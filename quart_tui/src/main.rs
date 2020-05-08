#![warn(missing_docs)]
//! Simple board game as TUI

/// Contains the Terminal User Interface
pub mod gui;
/// Contains the User Interface State
pub mod ui_state;

use quart_lib::{board::*, Game, GameState::*};
use self::gui::{Gui, Event};
use self::ui_state::UiState;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(feature = "ai_enemy")]
use quart_ai_enemy::*;

fn main() -> Result<()> {
	let res = run();
	log::info!("{:?}", res);
	res
}
fn run() -> Result<()> {
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

	#[cfg(feature = "ai_enemy")]
    let mut ai_agent: Box<dyn AiAgent> = get_ai_agent();

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
        }

		#[cfg(feature = "ai_enemy")] {
			use quart_lib::GameError;
			if !ui_state.game.is_over() && ui_state.game.player_turn == 2 {
		        gui.draw(&ui_state)?; // redraw boards and piece preview

		        loop { // we let the ai_agent try again and again until he does a valid move
					let (pos, piece) = ai_agent.play(&ui_state.game);
					match ui_state.game.place_piece(pos).and_then(|_| ui_state.game.select_next_piece(piece)) {
						Ok(()) => {
							// turn finished successfully
							ui_state.pieces_board.remove(piece);
							break
						},
						Err(GameError::GameIsOver) => break, // game over
						e @ Err(GameError::NoPieceSelected) => e?, // propagate critical error
						Err(_) => {}, // other GameErrors are less important
					}
		        }
	        }
		}

        // redraw boards and piece preview
        gui.draw(&ui_state)?;
    }
    log::trace!("After game loop");

    std::mem::drop(gui);

	log::info!("End, {:?}", ui_state.game);
    if ui_state.game.state == GameOver {
        println!("+++ GAME OVER +++");
        println!("Player {} won", ui_state.game.player_turn);
        println!("{:?}", ui_state.game.game_over_info);
    }

    Ok(())
}
