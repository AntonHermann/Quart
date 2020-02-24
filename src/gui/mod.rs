mod termion;

pub use self::termion::*;
use crate::{Game, BPos};

use std::io::{self, Write};
use ::termion::{
    clear,
    cursor,
    screen,
    event::{Event as TEvent, Key, MouseEvent},
    input::{TermRead, MouseTerminal},
    raw::{IntoRawMode, RawTerminal},
};


pub enum Event {
	// TODO
	Exit,
	Enter,
	CursorUp,
	CursorDown,
	CursorLeft,
	CursorRight,
	CursorToX(u16),
	CursorToY(u16),
	CursorToPos(BPos),
}

pub trait Gui where Self: Sized {
	fn new() -> io::Result<Self>;
	fn draw(&mut self, game: &Game) -> io::Result<()>;
	fn poll_event(&mut self, game: &Game) -> Option<Event>;
	fn events(&mut self) -> ::termion::input::Events<io::Stdin>;
}

pub fn create_default() -> io::Result<impl Gui> {
	TermionGui::new()
}

struct TermionGui {
	out: MouseTerminal<RawTerminal<io::Stdout>>,
}

impl Gui for TermionGui{
    fn new() -> io::Result<Self> {
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
	fn draw(&mut self, game: &Game) -> io::Result<()> {
		termion::draw(&mut self.out, game, None)
	}
	fn poll_event(&mut self, game: &Game) -> Option<Event> {
		io::stdin()
			.events()
			.filter_map(Result::ok)
			.inspect(|e| log::debug!("Event: {:?}", e))
			.filter_map(|te| event_from_termion_event(te, game))
			.next()
	}
	fn events(&mut self) -> ::termion::input::Events<io::Stdin> {
		io::stdin().events()
	}
}
impl Drop for TermionGui {
	fn drop(&mut self) {
		let _ = writeln!(self.out, "{}", screen::ToMainScreen);
	}
}

fn event_from_termion_event(e: TEvent, game: &Game) -> Option<Event> {
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
				if let Some(bpos) = screen_to_bpos(&game, None, x, y) {
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
