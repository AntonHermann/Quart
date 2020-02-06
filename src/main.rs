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

// type Map = Vec<Vec<Field>>;
type Map = [[Field; 4]; 4];

fn main() -> io::Result<()> {
	let map: Map = test_map();
	let stdout = io::stdout().into_raw_mode()?;
	let mut stdout = AlternateScreen::from(stdout);
	let stdin = io::stdin();
	write!(stdout, "{}{}{}", cursor::Hide, cursor::Goto(2,2), clear::All)?;

	let mut curr = (1,1);

	draw_map(&mut stdout, &map, curr)?;
	for c in stdin.keys() {
		match c? {
			Key::Up    => curr.1 = (curr.1 + 4 - 1) % 4,
			Key::Down  => curr.1 = (curr.1 + 4 + 1) % 4,
			Key::Left  => curr.0 = (curr.0 + 4 - 1) % 4,
			Key::Right => curr.0 = (curr.0 + 4 + 1) % 4,
			Key::Esc => break,
			_ => {},
		}
		write!(stdout, "{}{}", cursor::Goto(2,2), clear::All)?;
		draw_map(&mut stdout, &map, curr)?;
	}

	// std::mem::drop(out);

	write!(stdout, "{}{}", cursor::Show, cursor::Down(1))?;
	stdout.flush()?;
	Ok(())
}

fn test_map() -> Map {
	let mut map: Map = Default::default();

	for x in 0..4 {
		for y in 0..4 {
			let big	  = x <= 1;
			let dark  = x % 2 == 0;
			let round = y <= 1;
			let flat  = y % 2 == 0;

			map[x][y] = Field::Occupied { big, dark, round, flat };
		}
	}

	map
}
