/// Board management, checking for game over condition
pub mod board;

pub use self::board::*;

/// The state the game is in
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameState {
	/// The player selects a piece which his opponent has to place on the board
	SelectPiece,
	/// The player has to place the piece his opponent gave him
	PlacePiece,
	/// One of the players lost, game over
	GameOver,
}

/// The central data structure of the game
pub struct Game {
	/// The current state of the game
	pub state: GameState,
	/// Which player's turn it is
	pub player_turn: u32,
	/// The main board on which will be played
	pub main_board: Board,
	/// The board where the rest of the pieces is
	pub pieces_board: Board,
	/// On which position the cursor is
	pub cursor_pos: BPos,
	/// If there was a piece selected, it will be stored here
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
			main_board: Board::default(),
			pieces_board: Board::full(),
			cursor_pos: BPos::new(0,0),
			selected_piece: None,
			game_over_info: None,
		}
	}
	/// Whether the game is over
	pub fn is_over(&self) -> bool {
		self.state == GameState::GameOver
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
	/// Perform some action, depending on the game state.
	/// Normally picking up or putting down a piece
	pub fn enter(&mut self) {
		match self.state {
			GameState::SelectPiece => {
				self.selected_piece = self.pieces_board[self.cursor_pos].take();
				if self.selected_piece.is_some() {
					self.state = GameState::PlacePiece;
					self.player_turn = 3 - self.player_turn;
				}
			},
			GameState::PlacePiece => {
				if self.main_board[self.cursor_pos].is_none() {
					self.main_board[self.cursor_pos] = self.selected_piece.take();
					self.state = GameState::SelectPiece;
				}
			},
			GameState::GameOver => {}
		}
	}
	/// Check if the game is over (delegate from main_board)
	/// Returns true on GameOver
	pub fn check(&mut self) -> bool {
		if let Some(info) = self.main_board.check() {
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
