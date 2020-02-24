#![allow(dead_code)]

use crate::{Board, BPos, SPos};
use super::draw_piece;
use std::io::{self, Write};
use termion::style;

/// The horizontal line used for drawing the board
const LINE_H:   &str = "─────";
/// The horizontal line used for drawing the board - selected
const LINE_H_S: &str = "═════";

/// Draw a horizontal delimiter, possibly highlighting the current fields border
fn draw_h_delim<W: Write>(mut out: W, y: u16, cursor: BPos) -> io::Result<()> {
	log::trace!("draw_h_delim");
	
    // mark border if cell above or below is selected
    let ysel = cursor.y == y || cursor.y + 1 == y;

	let xsel = cursor.x == 0;
    // left corner
    if ysel && xsel {
        write!(out, "{}", style::Invert)?
    }
    if ysel && xsel {
	    write!(out, "{}", match y {
	        0 => "╔",
	        4 => "╚",
	        _ => "╠",
	    })?;
    } else {
	    write!(out, "{}", match y {
            0 => "┌",
            4 => "└",
            _ => "├",
        })?;
    }

    for x in 0..3 {
        // line segment
	    if ysel && cursor.x == x {
	        write!(out, "{}", LINE_H_S)?;
	    } else {
	        write!(out, "{}", LINE_H)?;
	    }

        // inner junction
        if ysel && cursor.x == x + 1 {
            write!(out, "{}", style::Invert)?
        }
        if ysel && (cursor.x == x || cursor.x == x + 1) {
	        write!(out, "{}", match y {
	            0 => "╦",
	            4 => "╩",
	            _ => "╬",
	        })?;
        } else {
	        write!(out, "{}", match y {
                0 => "┬",
                4 => "┴",
                _ => "┼",
            })?;
        }
        if ysel && cursor.x == x {
            write!(out, "{}", style::Reset)?
        }
    }
    if ysel && cursor.x == 3 {
	    write!(out, "{}", LINE_H_S)?;
    } else {
	    write!(out, "{}", LINE_H)?;
    }

    // right corner
    if ysel&& cursor.x == 3 {
	    write!(out, "{}", match y {
	        0 => "╗",
	        4 => "╝",
	        _ => "╣",
	    })?;
    } else {
	    write!(out, "{}", match y {
            0 => "┐",
            4 => "┘",
            _ => "┤",
        })?;
    }
 	write!(out, "{}", style::Reset)?;
 	log::trace!("draw_h_delim: end");
 	Ok(())
}

/// Draw a board at the current position, with `cursor` as selected cells.
/// draws a big border if `main` is true
pub fn draw_board<W: Write>(
    mut out: W,
    pos: SPos,
    board: &Board,
    cursor: BPos,
    sel: bool,
    main: bool,
    highlights: Vec<BPos>,
) -> io::Result<()> {
	log::trace!("draw_board");
    let cursor = if sel { cursor } else { BPos::new(99,99) };

    let v_border = if main { "║" } else { " " }; // vertical border

    write!(out, "{}", pos.to_goto())?;

    if main {
        write!(out, "╔════A═════B═════C═════D════╗")?;
    } else {
        write!(out, "     A     B     C     D")?;
    }

    for y in 0..4 {
        write!(
            out,
            "{}{} ",
            pos.to_goto_t(0, 4 * y + 1),
            v_border
        )?;
        draw_h_delim(&mut out, y, cursor)?;
        write!(out, " {}", v_border)?;

        for rows in 0..3 {
            write!(out, "{}", pos.to_goto_t(0, 4 * y + rows + 2))?;
            // draw big border or line number
            if rows == 1 {
                write!(out, "{} ", 4 - y)?;
            } else {
                write!(out, "{} ", v_border)?;
            }
            for x in 0..4 {
                // left/middle cell border
                if cursor.y == y && (cursor.x == x || cursor.x + 1 == x) {
                    write!(out, "{}║{}", style::Invert, style::Reset)?;
                } else {
                    write!(out, "│")?;
                }

                write!(out, "     ")?;

                // right-most cell border
                if x == 3 {
                    if (cursor.x, cursor.y) == (x, y) {
                        write!(out, "{}║{}", style::Invert, style::Reset)?;
                    } else {
                        write!(out, "│")?;
                    }

                    // draw big border or line number
                    if rows == 1 {
                        write!(out, " {}", 4 - y)?;
                    } else {
                        write!(out, " {}", v_border)?;
                    }
                }
            }
        }
    }

    for y in 0..4 {
		for x in 0..4 {
			let p = BPos::new(x,y);
			let highlighted = highlights.contains(&p);
			if highlighted {
	            write!(out, "{}", style::Italic)?;
			}
			draw_piece(&mut out, pos.translated(3 + 6*x, 2 + 4*y), board[(x,y)])?;
			if highlighted {
	            write!(out, "{}", style::Reset)?;
			}
		}
    }
    write!(out, "{}{} ", pos.to_goto_t(0, 4 * 4 + 1), v_border)?;
    draw_h_delim(&mut out, 4, cursor)?;
    if main {
        write!(out, " ║{}╚════A═════B═════C═════D════╝", pos.to_goto_t(0, 4 * 4 + 2))?;
    } else {
        write!(out, "{}     A     B     C     D", pos.to_goto_t(0, 4 * 4 + 2))?;
    }
    log::trace!("draw_board: end");
    Ok(())
}
