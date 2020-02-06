use std::io::{self, Write};
use termion::{
	cursor::{
		self,
		DetectCursorPos,
		Goto,
	},
	style,
	color,
};
use crate::{Board, Field};

pub const FIELD_W: u16 = 5; // only used in main.rs
const LINE_H  : &'static str = "─────";
const LINE_H_S: &'static str = "═════";

fn field_strs(field: &Field) -> [String; 3] {
	if let Field::Occupied { big, dark, round, flat } = field {
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

fn draw_h_delim<W: Write>(mut out: W, y: u16, curr: (u16, u16)) -> io::Result<()> {
	// mark border if cell above or below is selected
	let ysel = curr.1 == y || curr.1 + 1 == y;

	// left corner
	if ysel && curr.0 == 0 { write!(out, "{}", style::Invert)? }
	write!(out, "{}", match y {
		0 => if ysel && curr.0 == 0 { "╔" } else { "┌" },
		4 => if ysel && curr.0 == 0 { "╚" } else { "└" },
		_ => if ysel && curr.0 == 0 { "╠" } else { "├" },
	})?;

	for x in 0..3 {
		// line segment
		write!(out, "{}", if ysel && curr.0 == x { LINE_H_S } else { LINE_H })?;

		// inner junction
		if ysel && curr.0 == x+1 { write!(out, "{}", style::Invert)? }
		let xsel = curr.0 == x || curr.0 == x + 1;
		write!(out, "{}", match y {
			0 => if ysel && xsel { "╦" } else { "┬" },
			4 => if ysel && xsel { "╩" } else { "┴" },
			_ => if ysel && xsel { "╬" } else { "┼" },
		})?;
		if ysel && curr.0 == x { write!(out, "{}", style::Reset)? }
	}
	write!(out, "{}", if ysel && curr.0 == 3 { LINE_H_S } else { LINE_H })?;

	// right corner
	write!(out, "{}{}", match y {
		0 => if ysel && curr.0 == 3 { "╗" } else { "┐" },
		4 => if ysel && curr.0 == 3 { "╝" } else { "┘" },
		_ => if ysel && curr.0 == 3 { "╣" } else { "┤" },
	}, style::Reset)
}
pub fn draw_board<W: Write>(mut out: W, board: &Board, curr: (u16, u16)) -> io::Result<()> {
	let (cursor_x, cursor_y) = out.cursor_pos()?;
	// adjust cursor y to make room for outer border
	let cursor_y = cursor_y+1;

	write!(out, "╔═══════════════════════════╗")?;

	for y in 0..4 {
		write!(out, "{}", Goto(cursor_x, cursor_y + 4*y))?;
		write!(out, "║ ")?;
		draw_h_delim(&mut out, y, curr)?;
		write!(out, " ║")?;

		for rows in 0..3 {
			write!(out, "{}", Goto(cursor_x, cursor_y + 4*y + rows + 1))?;
			write!(out, "║ ")?;
			for x in 0..4 {
				// left/middle cell border
				if curr.1 == y && (curr.0 == x || curr.0 + 1 == x) {
					write!(out, "{}║{}", style::Invert, style::Reset)?;
				} else {
					write!(out, "│")?;
				}

				// cell content
				write!(out, "{}", field_strs(&board[y as usize][x as usize])[rows as usize])?;

				// right-most cell border
				if x == 3 {
					if curr == (x,y) {
						write!(out, "{}║{}", style::Invert, style::Reset)?;
					} else {
						write!(out, "│")?;
					}
					write!(out, " ║")?;
				}
			}
			// out.flush()?;
		}
	}
	write!(out, "{}", Goto(cursor_x, cursor_y + 4*4))?;
	write!(out, "║ ")?;
	draw_h_delim(&mut out, 4, curr)?;
	write!(out, " ║{}╚═══════════════════════════╝", Goto(cursor_x, cursor_y + 4*4 + 1))?;

	out.flush()
}

pub fn draw_board2<W: Write>(mut out: W, board: &Board) -> io::Result<()> {
	let (cursor_x, cursor_y) = out.cursor_pos()?;

	let g = color::Green.fg_str();
	let r = color::Reset.fg_str();

	// write!(out, "{}┌──A──┬──B──┬──C──┬──D──┐", g)?;
	write!(out, "{}{}   A     B     C     D   ", g, cursor::Up(1))?;
	write!(out, "{}┌─────┬─────┬─────┬─────┐", Goto(cursor_x, cursor_y))?;
	for y in 0..4 {
		write!(out, "{}", Goto(cursor_x, cursor_y + 4*y))?;
		if y > 0 {
			write!(out, "{}├─────┼─────┼─────┼─────┤", g)?;
		}
		
		write!(out, "{}{}{}", Goto(cursor_x-1,cursor_y+4*y+2), g, 4 - y)?;
		write!(out, "{}{}{}", Goto(cursor_x+25,cursor_y+4*y+2), 4 - y, r)?;

		for rows in 0..3 {
			write!(out, "{}", Goto(cursor_x, cursor_y + 4*y + rows + 1))?;
			for x in 0..4 {
				// left/middle cell border
				write!(out, "{}│{}", g, r)?;

				// cell content
				write!(out, "{}", field_strs(&board[y as usize][x as usize])[rows as usize])?;

				// right-most cell border
				if x == 3 {
					write!(out, "{}│", g)?;
				}
			}
		}
	}
	// write!(out, "{}{}└──A──┴──B──┴──C──┴──D──┘{}", Goto(cursor_x, cursor_y + 4*4), g, r)?;
	write!(out, "{}{}└─────┴─────┴─────┴─────┘", Goto(cursor_x, cursor_y + 4*4), g)?;
	write!(out, "{}   A     B     C     D   {}", Goto(cursor_x, cursor_y + 4*4 + 1), r)?;

	out.flush()
}
