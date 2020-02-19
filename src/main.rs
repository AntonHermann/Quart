#![warn(missing_docs)]
//! Simple board game as TUI

/// Contains fundamental game structs, game logic
pub mod game;
/// Contains the Terminal User Interface
mod gui;

use std::io::{self, Write};
use termion::{
    clear,
    cursor,
    screen,
    event::*,
    input::{TermRead, MouseTerminal},
    raw::IntoRawMode,
};

use self::game::{board::*, Game, GameState::*};

fn main() -> io::Result<()> {
	let res = run();
	log::info!("{:?}", res);
	res
}

fn run() -> io::Result<()> {
    // FIXME: when used outside of this dir, the log files are around everywhere!
    flexi_logger::Logger::with_env_or_str("info, quart::gui=debug")
        .log_to_file()
        .directory("logs")
        .create_symlink("log.log")
        .format(flexi_logger::default_format)
        .start()
        .unwrap();

    // prepare input/output
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode()?);
    let stdin = io::stdin();
    write!(stdout, "{}{}{}", screen::ToAlternateScreen, cursor::Goto(2, 2), clear::All)?;

    // Panicking shows weird output when in raw mode -> at least log panic msg
    let default_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
		log::error!("{}", info);
		default_panic_hook(info);
    }));

    // game state
    let mut game = Game::new();
    log::debug!("Created game");

    // initial drawing
    gui::draw(&mut stdout, &game, None)?;

    // game loop
    for c in stdin.events() {
	    log::debug!("Key pressed: {:?}", c);
        match c? {
	        Event::Key(k) => match k {
	            Key::Esc | Key::Char('q') => break,
	            Key::Up | Key::Char('k') => game.move_cursor(0, -1),
	            Key::Down | Key::Char('j') => game.move_cursor(0, 1),
	            Key::Left | Key::Char('h') => game.move_cursor(-1, 0),
	            Key::Right | Key::Char('l') => game.move_cursor(1, 0),
	            Key::Char(n) if "abcd".contains(n) => {
	                game.set_cursor_x("abcd".find(n).unwrap() as u16);
	            }
	            Key::Char(n) if "1234".contains(n) => {
	                game.set_cursor_y(3 - "1234".find(n).unwrap() as u16);
	            }
	            Key::Char('\n') => game.enter(),
	            _ => {},
	        },
	        Event::Mouse(m) => match m {
				MouseEvent::Press(_,x,y) => {
					if let Some(bpos) = gui::screen_to_bpos(&game, None, x, y) {
						game.cursor_pos = bpos;
					}
				},
				_ => continue,
	        },
	        _ => continue,
        }

        if game.check() {
	        log::info!("Game Over: {:?}", game.game_over_info);
            // GAME OVER
            // break;
        }

        // redraw boards and piece preview
        gui::draw(&mut stdout, &game, None)?;
    }
    log::trace!("After game loop");

    write!(stdout, "{}", screen::ToMainScreen)?;
    stdout.flush()?;
    std::mem::drop(stdout);

	log::info!("End, {:?}", game);
    if game.state == GameOver {
        println!("+++ GAME OVER +++");
        println!("Player {} lost", game.player_turn);
        println!("{:?}", game.game_over_info);
    }

    Ok(())
}
