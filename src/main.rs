mod draw;

use termion::{
	screen::AlternateScreen,
	raw::IntoRawMode,
	input::TermRead,
	event::Key,
	cursor,
	clear,
};
use std::io::{self, Write};

use self::draw::*;
use self::GameState::*;

/// One field on a game board
#[derive(PartialEq, Eq)]
pub struct Piece {
	big: bool,
	dark: bool,
	round: bool,
	flat: bool,
}

/// The state the game is in
#[derive(PartialEq, Eq)]
enum GameState {
	SelectPiece,
	PlacePiece,
}

/// A game board (array of rows)
type Board = [[Option<Piece>; 4]; 4];

/// The text above the pieces board
const PIECES_BOARD_LABEL: &'static str = "Available Pieces";
// const PIECES_BOARD_LABEL: &'static str = "Verfügbare Steine";

fn main() -> io::Result<()> {
	// prepare input/output
	let stdout = io::stdout().into_raw_mode()?;
	let mut stdout = AlternateScreen::from(stdout);
	let stdin = io::stdin();
	write!(stdout, "{}{}{}", cursor::Hide, cursor::Goto(2,2), clear::All)?;

	// game state
	let mut main_board:   Board = Board::default();
	let mut pieces_board: Board = full_board();
	let mut curr = (0,0);
	let mut selected_piece: Option<Piece> = None;
	let mut state = GameState::SelectPiece;

	// 2nd board left. 2: offset main board. +1: last edge, +3: distance to main board
	let pieces_board_left = 4*(FIELD_W+1) + 10 + 1 + 10;

	// initial drawing
	draw_board(&mut stdout, &main_board, (9,9), true)?;
	draw_label(&mut stdout, (pieces_board_left + 3, 1), 25, PIECES_BOARD_LABEL)?;
	write!(stdout, "{}", cursor::Goto(pieces_board_left, 2))?;
	draw_board(&mut stdout, &pieces_board, curr, false)?;

	// game loop
	for c in stdin.keys() {
		match c? {
			Key::Esc | Key::Char('q') => break,
			Key::Up    => curr.1 = (curr.1 + 4 - 1) % 4,
			Key::Down  => curr.1 = (curr.1 + 4 + 1) % 4,
			Key::Left  => curr.0 = (curr.0 + 4 - 1) % 4,
			Key::Right => curr.0 = (curr.0 + 4 + 1) % 4,
			Key::Char(n) if "abcd".contains(n) => {
				curr.0 = "abcd".find(n).unwrap() as u16;
			},
			Key::Char(n) if "1234".contains(n) => {
				curr.0 = "1234".find(n).unwrap() as u16;
			},
			Key::Char('\n') if state == SelectPiece  => { // Enter, SelectPiece mode
				selected_piece = pieces_board[curr.1 as usize][curr.0 as usize].take();
				if selected_piece.is_some() {
					state = PlacePiece;
				}
			},
			Key::Char('\n') if state == PlacePiece => { // Enter, PlacePiece
				if main_board[curr.1 as usize][curr.0 as usize].is_none() {
					main_board[curr.1 as usize][curr.0 as usize] = selected_piece.take();
					state = SelectPiece;
				}
			},
			_ => {},
		}

		// redraw boards and piece preview
		let main_curr = if state == PlacePiece { curr } else { (9,9) };
		write!(stdout, "{}{}", cursor::Goto(2,2), clear::All)?;
		draw_board(&mut stdout, &main_board, main_curr, true)?;

		let pieces_curr = if state == SelectPiece { curr } else { (9,9) };
		write!(stdout, "{}", cursor::Goto(pieces_board_left, 2))?;
		draw_board(&mut stdout, &pieces_board, pieces_curr, false)?;

		draw_label(&mut stdout, (pieces_board_left + 3, 1), 25, PIECES_BOARD_LABEL)?;

		if state == PlacePiece {
			draw_selected_piece(&mut stdout, (35, 3), &selected_piece)?;
		}
	}

	// restore state and exit
	write!(stdout, "{}{}", cursor::Show, cursor::Down(1))?;
	stdout.flush()?;
	Ok(())
}

/// create a board with all different stones on it
fn full_board() -> Board {
	let mut board: Board = Default::default();

	for x in 0..4 {
		for y in 0..4 {
			let big	  = x <= 1;
			let dark  = x % 2 == 0;
			let round = y <= 1;
			let flat  = y % 2 == 0;

			board[x][y] = Some(Piece { big, dark, round, flat });
		}
	}
	// board[0][0] = None;

	board
}
