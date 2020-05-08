use quart_lib::{Game, BPos, Piece};

/// An AI enemy
pub struct AiEnemy {}

impl AiEnemy {
	/// Create a new AI enemy
	pub fn new() -> Self {
		Self {}
	}

	/// Given the current game state, here the AI decides what to do.
	/// First it outputs a `BPos` which indicates where the piece shall be placed,
	/// Second it outputs a next piece for it's opponent to place
	pub fn play(&mut self, _game: &Game) -> (BPos, Piece) {
		todo!();
	}
}

impl Default for AiEnemy {
	fn default() -> Self { Self::new() }
}
