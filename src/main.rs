mod gui;
pub mod game;

use termion::{
	screen::AlternateScreen,
	raw::IntoRawMode,
	input::TermRead,
	event::Key,
	cursor,
	clear,
};
use std::io::{self, Write};

use self::gui::*;
pub use self::game::{
	board::*,
	Game,
	GameState::*,
};

fn main() -> io::Result<()> {
	// FIXME: when used outside of this dir, the log files are around everywhere!
	flexi_logger::Logger::with_env_or_str("warn")
		.log_to_file()
		.directory("logs")
		.create_symlink("log.log")
		.format(flexi_logger::default_format)
		.start().unwrap();

	// prepare input/output
	let stdout = io::stdout().into_raw_mode()?;
	let mut stdout = AlternateScreen::from(stdout);
	let stdin = io::stdin();
	write!(stdout, "{}{}{}", cursor::Hide, cursor::Goto(2,2), clear::All)?;

	// game state
	let mut game = Game::new();

	// initial drawing
	draw_gui(&mut stdout, &game, None)?;

	// game loop
	for c in stdin.keys() {
		match c? {
			Key::Esc | Key::Char('q') => break,
			Key::Up    => game.move_cursor( 0, -1),
			Key::Down  => game.move_cursor( 0,  1),
			Key::Left  => game.move_cursor(-1,  0),
			Key::Right => game.move_cursor( 1,  0),
			Key::Char(n) if "abcd".contains(n) => {
				game.set_cursor_x("abcd".find(n).unwrap() as u16);
			},
			Key::Char(n) if "1234".contains(n) => {
				game.set_cursor_y(3 - "1234".find(n).unwrap() as u16);
			},
			Key::Char('\n') => game.enter(),
			_ => {},
		}

		if game.check() {
			// GAME OVER
			break;
		}

		// redraw boards and piece preview
		draw_gui(&mut stdout, &game, None)?;
	}

	// restore state and exit
	write!(stdout, "{}{}", cursor::Show, cursor::Down(1))?;
	stdout.flush()?;
	std::mem::drop(stdout);

	if game.state == GameOver {
		println!("+++ GAME OVER +++");
		println!("Player {} lost", game.player_turn);
	}
	
	Ok(())
}
