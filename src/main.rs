mod draw;

// use termion::screen::AlternateScreen;
// use std::io::{self, Write};

use self::draw::*;

pub enum Field {
	Empty,
	Occupied {
		big: bool,
		black: bool,
		rount: bool,
		flat: bool,
	}
}
impl Default for Field {
	fn default() -> Self {
		Field::Empty
	}
}
impl Field {
	pub fn strs(&self) -> [&'static str; 3] {
		match self {
			Field::Empty => ["\\ /"," . ","/ \\"],
			Field::Occupied{..} => ["xxx","xxx","xxx"],
		}
	}
}

// type Map = Vec<Vec<Field>>;
type Map = [[Field; 4]; 4];


fn main() {
	let map: Map = Default::default();
	// let mut screen = AlternateScreen::from(std::io::stdout());
	let mut screen = std::io::stdout();
	let curr = (1,1);

	let _ = draw_map(&mut screen, &map, curr);

	// std::thread::sleep(std::time::Duration::from_secs(5));

	std::mem::drop(screen);
}
