#![warn(missing_docs)]
//! Simple board game as TUI

/// Contains fundamental game structs, game logic
pub mod game;
/// Contains the Terminal User Interface
mod gui;

use std::io::{self, Write};
use termion::{
    clear, cursor, event::Key, input::TermRead, raw::IntoRawMode, screen,
};

use self::game::{board::*, Game, GameState::*};
use self::gui::*;

fn main() -> io::Result<()> {
	let res = run();
	log::info!("{:?}", res);
	res
}

fn run() -> io::Result<()> {
    // FIXME: when used outside of this dir, the log files are around everywhere!
    flexi_logger::Logger::with_env_or_str("trace, quart::game::board=debug")
        .log_to_file()
        .directory("logs")
        .create_symlink("log.log")
        .format(flexi_logger::default_format)
        .start()
        .unwrap();

    // prepare input/output
    let mut stdout = io::stdout().into_raw_mode()?;
    let stdin = io::stdin();
    write!(stdout, "{}{}{}", screen::ToAlternateScreen, cursor::Goto(2, 2), clear::All)?;

    // game state
    let mut game = Game::new();
    log::debug!("Created game");

    // initial drawing
    draw_gui(&mut stdout, &game, None)?;

    // game loop
    for c in stdin.keys() {
	    log::debug!("Key pressed: {:?}", c);
        match c? {
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
            _ => {}
        }

        if game.check() {
	        log::info!("Game Over: {:?}", game.game_over_info);
            // GAME OVER
            // break;
        }

        // redraw boards and piece preview
        draw_gui(&mut stdout, &game, None)?;
    }
    log::trace!("After game loop");

    write!(stdout, "{}", screen::ToMainScreen)?;
    stdout.flush()?;

	log::info!("End, {:?}", game);
    if game.state == GameOver {
        println!("+++ GAME OVER +++");
        println!("Player {} lost", game.player_turn);
        println!("{:?}", game.game_over_info);
    }

    Ok(())
}
