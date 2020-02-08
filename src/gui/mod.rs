mod util;

use termion::{clear, cursor};
use std::io::{self, Write};
pub use self::util::*;
use crate::game::{Game, GameState::*, BPos};

/// The text above the pieces board
const PIECES_BOARD_LABEL: &str = "Available Pieces";

/// A layout determining what GUI-Element goes where
pub struct Layout {
	main_board: SPos,
	curr_piece: SPos,
	pieces_board: SPos,
	status_label: SPos,
}
/// A default, wide screen layout
const LAYOUT_WIDE: Layout = Layout {
	main_board:	  SPos { x:  2, y:  2},
	curr_piece:	  SPos { x: 35, y:  3},
	pieces_board: SPos { x: 45, y:  2},
	status_label: SPos { x:  4, y: 22},
};
impl Layout {
	/// A default, wide screen layout
	pub fn _wide() -> Self {
		LAYOUT_WIDE
	}
}
/// This function brings the whole game to live. Here the game scene is drawn
pub fn draw_gui<W: Write>(mut out: W, game: &Game, layout: Option<&Layout>) -> io::Result<()> {
	let layout = layout.unwrap_or(&LAYOUT_WIDE);

	write!(out, "{}", clear::All)?;

	let main_board_focus = game.state == PlacePiece || game.state == GameOver;

	let main_curr = if main_board_focus { game.cursor_pos } else { BPos::invalid() };
	draw_board(&mut out, layout.main_board, &game.main_board, main_curr, true)?;

	let pieces_curr = if !main_board_focus { game.cursor_pos } else { BPos::invalid() };
	draw_board(&mut out, layout.pieces_board, &game.pieces_board, pieces_curr, false)?;

	let label_pos = layout.pieces_board.translated_i32(2, -1);
	draw_label(&mut out, label_pos, 25, PIECES_BOARD_LABEL)?;

	let status_str = if game.is_over() {
		format!(
			"Game over because of {}",
			game.game_over_info.as_ref().unwrap().property)
	} else {
		format!("Player {}'s turn!", game.player_turn)
	};
	draw_label(&mut out, layout.status_label, 25, &status_str)?;

	if game.state == PlacePiece {
		draw_selected_piece(&mut out, layout.curr_piece, game.selected_piece)?;
	}

	write!(out, "{}", cursor::Goto(0,1))?;
	out.flush()
}
