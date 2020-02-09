use crate::{BPos, Board, Piece};
use std::io::{self, Write};
use termion::{color, cursor::Goto, style};

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
        let reset = color::Fg(color::Reset).to_string();

        // (\ /) (---) |---| |\ /|
        // ( O ) (   ) |   | | O | (\ /) (---) |---| |\ /|
        // (   ) (   ) |   | |   | ( O ) (   ) |   | | O |
        //  BRf   BRF   BrF   Brf   bRf   bRF   brF   brf

        let roof  = format!("{}{}{}{}{}", d, rl, f1, rr, reset);
        let base1 = format!("{}{}{}{}{}", d, rl, f2, rr, reset);
        let base2 = format!("{}{}   {}{}", d, rl, rr, reset);
        let empty = format!("{}     {}", d, reset);

        if big {
            [roof, base1, base2]
        } else {
            [empty, roof, base1]
        }
    } else {
        ["     ".to_owned(), "     ".to_owned(), "     ".to_owned()]
    }
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

	// draw border
    if main {
        write!(out, "{}╔═══════════════════════════╗", pos.to_goto())?;
        write!(out, "{}╚═══════════════════════════╝", pos.to_goto_t(0, 18))?;
    }
    // vertical border, gets drawn together with grid
    let v_border = if main { "║" } else { " " };

	// draw grid
    for row in 0..=4*4 {
        write!(out, "{}", pos.to_goto_t(0, row + 1))?;
        let s = if row == 0 {
			"┌─────┬─────┬─────┬─────┐"
        } else if row == 4*4 {
			"└─────┴─────┴─────┴─────┘"
        } else if row % 4 == 0 {
			"├─────┼─────┼─────┼─────┤"
        } else {
	        "│     │     │     │     │"
        };
        write!(out, "{b} {} {b}", s, b=v_border)?;
    }

    // draw line numbers
    let letters = ['A','B','C','D'];
    for i in 0..4 {
        write!(out, "{}{}", pos.to_goto_t( 0, 4 * i + 3), 4 - i)?; // left
        write!(out, "{}{}", pos.to_goto_t(28, 4 * i + 3), 4 - i)?; // right
        write!(out, "{}{}", pos.to_goto_t(6 * i + 5,  0), letters[i as usize])?; // top
        write!(out, "{}{}", pos.to_goto_t(6 * i + 5, 18), letters[i as usize])?; // bottom
	}

	// draw cursor
	if sel {
		let cursor_pos = pos.translated(2+6*cursor.x, 4*cursor.y+1);
		write!(out, "{}{}╔═════╗", style::Invert, cursor_pos.to_goto())?;
		for i in 1..4 {
			write!(out, "{}║     ║", cursor_pos.to_goto_t(0, i))?;
		}
		write!(out, "{}╚═════╝{}", cursor_pos.to_goto_t(0,4), style::Reset)?;
	}

	// draw pieces
    for y in 0..4 {
		for x in 0..4 {
			let p = BPos::new(x,y);
			let highlighted = highlights.contains(&p);
			if highlighted {
	            write!(out, "{}", style::Italic)?;
			}
			draw_piece(&mut out, pos.translated(3+6*x, 2+4*y), board[p])?;
			if highlighted {
	            write!(out, "{}", style::Reset)?;
			}
		}
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
