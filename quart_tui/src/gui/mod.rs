use match_cfg::match_cfg;
match_cfg! {
	#[cfg(feature = "termion")] => {
		mod termion_ui;
		use termion_ui::TermionGui as GuiImpl;
	}
	_ => {
		compile_error!("No Gui implementation found. Use exactly one of the provided features!");
	}
}

use crate::BPos;
use crate::UiState;

use std::io::Result;

/// Game UI Events
pub enum Event {
	/// Exit Game
	Exit,
	/// Primary action, confirm, select, ..
	Enter,
	/// Move cursor up
	CursorUp,
	/// Move cursor down
	CursorDown,
	/// Move cursor left
	CursorLeft,
	/// Move cursor right
	CursorRight,
	/// Move cursor to specific x position
	CursorToX(u16),
	/// Move cursor to specific y position
	CursorToY(u16),
	/// Move cursor to specific position
	CursorToPos(BPos),
}

/// A generic user interface
pub trait Gui {
	/// Draw Gui
	fn draw(&mut self, ui_state: &UiState) -> Result<()>;
	/// Poll pending events
	fn poll_event(&mut self, ui_state: &UiState) -> Option<Event>;
}

/// Create a Gui instance
pub fn create_default() -> Result<impl Gui> {
	GuiImpl::new()
}
