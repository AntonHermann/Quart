//! TUI based on termion

/// drawing utilities
mod util;
// mod old; // some old, way too complex functions

pub use self::util::*;
use super::{Gui, Event};
use quart_lib::{GameState::*, BPos};
use crate::UiState;
use std::io::{self, Write};
use termion::{
    clear,
    cursor,
    screen,
    event::{Event as TEvent, Key, MouseEvent},
    input::{TermRead, MouseTerminal},
    raw::{IntoRawMode, RawTerminal},
};

/// TUI based on termion
pub struct TermionGui {
	out: MouseTerminal<RawTerminal<io::Stdout>>,
}
impl TermionGui {
	/// Create new instance, switch to alternate screen
	/// and setup panic hook that logs panics
	/// (the alternate screen makes logging to stderr not viable)
    pub fn new() -> io::Result<Self> {
		// prepare input/output
	    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode()?);
	    write!(stdout, "{}{}{}", screen::ToAlternateScreen, cursor::Goto(2, 2), clear::All)?;

	    // Panicking shows weird output when in raw mode -> at least log panic msg
	    let default_panic_hook = std::panic::take_hook();
	    std::panic::set_hook(Box::new(move |info| {
			log::error!("{}", info);
			default_panic_hook(info);
	    }));
		Ok(Self {
			out: stdout
		})
	}
}
impl Gui for TermionGui{
	fn draw(&mut self, ui_state: &UiState) -> io::Result<()> {
		draw(&mut self.out, ui_state, None)
	}
	fn poll_event(&mut self, ui_state: &UiState) -> Option<Event> {
		io::stdin()
			.events()
			.filter_map(Result::ok)
			.inspect(|e| log::debug!("Event: {:?}", e))
			.filter_map(|te| event_from_termion_event(te, ui_state))
			.next()
	}
}
impl Drop for TermionGui {
	fn drop(&mut self) {
		let _ = write!(self.out, "{}", screen::ToMainScreen);
		let _ = self.out.flush();
	}
}

fn event_from_termion_event(e: TEvent, ui_state: &UiState) -> Option<Event> {
	Some(match e {
		TEvent::Key(k) => match k {
            Key::Esc | Key::Char('q') => Event::Exit,
            Key::Up | Key::Char('k') => Event::CursorUp,
            Key::Down | Key::Char('j') => Event::CursorDown,
            Key::Left | Key::Char('h') => Event::CursorLeft,
            Key::Right | Key::Char('l') => Event::CursorRight,
            Key::Char(n) if "abcd".contains(n) => {
                Event::CursorToX("abcd".find(n).unwrap() as u16)
            }
            Key::Char(n) if "1234".contains(n) => {
                Event::CursorToY(3 - "1234".find(n).unwrap() as u16)
            }
            Key::Char('\n') => Event::Enter,
            _ => return None,
        },
        TEvent::Mouse(m) => match m {
			MouseEvent::Press(_,x,y) => {
				if let Some(bpos) = screen_to_bpos(&ui_state, None, x, y) {
					Event::CursorToPos(bpos)
				} else {
			        return None
				}
			},
			_ => return None,
        },
        TEvent::Unsupported(_) => return None,
	})
}

/// The text above the pieces board
const PIECES_BOARD_LABEL: &str = "Available Pieces";

/// A layout determining what GUI-Element goes where
struct Layout {
    main_board: SPos,
    curr_piece: SPos,
    pieces_board: SPos,
    status_label: SPos,
}
/// A default, wide screen layout
const LAYOUT_WIDE: Layout = Layout {
    main_board: SPos { x: 2, y: 2 },
    curr_piece: SPos { x: 35, y: 3 },
    pieces_board: SPos { x: 45, y: 2 },
    status_label: SPos { x: 4, y: 22 },
};
impl Layout {
    /// A default, wide screen layout
    pub fn _wide() -> Self {
        LAYOUT_WIDE
    }
}
/// This function brings the whole game to live. Here the game scene is drawn
fn draw<W: Write>(mut out: W, ui_state: &UiState, layout: Option<&Layout>) -> io::Result<()> {
	log::trace!("draw_gui");
    let layout = layout.unwrap_or(&LAYOUT_WIDE);

    write!(out, "{}", clear::All)?;

    let main_focus = ui_state.game.state == PlacePiece || ui_state.game.state == GameOver;
    let highlights = if let Some(goi) = ui_state.game.game_over_info.as_ref() {
		goi.positions.clone()
    } else {
	    Vec::new()
    };

    draw_board(
        &mut out,
        layout.main_board,
        &ui_state.game.board,
        ui_state.cursor_pos,
        main_focus,
        true,
        highlights,
    )?;

    draw_board(
        &mut out,
        layout.pieces_board,
        &ui_state.pieces_board,
        ui_state.cursor_pos,
        !main_focus,
        false,
        Vec::new(),
    )?;

    let label_pos = layout.pieces_board.translated_i32(2, -1);
    draw_label(&mut out, label_pos, 25, PIECES_BOARD_LABEL)?;

    let status_str = if ui_state.game.is_over() {
	    if let Some(goi) = ui_state.game.game_over_info.as_ref() {
	        format!("Game over because of {}", goi.property)
	    } else {
			log::error!("Should be Some(..)");
			String::new()
	    }
    } else {
        format!("Player {}'s turn!", ui_state.game.player_turn)
    };
    draw_label(&mut out, layout.status_label, std::cmp::max(25,status_str.len() as u16), &status_str)?;

    if ui_state.game.state == PlacePiece {
        draw_selected_piece(&mut out, layout.curr_piece, ui_state.game.selected_piece)?;
    }

	log::trace!("before goto 0,1");
    write!(out, "{}", cursor::Goto(0, 1))?;
	log::trace!("after goto 0,1, before flush");
    out.flush()?;

	log::trace!("draw_gui: end");
    Ok(())
}

/// Returns the BoardPos sfKLJSFLKJSFLKDSJF:LKDSJF:LSD
fn screen_to_bpos(ui_state: &UiState, layout: Option<&Layout>, x: u16, y: u16) -> Option<BPos> {
    let layout = layout.unwrap_or(&LAYOUT_WIDE);
	let offset = match ui_state.game.state {
		PlacePiece => layout.main_board,
		SelectPiece => layout.pieces_board,
		GameOver => return None,
	};
	// the space between offset and top-left of the real field
	let offset = offset.translated(2,1);
	if x < offset.x || y < offset.y { return None }

	let mut b_x = (x - offset.x) / 6;
	let mut b_y = (y - offset.y) / 4;

	// right/bottom edges should still match
	if x-offset.x == 24 { b_x = 3 }
	if y-offset.y == 16 { b_y = 3 }

	// field is 4x4 => position should be (0..3)x(0..3)
	if b_x < 4 && b_y < 4 {
		Some(BPos::new(b_x, b_y))
	} else {
		None
	}
}
