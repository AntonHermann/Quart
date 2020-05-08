use std::{error::Error, fmt};
use self::GameError::*;

// FIXME
#[derive(Debug)]
/// Errors that can occur when placing a new piece via `place_piece()`
pub enum GameError {
	/// The specified cell is already occupied with a piece
	CellOccupied,
	/// The specified piece is already used on the board
	PeaceInUse,
	/// An action required `self.selected_piece` to be `Some(_)`, but is was `None`
	NoPieceSelected,
	/// An action required `self.selected_piece` to be `None`, but it was `Some(_)`
	PieceAlreadySelected,
}
impl fmt::Display for GameError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			CellOccupied         => write!(f, "The specified cell is already occupied with a piece"),
			PeaceInUse           => write!(f, "The specified piece is already on the board"),
			NoPieceSelected      => write!(f, "An action required `self.selected_piece` to be `Some(_)`, but is was `None`"),
			PieceAlreadySelected => write!(f, "An action required `self.selected_piece` to be `None`, but it was `Some(_)`"),
		}
	}
}
impl Error for GameError {}
