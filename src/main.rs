use termion::screen::AlternateScreen;
use std::io::{self, Write};

enum Field {
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

fn draw_h_delim<W: Write>(out: &mut W, y: usize, curr: (usize, usize)) -> io::Result<()> {
	writeln!(out,
		" {l}───{m}───{m}───{m}───{r}",
		l = match y { 0 => "┌", 4 => "└", _ => "├" },
		m = match y { 0 => "┬", 4 => "┴", _ => "┼" },
		r = match y { 0 => "┐", 4 => "┘", _ => "┤" },
	)
}
fn draw_map<W: Write>(out: &mut W, map: &Map, curr: (usize, usize)) -> io::Result<()> {
	writeln!(out)?; // start with empty line
	for y in 0..4 {
		draw_h_delim(out, y, curr)?;
		for rows in 0..3 {
			write!(out, " ")?; // 1 space to the left
			for x in 0..4 {
				// left/middle cell border
				if curr.1 == y && (curr.0 == x || curr.0 + 1 == x) {
					write!(out, "║")?;
				} else {
					write!(out, "│")?;
				}

				// cell content
				write!(out, "{}", map[y][x].strs()[rows])?;

				// right-most cell border
				if x == 3 {
					write!(out, "{}", if curr == (x,y) { "║" } else {"│"})?
				}
			}
			writeln!(out)?;
		}
	}
	draw_h_delim(out, 4, curr)?;
	writeln!(out)
}

fn main() {
	let map: Map = Default::default();
	// let mut screen = AlternateScreen::from(std::io::stdout());
	let mut screen = std::io::stdout();
	let curr = (0,0);

	let _ = draw_map(&mut screen, &map, curr);

	// std::thread::sleep(std::time::Duration::from_secs(5));

	std::mem::drop(screen);
}
