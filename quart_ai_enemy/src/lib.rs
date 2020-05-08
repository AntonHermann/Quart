mod rand_agent;

use quart_lib::{Game, BPos, Piece};

/// Some kind of AI agent the player can play against
pub trait AiAgent {
	/// Given the current game state, here the AI decides what to do.
	/// First it outputs a `BPos` which indicates where the piece shall be placed,
	/// Second it outputs a next piece for it's opponent to place
	fn play(&mut self, game: &Game) -> (BPos, Piece);
}

pub fn get_ai_agent() -> Box<dyn AiAgent> {
	Box::new(self::rand_agent::RandAgent::new())
}
