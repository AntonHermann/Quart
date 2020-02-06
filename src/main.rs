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

// type Board = Vec<Vec<Field>>;
type Board = [[Field; 4]; 4];

fn main() -> io::Result<()> {
	let stdout = io::stdout().into_raw_mode()?;
	let mut stdout = AlternateScreen::from(stdout);
	let stdin = io::stdin();
	write!(stdout, "{}{}{}", cursor::Hide, cursor::Goto(2,2), clear::All)?;

	let main_board:   Board = full_board();
	let pieces_board: Board = full_board();

	let mut curr = (1,1);
	// 2nd board left. 2: offset main board. +1: last edge, +3: distance to main board
	let pieces_board_left = 2 + 4*(FIELD_W+1) + 1 + 10;

	draw_board(&mut stdout, &main_board, curr)?;
	write!(stdout, "{}", cursor::Goto(pieces_board_left, 3))?;
	draw_board2(&mut stdout, &pieces_board)?;
	for c in stdin.keys() {
		match c? {
			Key::Up    => curr.1 = (curr.1 + 4 - 1) % 4,
			Key::Down  => curr.1 = (curr.1 + 4 + 1) % 4,
			Key::Left  => curr.0 = (curr.0 + 4 - 1) % 4,
			Key::Right => curr.0 = (curr.0 + 4 + 1) % 4,
			Key::Esc | Key::Char('q') => break,
			_ => {},
		}
		write!(stdout, "{}{}", cursor::Goto(2,2), clear::All)?;
		draw_board(&mut stdout, &main_board, curr)?;
		write!(stdout, "{}", cursor::Goto(pieces_board_left, 3))?;
		draw_board2(&mut stdout, &pieces_board)?;
	}

	write!(stdout, "{}{}", cursor::Show, cursor::Down(1))?;
	stdout.flush()?;
	Ok(())
}

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
