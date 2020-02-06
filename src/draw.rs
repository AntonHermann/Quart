use std::io::{self, Write};
use termion::{
	cursor::{
		DetectCursorPos,
		Goto,
	},
	style,
	color,
};
use crate::{Map, Field};

const LINE_H  : &'static str = "─────";
const LINE_H_S: &'static str = "═════";

fn field_strs(field: &Field) -> [String; 3] {
	if let Field::Occupied { big, dark, round, flat } = field {
		let d = if *dark  { color::Fg(color::Red).to_string() } else { "".into() };
		let (rl,rr) = if *round { ("(",")") } else { ("|","|") };
		let f = if *flat  { "-" } else { "O" };
		let reset = color::Fg(color::Reset).to_string() + style::Reset.as_ref();

		// (-O-) (---) |---| |-O-|
		// (   ) (   ) |   | |   | (-O-) (---) |---| |-O-|
		// (   ) (   ) |   | |   | (   ) (   ) |   | |   |
		//  BRf   BRF   BrF   Brf   bRf   bRF   brF   brf

		let roof = format!("{}{}-{}-{}{}", d, rl, f, rr, reset);
		let base = format!("{}{}   {}{}" , d, rl,    rr, reset);
		let empty = "     ".into();

		if *big {
			[roof,base.clone(),base]
		} else {
			[empty,roof,base]
		}
	} else {
		[
			"\\   /".to_owned(),
			" ... ".to_owned(),
			"/   \\".to_owned(),
		]
	}
}

fn draw_h_delim<W: Write>(mut out: W, y: u16, curr: (u16, u16)) -> io::Result<()> {
	// mark border if cell above or below is selected
	let ysel = curr.1 == y || curr.1 + 1 == y;

	// left corner
	write!(out, "{}", match y {
		0 => if ysel && curr.0 == 0 { "╔" } else { "┌" },
		4 => if ysel && curr.0 == 0 { "╚" } else { "└" },
		_ => if ysel && curr.0 == 0 { "╠" } else { "├" },
	})?;

	for x in 0..3 {
		// line segment
		write!(out, "{}", if ysel && curr.0 == x { LINE_H_S } else { LINE_H })?;

		// inner junction
		let xsel = curr.0 == x || curr.0 == x + 1;
		write!(out, "{}", match y {
			0 => if ysel && xsel { "╦" } else { "┬" },
			4 => if ysel && xsel { "╩" } else { "┴" },
			_ => if ysel && xsel { "╬" } else { "┼" },
		})?;
	}
	write!(out, "{}", if ysel && curr.0 == 3 { LINE_H_S } else { LINE_H })?;

	// right corner
	write!(out, "{}", match y {
		0 => if ysel && curr.0 == 3 { "╗" } else { "┐" },
		4 => if ysel && curr.0 == 3 { "╝" } else { "┘" },
		_ => if ysel && curr.0 == 3 { "╣" } else { "┤" },
	})
}
pub fn draw_map<W: Write>(mut out: W, map: &Map, curr: (u16, u16)) -> io::Result<()> {
	let (cursor_x, cursor_y) = out.cursor_pos()?;

	for y in 0..4 {
		write!(out, "{}", Goto(cursor_x, cursor_y + 4*y))?;
		draw_h_delim(&mut out, y, curr)?;

		for rows in 0..3 {
			write!(out, "{}", Goto(cursor_x, cursor_y + 4*y + rows + 1))?;
			for x in 0..4 {
				// left/middle cell border
				if curr.1 == y && (curr.0 == x || curr.0 + 1 == x) {
					write!(out, "║")?;
				} else {
					write!(out, "│")?;
				}

				// cell content
				write!(out, "{}", field_strs(&map[y as usize][x as usize])[rows as usize])?;

				// right-most cell border
				if x == 3 {
					write!(out, "{}", if curr == (x,y) { "║" } else {"│"})?
				}
			}
			// out.flush()?;
		}
	}
	write!(out, "{}", Goto(cursor_x, cursor_y + 4*4))?;
	draw_h_delim(&mut out, 4, curr)?;

	out.flush()
}
