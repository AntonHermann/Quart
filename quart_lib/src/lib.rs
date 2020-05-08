#![warn(missing_docs)]
//! Simple board game, game logic

/// Board management, checking for game over condition
pub mod board;

pub use self::board::*;

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
            game_over_info: None,
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
