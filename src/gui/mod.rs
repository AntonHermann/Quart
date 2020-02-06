mod util;

use termion::{
	// screen::AlternateScreen,
	// raw::IntoRawMode,
	// input::TermRead,
	// event::Key,
	// cursor,
	clear,
};
use std::io::{self, Write};
pub use self::util::*;
use crate::game::{Game, GameState::*, BPos};

/// The text above the pieces board
const PIECES_BOARD_LABEL: &'static str = "Available Pieces";

/// A layout determining what GUI-Element goes where
pub struct Layout {
	main_board: SPos,
	curr_piece: SPos,
	pieces_board: SPos,
	player_turn_label: SPos,
}
const LAYOUT_WIDE: Layout = Layout {
	main_board:        SPos { x:  2, y:  2},
	curr_piece: 	   SPos { x: 35, y:  3},
	pieces_board: 	   SPos { x: 45, y:  2},
	player_turn_label: SPos { x:  4, y: 22},
};
impl Layout {
	pub fn _wide() -> Self {
		LAYOUT_WIDE
	}
}
pub fn draw_gui<W: Write>(mut out: W, game: &Game, layout: Option<&Layout>) -> io::Result<()> {
	let layout = layout.unwrap_or(&LAYOUT_WIDE);

	write!(out, "{}", clear::All)?;

	let main_curr = if game.state == PlacePiece { game.cursor_pos } else { BPos::invalid() };
	draw_board(&mut out, layout.main_board, &game.main_board, main_curr, true)?;

	let pieces_curr = if game.state == SelectPiece { game.cursor_pos } else { BPos::invalid() };
	draw_board(&mut out, layout.pieces_board, &game.pieces_board, pieces_curr, false)?;

	let label_pos = layout.pieces_board.translated_i32(2, -1);
	draw_label(&mut out, label_pos, 25, PIECES_BOARD_LABEL)?;

	let player_turn_str = format!("Player {}'s turn!", game.player_turn);
	draw_label(&mut out, layout.player_turn_label, 25, &player_turn_str)?;

	if game.state == PlacePiece {
		draw_selected_piece(&mut out, layout.curr_piece, &game.selected_piece)?;
	}
	Ok(())
}
