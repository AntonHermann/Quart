use std::io::{self, Write};
use termion::{
	cursor::Goto,
	style,
	color,
};
use crate::{Board, Piece, BPos};

/// The horizontal line used for drawing the board
const LINE_H  : &str = "─────";
/// The horizontal line used for drawing the board - selected
const LINE_H_S: &str = "═════";

/// A position on the screen
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct SPos { pub x: u16, pub y: u16 }
impl SPos {
	pub fn _new(x: u16, y: u16) -> Self {
		Self { x, y }
	}
	/// Create a termion [`Goto`] struct to simplify placing the terminal cursor
	/// at the specified position
	pub fn to_goto(self) -> Goto {
		Goto(self.x, self.y)
	}
	/// Creates a copy of self, translated by `dx`/`dy`.
	pub fn translated_i32(self, dx: i32, dy: i32) -> Self {
		Self {
			x: (self.x as i32 + dx) as u16,
			y: (self.y as i32 + dy) as u16,
		}
	}
	/// Creates a copy of self, translated by `dx`/`dy`.
	/// Here `dx`, `dy` are unsigned, so only steps in down/right direction are possible
	pub fn translated(self, dx: u16, dy: u16) -> Self {
		Self {
			x: self.x + dx,
			y: self.y + dy,
		}
	}
}

/// Creates the visual representation of a field, split into 3 lines
fn field_strs(field: Option<Piece>) -> [String; 3] {
	if let Some(Piece { big, dark, round, flat }) = field {
		let d = if dark  { color::Fg(color::Red).to_string() } else { "".into() };
		let (rl,rr) = if round { ("(",")") } else { ("|","|") };
		let f1 = if flat  { "---" } else { "\\ /" };
		let f2 = if flat  { "   " } else { " O "  };
		let reset = color::Fg(color::Reset).to_string() + style::Reset.as_ref();

		// (\ /) (---) |---| |\ /|
		// ( O ) (   ) |   | | O | (\ /) (---) |---| |\ /|
		// (   ) (   ) |   | |   | ( O ) (   ) |   | | O |
		//  BRf   BRF   BrF   Brf   bRf   bRF   brF   brf

		let roof  = format!("{}{}{}{}{}" , d, rl, f1, rr, reset);
		let base1 = format!("{}{}{}{}{}" , d, rl, f2, rr, reset);
		let base2 = format!("{}{}   {}{}", d, rl,     rr, reset);
		let empty = "     ".into();

		if big {
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
	let v_border = if main { "║" } else { " " }; // vertical border

	write!(out, "{}", pos.to_goto())?;

	if main {
		write!(out, "╔════A═════B═════C═════D════╗")?;
	} else {
		write!(out, "     A     B     C     D")?;
	}

	for y in 0..4 {
		write!(out, "{}{} ", pos.translated(0,4*y+1).to_goto(), v_border)?;
		draw_h_delim(&mut out, y, curr)?;
		write!(out, " {}", v_border)?;

		for rows in 0..3 {
			write!(out, "{}", pos.translated(0,4*y+rows+2).to_goto())?;
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
				write!(out, "{}", field_strs(board.0[y as usize][x as usize])[rows as usize])?;

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
	write!(out, "{}{} ", pos.translated(0,4*4+1).to_goto(), v_border)?;
	draw_h_delim(&mut out, 4, curr)?;
	if main {
		write!(out, " ║{}╚════A═════B═════C═════D════╝", pos.translated(0,4*4+2).to_goto())?;
	} else {
		write!(out, "{}     A     B     C     D", pos.translated(0,4*4+2).to_goto())?;
	}

	out.flush()
}

/// Draw a small box to show which piece is currently selected
pub fn draw_selected_piece<W: Write>(mut out: W, pos: SPos, piece: Option<Piece>) -> io::Result<()> {
	write!(out, "{}┌PLACE┐", pos.to_goto())?;
	let piece_strs = field_strs(piece);
	for row in 0..3 {
		write!(out, "{}│{}│", pos.translated(0,row+1).to_goto(), piece_strs[row as usize])?;
	}
	write!(out, "{}└─────┘", pos.translated(0,4).to_goto())?;
	out.flush()
}

/// Draw a label, with origin at x,y, such that is is centered inside the given width
/// label shouldn't be wider thatn total_width
pub fn draw_label<W: Write>(mut out: W, pos: SPos, total_width: u16, label: &str) -> io::Result<()> {
	let offset = (total_width / 2) - (label.len() as u16 / 2);
	write!(out, "{}{}", pos.translated(offset,0).to_goto(), label)?;
	out.flush()
}
