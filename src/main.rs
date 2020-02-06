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
#[derive(Default)]
pub struct Board(pub [[Option<Piece>; 4]; 4]);
impl std::ops::Index<BPos> for Board {
	type Output = Option<Piece>;
	fn index(&self, pos: BPos) -> &Self::Output {
		&self.0[pos.y as usize][pos.x as usize]
	}
}
impl std::ops::IndexMut<BPos> for Board {
	fn index_mut(&mut self, pos: BPos) -> &mut Self::Output {
		&mut self.0[pos.y as usize][pos.x as usize]
	}
}
impl Board {
	/// create a board with all different stones on it
	pub fn full() -> Board {
		let mut board: Board = Default::default();
		for x in 0..4 {
			for y in 0..4 {
				let big	  = x <= 1;
				let dark  = x % 2 == 0;
				let round = y <= 1;
				let flat  = y % 2 == 0;

				board.0[x][y] = Some(Piece { big, dark, round, flat });
			}
		}
		board
	}
}

/// A position on the board
#[derive(Copy, Clone)]
pub struct BPos { pub x: u16, pub y: u16 }
impl BPos {
	pub fn new(x: u16, y: u16) -> Self {
		Self { x, y }
	}
	pub fn invalid() -> Self {
		// Self { x: std::u16::MAX, y: std::u16::MAX }
		Self { x: 99, y: 99 }
	}
	pub fn is_invalid(&self) -> bool {
		// self.x == std::u16::MAX && self.y == std::u16::MAX
		self.x == 99 && self.y == 99
	}
}

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
	let mut pieces_board: Board = Board::full();
	let mut curr = BPos::new(0,0);
	let mut selected_piece: Option<Piece> = None;
	let mut state = GameState::SelectPiece;

	// TODO: when the terminal is too small, move the 2nd board under the first one
	// 2nd board left. 2: offset main board. +1: last edge, +3: distance to main board
	let pieces_board_left = 4*(FIELD_W+1) + 10 + 1 + 10;

	// initial drawing
	draw_board(&mut stdout, SPos::new(2,2), &main_board, BPos::invalid(), true)?;
	draw_label(&mut stdout, SPos::new(pieces_board_left + 3, 1), 25, PIECES_BOARD_LABEL)?;
	draw_board(&mut stdout, SPos::new(pieces_board_left,2), &pieces_board, curr, false)?;

	// game loop
	for c in stdin.keys() {
		match c? {
			Key::Esc | Key::Char('q') => break,
			Key::Up    => curr.y = (curr.y + 4 - 1) % 4,
			Key::Down  => curr.y = (curr.y + 4 + 1) % 4,
			Key::Left  => curr.x = (curr.x + 4 - 1) % 4,
			Key::Right => curr.x = (curr.x + 4 + 1) % 4,
			Key::Char(n) if "abcd".contains(n) => {
				curr.x = "abcd".find(n).unwrap() as u16;
			},
			Key::Char(n) if "1234".contains(n) => {
				curr.y = 3 - "1234".find(n).unwrap() as u16;
			},
			Key::Char('\n') if state == SelectPiece  => { // Enter, SelectPiece mode
				selected_piece = pieces_board[curr].take();
				if selected_piece.is_some() {
					state = PlacePiece;
				}
			},
			Key::Char('\n') if state == PlacePiece => { // Enter, PlacePiece
				if main_board[curr].is_none() {
					main_board[curr] = selected_piece.take();
					state = SelectPiece;
				}
			},
			_ => {},
		}

		// redraw boards and piece preview
		let main_curr = if state == PlacePiece { curr } else { BPos::invalid() };
		write!(stdout, "{}{}", cursor::Goto(2,2), clear::All)?;
		draw_board(&mut stdout, SPos::new(2,2), &main_board, main_curr, true)?;

		let pieces_curr = if state == SelectPiece { curr } else { BPos::invalid() };
		write!(stdout, "{}", cursor::Goto(pieces_board_left, 2))?;
		draw_board(&mut stdout, SPos::new(pieces_board_left,2), &pieces_board, pieces_curr, false)?;

		draw_label(&mut stdout, SPos::new(pieces_board_left + 3, 1), 25, PIECES_BOARD_LABEL)?;

		if state == PlacePiece {
			draw_selected_piece(&mut stdout, SPos::new(35, 3), &selected_piece)?;
		}
	}

	// restore state and exit
	write!(stdout, "{}{}", cursor::Show, cursor::Down(1))?;
	stdout.flush()?;
	Ok(())
}


