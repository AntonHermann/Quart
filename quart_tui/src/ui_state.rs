use quart_lib::{Game, Board, BPos, GameState, GameError};

/// Current User Interface State (cursor position, highlighted fields, ...)
pub struct UiState {
	/// The game that's played
	pub game: Game,

    /// On which position the cursor is
    pub cursor_pos: BPos,

    /// The board where the rest of the pieces is
    pub pieces_board: Board,
}

impl UiState {
	/// Create a new User Interface State struct
	pub fn new(game: Game) -> Self {
		Self {
			game,
			cursor_pos: BPos::new(0, 0),
			pieces_board: Board::full(),
		}
	}
	
    /// Move the cursor position by some given deltas
    pub fn move_cursor(&mut self, dx: i32, dy: i32) {
        self.cursor_pos.x = (self.cursor_pos.x as i32 + 4 + dx).abs() as u16 % 4;
        self.cursor_pos.y = (self.cursor_pos.y as i32 + 4 + dy).abs() as u16 % 4;
    }
    /// Set the cursor onto a specific x position
    pub fn set_cursor_x(&mut self, x: u16) {
        self.cursor_pos.x = x;
    }
    /// Set the cursor onto a specific y position
    pub fn set_cursor_y(&mut self, y: u16) {
        self.cursor_pos.y = y;
    }
    /// Set the cursor to a specific position
    pub fn set_cursor_pos(&mut self, pos: BPos) {
		self.cursor_pos = pos;
    }

    /// Perform some action, depending on the game state.
    /// Normally picking up or putting down a piece
    pub fn enter(&mut self) {
        match self.game.state {
            GameState::SelectPiece => {
	            if let Some(piece) = self.pieces_board[self.cursor_pos] {
					match self.game.select_next_piece(piece) {
						Ok(()) => self.pieces_board[self.cursor_pos] = None,
						Err(GameError::PeaceInUse) => {
							log::error!("Something went terribly wrong, we tried to place a piece that was already on the board");
							panic!("Duplicate pieces on same board");
						},
						Err(_) => {},
		            }
	            }
            },
            GameState::PlacePiece => {
	            if let Err(GameError::NoPieceSelected) = self.game.place_piece(self.cursor_pos) {
					log::error!("Something went terribly wrong, we tried to place a piece while none was selected");
					panic!("Place without selected piece");
	            }
            },
            GameState::GameOver => {},
        }
    }
}
