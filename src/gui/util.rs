use crate::{BPos, Board, Piece};
use std::io::{self, Write};
use termion::{color, cursor::Goto, style};

/// The horizontal line used for drawing the board
const LINE_H: &str = "─────";
/// The horizontal line used for drawing the board - selected
const LINE_H_S: &str = "═════";

/// A position on the screen
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct SPos {
    pub x: u16,
    pub y: u16,
}
impl SPos {
    pub fn _new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
    /// Create a termion [`Goto`] struct to simplify placing the terminal cursor
    /// at the specified position
    pub fn to_goto(self) -> Goto {
        Goto(self.x, self.y)
    }
    /// Create a termion [`Goto`], translated by `dx` and `dy`
    pub fn to_goto_t(self, dx: u16, dy: u16) -> Goto {
		Goto(self.x + dx, self.y + dy)
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
    if let Some(Piece {
        big,
        dark,
        round,
        flat,
    }) = field
    {
        let d = if dark {
            color::Fg(color::Red).to_string()
        } else {
            "".into()
        };
        let (rl, rr) = if round { ("(", ")") } else { ("|", "|") };
        let f1 = if flat { "---" } else { "\\ /" };
        let f2 = if flat { "   " } else { " O " };
        let reset = color::Fg(color::Reset).to_string() + style::Reset.as_ref();

        // (\ /) (---) |---| |\ /|
        // ( O ) (   ) |   | | O | (\ /) (---) |---| |\ /|
        // (   ) (   ) |   | |   | ( O ) (   ) |   | | O |
        //  BRf   BRF   BrF   Brf   bRf   bRF   brF   brf

        let roof = format!("{}{}{}{}{}", d, rl, f1, rr, reset);
        let base1 = format!("{}{}{}{}{}", d, rl, f2, rr, reset);
        let base2 = format!("{}{}   {}{}", d, rl, rr, reset);
        let empty = "     ".into();

        if big {
            [roof, base1, base2]
        } else {
            [empty, roof, base1]
        }
    } else {
        ["     ".to_owned(), "     ".to_owned(), "     ".to_owned()]
    }
}

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
    let cursor = if sel { cursor } else { BPos::invalid() };

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
			draw_piece(&mut out, pos.translated(3 + 6*x, 2 + 4*y), board[(x,y)])?;
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

/// Draw a small box to show which piece is currently selected
pub fn draw_selected_piece<W: Write>(
    mut out: W,
    pos: SPos,
    piece: Option<Piece>,
) -> io::Result<()> {
	log::trace!("draw_selected_piece");
    write!(out, "{}┌PLACE┐", pos.to_goto())?;
    // let piece_strs = field_strs(piece);
    for row in 0..3 {
        write!(out, "{}│     │", pos.to_goto_t(0, row + 1))?;
    }
    draw_piece(&mut out, pos.translated(1,1), piece)?;
    write!(out, "{}└─────┘", pos.to_goto_t(0, 4))?;
    log::trace!("draw_selected_piece: end");
    Ok(())
}

/// Draw a piece at the given location
pub fn draw_piece<W: Write>(
    mut out: W,
    pos: SPos,
    piece: Option<Piece>,
) -> io::Result<()> {
	log::trace!("draw_piece");

    write!(out, "{}", pos.to_goto())?;
    let piece_strs = field_strs(piece);
    for row in 0..3 {
        write!(out, "{}{}", pos.to_goto_t(0, row), piece_strs[row as usize])?;
    }
    Ok(())
}

/// Draw a label, with origin at x,y, such that is is centered inside the given width
/// label shouldn't be wider thatn total_width
pub fn draw_label<W: Write>(
    mut out: W,
    pos: SPos,
    total_width: u16,
    label: &str,
) -> io::Result<()> {
	log::trace!("draw_label: {:?}, {:?}, {:?}, {}", pos, total_width, label, label.len());
	assert!(total_width >= label.len() as u16);
    let offset = (total_width / 2) - (label.len() as u16 / 2);
    write!(out, "{}{}", pos.to_goto_t(offset, 0), label)?;
	log::trace!("draw_label: end");
	Ok(())
}
