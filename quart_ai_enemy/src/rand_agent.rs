use quart_lib::{Game, BPos, Piece};
use rand::prelude::*;
use crate::AiAgent;

/// An AI enemy
pub struct RandAgent;

impl RandAgent {
	/// Create a new AI enemy
	pub fn new() -> Self {
		Self
	}
}
impl AiAgent for RandAgent {
	fn play(&mut self, game: &Game) -> (BPos, Piece) {
		let mut pos = random_bpos();
		while game.board[pos].is_some() {
			pos = random_bpos();
		}

		let mut piece = random_piece();
		while game.board.contains(piece) {
			piece = random_piece();
		}

		(pos, piece)
	}
}
impl Default for RandAgent {
	fn default() -> Self { Self::new() }
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
