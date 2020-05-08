#![warn(missing_docs)]
//! Simple board game, game logic

/// Board management, checking for game over condition
pub mod board;
/// Contains `GameError`, the core game error type
pub mod error;

pub use self::board::*;
pub use self::error::GameError;

/// The state the game is in
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    /// The player selects a piece which his opponent has to place on the board
    SelectPiece,
    /// The player has to place the piece his opponent gave him
    PlacePiece,
    /// One of the players lost, game over
    GameOver,
}

/// The central data structure of the game
#[derive(Debug)]
pub struct Game {
    /// The current state of the game
    pub state: GameState,
    /// Which player's turn it is
    pub player_turn: u32,
    /// The board on which will be played
    pub board: Board,
    /// The selected piece, if any
    pub selected_piece: Option<Piece>,
    /// In case of Game Over, this contains a description
    pub game_over_info: Option<GameOverInfo>,
}
impl Game {
    /// Create a new `Game`
    pub fn new() -> Self {
        Self {
            state: GameState::SelectPiece,
            player_turn: 1,
            board: Board::default(),
            selected_piece: None,
            game_over_info: None,
        }
    }

	/// Place selected piece on the board at `place_pos`
	///
	/// Errors:
	/// - NoPieceSelected: `self.selected_piece` is `None`
	/// - CellOccupied:	`place_pos` is already occupied with a piece
	pub fn place_piece(&mut self, place_pos: BPos) -> Result<(), GameError> {
		if self.selected_piece.is_none() {
			Err(GameError::NoPieceSelected)
		} else if self.board[place_pos].is_some() {
			Err(GameError::CellOccupied)
		} else {
			self.board[place_pos] = self.selected_piece;
			self.state = GameState::SelectPiece;
			// self.player_turn = 3 - self.player_turn; // FIXME
			Ok(())
		}
	}

	/// Select `next_piece` for the next player, it's his turn now
	///
	/// Errors:
	/// - PeaceInUse: `next_piece` is already on the board
	pub fn select_next_piece(&mut self, next_piece: Piece) -> Result<(), GameError> {
		if self.board.contains(next_piece) {
			Err(GameError::PeaceInUse)
		} else {
			self.selected_piece = Some(next_piece);
			self.state = GameState::PlacePiece;
			self.player_turn = 3 - self.player_turn; // FIXME
			Ok(())
		}
	}

    /// Whether the game is over
    pub fn is_over(&self) -> bool {
        self.state == GameState::GameOver
    }

    /// Check if the game is over (delegate from main_board)
    /// Returns true on GameOver
    pub fn check(&mut self) -> bool {
        if let Some(info) = self.board.check() {
            self.state = GameState::GameOver;
            self.game_over_info = Some(info);
            true
        } else {
            false
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
