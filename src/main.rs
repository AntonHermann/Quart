mod draw;
mod board;

use termion::{
	screen::AlternateScreen,
	raw::IntoRawMode,
	input::TermRead,
	event::Key,
	cursor,
	clear,
};
use std::io::{self, Write};

use self::draw::*;
use self::GameState::*;
use self::board::*;

/// The state the game is in
#[derive(PartialEq, Eq)]
enum GameState {
	SelectPiece,
	PlacePiece,
}

struct Game {
	state: GameState,
	main_board: Board,
	pieces_board: Board,
	cursor_pos: BPos,
	selected_piece: Option<Piece>,
}
impl Game {
	fn new() -> Self {
		Self {
			state: GameState::SelectPiece,
			main_board: Board::default(),
			pieces_board: Board::full(),
			cursor_pos: BPos::new(0,0),
			selected_piece: None,
		}
	}
}

fn main() -> io::Result<()> {
	// prepare input/output
	let stdout = io::stdout().into_raw_mode()?;
	let mut stdout = AlternateScreen::from(stdout);
	let stdin = io::stdin();
	write!(stdout, "{}{}{}", cursor::Hide, cursor::Goto(2,2), clear::All)?;

	// game state
	let mut game = Game::new();

	// layout
	let layout = Layout::wide();

	// initial drawing
	draw(&mut stdout, &game, &layout)?;

	// game loop
	for c in stdin.keys() {
		match c? {
			Key::Esc | Key::Char('q') => break,
			Key::Up    => game.cursor_pos.y = (game.cursor_pos.y + 4 - 1) % 4,
			Key::Down  => game.cursor_pos.y = (game.cursor_pos.y + 4 + 1) % 4,
			Key::Left  => game.cursor_pos.x = (game.cursor_pos.x + 4 - 1) % 4,
			Key::Right => game.cursor_pos.x = (game.cursor_pos.x + 4 + 1) % 4,
			Key::Char(n) if "abcd".contains(n) => {
				game.cursor_pos.x = "abcd".find(n).unwrap() as u16;
			},
			Key::Char(n) if "1234".contains(n) => {
				game.cursor_pos.y = 3 - "1234".find(n).unwrap() as u16;
			},
			Key::Char('\n') if game.state == SelectPiece  => { // Enter, SelectPiece mode
				game.selected_piece = game.pieces_board[game.cursor_pos].take();
				if game.selected_piece.is_some() {
					game.state = PlacePiece;
				}
			},
			Key::Char('\n') if game.state == PlacePiece => { // Enter, PlacePiece
				if game.main_board[game.cursor_pos].is_none() {
					game.main_board[game.cursor_pos] = game.selected_piece.take();
					game.state = SelectPiece;
				}
			},
			_ => {},
		}

		// redraw boards and piece preview
		draw(&mut stdout, &game, &Layout::wide())?;
	}

	// restore state and exit
	write!(stdout, "{}{}", cursor::Show, cursor::Down(1))?;
	stdout.flush()?;
	Ok(())
}

/// The text above the pieces board
const PIECES_BOARD_LABEL: &'static str = "Available Pieces";
// const PIECES_BOARD_LABEL: &'static str = "Verfügbare Steine";

struct Layout {
	main_board: SPos,
	curr_piece: SPos,
	pieces_board: SPos,
}
impl Layout {
	fn wide() -> Self {
		Self {
			main_board: SPos::new(2,2),
			curr_piece: SPos::new(35,3),
			pieces_board: SPos::new(45,2),
		}
	}
}
fn draw<W: Write>(mut out: W, game: &Game, layout: &Layout) -> io::Result<()> {
	write!(out, "{}", clear::All)?;

	let main_curr = if game.state == PlacePiece { game.cursor_pos } else { BPos::invalid() };
	draw_board(&mut out, layout.main_board, &game.main_board, main_curr, true)?;

	let pieces_curr = if game.state == SelectPiece { game.cursor_pos } else { BPos::invalid() };
	draw_board(&mut out, layout.pieces_board, &game.pieces_board, pieces_curr, false)?;

	let label_pos = layout.pieces_board.translated_i32(2, -1);
	draw_label(&mut out, label_pos, 25, PIECES_BOARD_LABEL)?;

	if game.state == PlacePiece {
		draw_selected_piece(&mut out, layout.curr_piece, &game.selected_piece)?;
	}
	Ok(())
}
