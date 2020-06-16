use quart_lib::{Game, BPos, Piece, Board};
use rand::prelude::*;
use crate::AiAgent;

/// An AI enemy
pub struct RandAgent;

impl RandAgent {
	/// Create a new AI enemy
	pub fn new(_game: &Game) -> Self {
		Self
	}
}
impl AiAgent for RandAgent {
	fn play(&mut self, game: &Game) -> (BPos, Piece) {
		valid_random_move(&game.board)
	}
}

/// Generates a random move thats valid on the given board
pub(crate) fn valid_random_move(board: &Board) -> (BPos, Piece) {
	assert!(board.piece_count() < 16);
	let mut pos = random_bpos();
	while board[pos].is_some() {
		pos = random_bpos();
	}

	let mut piece = random_piece();
	while board.contains(piece) {
		piece = random_piece();
	}

	(pos, piece)
}
fn random_bpos() -> BPos {
	let mut rng = thread_rng();
	let x = rng.gen_range(0, 4);
	let y = rng.gen_range(0, 4);
	BPos::new(x, y)
}
fn random_piece() -> Piece {
	let (big, dark, round, flat) = random();
	Piece { big, dark, round, flat }
}
