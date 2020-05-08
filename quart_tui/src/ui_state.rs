use quart_lib::{Game, Board, BPos, GameState, Piece};

/// Current User Interface State (cursor position, highlighted fields, ...)
pub struct UiState {
	/// The game that's played
	pub game: Game,

    /// On which position the cursor is
    pub cursor_pos: BPos,

    /// The board where the rest of the pieces is
    pub pieces_board: Board,

    /// If there was a piece selected, it will be stored here
    pub selected_piece: Option<Piece>,
}

impl UiState {
	/// Create a new User Interface State struct
	pub fn new(game: Game) -> Self {
		Self {
			game,
			cursor_pos: BPos::new(0, 0),
			pieces_board: Board::full(),
			selected_piece: None,
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
                self.selected_piece = self.pieces_board[self.cursor_pos].take();
                if self.selected_piece.is_some() {
                    self.game.state = GameState::PlacePiece;
                    self.game.player_turn = 3 - self.game.player_turn;
                }
            }
            GameState::PlacePiece => {
                if self.game.board[self.cursor_pos].is_none() {
                    self.game.board[self.cursor_pos] = self.selected_piece.take();
                    self.game.state = GameState::SelectPiece;
                }
            }
            GameState::GameOver => {}
        }
    }
}
