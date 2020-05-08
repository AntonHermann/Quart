use std::{error::Error, fmt};
use self::GameError::*;

// FIXME
#[derive(Debug)]
/// Errors that can occur when placing a new piece via `place_piece()`
pub enum GameError {
	/// The specified cell is already occupied with a piece
	CellOccupied,
	/// The specified piece is already used on the board
	PieceInUse,
	/// An action required `self.selected_piece` to be `Some(_)`, but is was `None`
	NoPieceSelected,
	/// An action required `self.selected_piece` to be `None`, but it was `Some(_)`
	PieceAlreadySelected,
	/// An action couldn't be fulfilled because the game was over
	GameIsOver,
}
impl fmt::Display for GameError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			CellOccupied         => write!(f, "The specified cell is already occupied with a piece"),
			PieceInUse           => write!(f, "The specified piece is already on the board"),
			NoPieceSelected      => write!(f, "An action required `self.selected_piece` to be `Some(_)`, but is was `None`"),
			PieceAlreadySelected => write!(f, "An action required `self.selected_piece` to be `None`, but it was `Some(_)`"),
			GameIsOver           => write!(f, "Game Over"),
		}
	}
}
impl Error for GameError {}
