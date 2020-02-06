use std::io::{self, Write};
use crate::Map;

fn draw_h_delim<W: Write>(out: &mut W, y: usize, curr: (usize, usize)) -> io::Result<()> {
	// mark border if cell above or below is selected
	let ysel = curr.1 == y || curr.1 + 1 == y;

	// left corner
	write!(out, " {}", match y {
		0 => if ysel && curr.0 == 0 { "╔" } else { "┌" },
		4 => if ysel && curr.0 == 0 { "╚" } else { "└" },
		_ => if ysel && curr.0 == 0 { "╠" } else { "├" },
	})?;

	for x in 0..3 {
		// line segment
		write!(out, "{}", if ysel && curr.0 == x { "═══" } else { "───" })?;

		// inner junction
		let xsel = curr.0 == x || curr.0 == x + 1;
		write!(out, "{}", match y {
			0 => if ysel && xsel { "╦" } else { "┬" },
			4 => if ysel && xsel { "╩" } else { "┴" },
			_ => if ysel && xsel { "╬" } else { "┼" },
		})?;
	}
	write!(out, "{}", if ysel && curr.0 == 3 { "═══" } else { "───" })?;

	// right corner
	writeln!(out, "{}", match y {
		0 => if ysel && curr.0 == 3 { "╗" } else { "┐" },
		4 => if ysel && curr.0 == 3 { "╝" } else { "┘" },
		_ => if ysel && curr.0 == 3 { "╣" } else { "┤" },
	})
}
pub fn draw_map<W: Write>(out: &mut W, map: &Map, curr: (usize, usize)) -> io::Result<()> {
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
