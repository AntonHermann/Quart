use std::io::{self, Write};
use termion::{
	cursor::{
		self,
		Goto,
	},
	style,
	color,
};
use crate::{Board, Piece, BPos};

/// The width of a single field
pub const FIELD_W: u16 = 5; // only used in main.rs
/// The horizontal line used for drawing the board
const LINE_H  : &'static str = "─────";
/// The horizontal line used for drawing the board - selected
const LINE_H_S: &'static str = "═════";

/// A position on the screen
#[derive(Copy, Clone)]
pub struct SPos { pub x: u16, pub y: u16 }
impl SPos {
	pub fn new(x: u16, y: u16) -> Self {
		Self { x, y }
	}
}

/// Creates the visual representation of a field, split into 3 lines
fn field_strs(field: &Option<Piece>) -> [String; 3] {
	if let Some(Piece { big, dark, round, flat }) = field {
		let d = if *dark  { color::Fg(color::Red).to_string() } else { "".into() };
		let (rl,rr) = if *round { ("(",")") } else { ("|","|") };
		let f1 = if *flat  { "---" } else { "\\ /" };
		let f2 = if *flat  { "   " } else { " O "  };
		let reset = color::Fg(color::Reset).to_string() + style::Reset.as_ref();

		// (\ /) (---) |---| |\ /|
		// ( O ) (   ) |   | | O | (\ /) (---) |---| |\ /|
		// (   ) (   ) |   | |   | ( O ) (   ) |   | | O |
		//  BRf   BRF   BrF   Brf   bRf   bRF   brF   brf

		let roof  = format!("{}{}{}{}{}" , d, rl, f1, rr, reset);
		let base1 = format!("{}{}{}{}{}" , d, rl, f2, rr, reset);
		let base2 = format!("{}{}   {}{}", d, rl,     rr, reset);
		let empty = "     ".into();

		if *big {
			[roof,base1,base2]
		} else {
			[empty,roof,base1]
		}
	} else {
		["     ".to_owned(),"     ".to_owned(),"     ".to_owned()]
	}
}

/// Draw a horizontal delimiter, possibly highlighting the current fields border
fn draw_h_delim<W: Write>(mut out: W, y: u16, curr: BPos) -> io::Result<()> {
	// mark border if cell above or below is selected
	let ysel = curr.y == y || curr.y + 1 == y;

	// left corner
	if ysel && curr.x == 0 { write!(out, "{}", style::Invert)? }
	write!(out, "{}", match y {
		0 => if ysel && curr.x == 0 { "╔" } else { "┌" },
		4 => if ysel && curr.x == 0 { "╚" } else { "└" },
		_ => if ysel && curr.x == 0 { "╠" } else { "├" },
	})?;

	for x in 0..3 {
		// line segment
		write!(out, "{}", if ysel && curr.x == x { LINE_H_S } else { LINE_H })?;

		// inner junction
		if ysel && curr.x == x+1 { write!(out, "{}", style::Invert)? }
		let xsel = curr.x == x || curr.x == x + 1;
		write!(out, "{}", match y {
			0 => if ysel && xsel { "╦" } else { "┬" },
			4 => if ysel && xsel { "╩" } else { "┴" },
			_ => if ysel && xsel { "╬" } else { "┼" },
		})?;
		if ysel && curr.x == x { write!(out, "{}", style::Reset)? }
	}
	write!(out, "{}", if ysel && curr.x == 3 { LINE_H_S } else { LINE_H })?;

	// right corner
	write!(out, "{}{}", match y {
		0 => if ysel && curr.x == 3 { "╗" } else { "┐" },
		4 => if ysel && curr.x == 3 { "╝" } else { "┘" },
		_ => if ysel && curr.x == 3 { "╣" } else { "┤" },
	}, style::Reset)
}

/// Draw a board at the current position, with [curr] as selected cells.
/// draws a big border if [main] is true
pub fn draw_board<W: Write>(mut out: W, pos: SPos, board: &Board, curr: BPos, main: bool) -> io::Result<()> {
	let SPos { x: cursor_x, y: cursor_y} = pos;

	let v_border = if main { "║" } else { " " }; // vertical border

	write!(out, "{}", Goto(cursor_x, cursor_y))?;

	if main {
		write!(out, "╔════A═════B═════C═════D════╗")?;
	} else {
		write!(out, "     A     B     C     D")?;
	}

	for y in 0..4 {
		write!(out, "{}{} ", Goto(cursor_x, cursor_y + 4*y + 1), v_border)?;
		draw_h_delim(&mut out, y, curr)?;
		write!(out, " {}", v_border)?;

		for rows in 0..3 {
			write!(out, "{}", Goto(cursor_x, cursor_y + 4*y + rows + 2))?;
			// draw big border or line number
			if rows == 1 {
				write!(out, "{} ", 4-y)?;
			} else {
				write!(out, "{} ", v_border)?;
			}
			for x in 0..4 {
				// left/middle cell border
				if curr.y == y && (curr.x == x || curr.x + 1 == x) {
					write!(out, "{}║{}", style::Invert, style::Reset)?;
				} else {
					write!(out, "│")?;
				}

				// cell content
				write!(out, "{}", field_strs(&board.0[y as usize][x as usize])[rows as usize])?;

				// right-most cell border
				if x == 3 {
					if (curr.x,curr.y) == (x,y) {
						write!(out, "{}║{}", style::Invert, style::Reset)?;
					} else {
						write!(out, "│")?;
					}

					// draw big border or line number
					if rows == 1 {
						write!(out, " {}", 4-y)?;
					} else {
						write!(out, " {}", v_border)?;
					}
				}
			}
		}
	}
	write!(out, "{}{} ", Goto(cursor_x, cursor_y + 4*4 + 1), v_border)?;
	draw_h_delim(&mut out, 4, curr)?;
	if main {
		write!(out, " ║{}╚════A═════B═════C═════D════╝", Goto(cursor_x, cursor_y + 4*4 + 2))?;
	} else {
		write!(out, "{}     A     B     C     D", Goto(cursor_x, cursor_y + 4*4 + 2))?;
	}

	out.flush()
}

pub fn draw_selected_piece<W: Write>(mut out: W, pos: SPos, piece: &Option<Piece>) -> io::Result<()> {
	write!(out, "{}┌PLACE┐", Goto(pos.x,pos.y))?;
	let piece_strs = field_strs(piece);
	for row in 0..3 {
		write!(out, "{}│{}│", Goto(pos.x,pos.y+row+1), piece_strs[row as usize])?;
	}
	write!(out, "{}└─────┘", Goto(pos.x,pos.y+4))?;
	out.flush()
}

/// Draw a label, with origin at x,y, such that is is centered inside the given width
/// label shouldn't be wider thatn total_width
pub fn draw_label<W: Write>(mut out: W, pos: SPos, total_width: u16, label: &str) -> io::Result<()> {
	let offset = (total_width / 2) - (label.len() as u16 / 2);
	write!(out, "{}{}", cursor::Goto(pos.x + offset, pos.y), label)?;
	out.flush()
}
