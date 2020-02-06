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

/// One field on a game board
pub enum Field {
	Empty,
	Occupied {
		big: bool,
		dark: bool,
		round: bool,
		flat: bool,
	}
}
impl Default for Field {
	fn default() -> Self {
		Field::Empty
	}
}

/// A game board
type Board = [[Field; 4]; 4];

/// The text above the pieces board
const PIECES_BOARD_LABEL: &'static str = "Verfügbare Steine";

fn main() -> io::Result<()> {
	// prepare input/output
	let stdout = io::stdout().into_raw_mode()?;
	let mut stdout = AlternateScreen::from(stdout);
	let stdin = io::stdin();
	write!(stdout, "{}{}{}", cursor::Hide, cursor::Goto(2,2), clear::All)?;

	// game state
	let main_board:   Board = Board::default();
	let pieces_board: Board = full_board();
	let mut curr = (1,1);

	// 2nd board left. 2: offset main board. +1: last edge, +3: distance to main board
	let pieces_board_left = 2 + 4*(FIELD_W+1) + 1 + 10;

	// initial drawing
	draw_board(&mut stdout, &main_board, curr, true)?;
	draw_label(&mut stdout, pieces_board_left + 3, 1, 25, PIECES_BOARD_LABEL)?;
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
			_ => {},
		}
		write!(stdout, "{}{}", cursor::Goto(2,2), clear::All)?;
		draw_board(&mut stdout, &main_board, curr, true)?;
		write!(stdout, "{}", cursor::Goto(pieces_board_left, 2))?;
		draw_board(&mut stdout, &pieces_board, (5,5), false)?;
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

			board[x][y] = Field::Occupied { big, dark, round, flat };
		}
	}
	// board[0][0] = Field::Empty;

	board
}
