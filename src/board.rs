use std::ops::{Index, IndexMut};

/// One field on a game board
#[derive(PartialEq, Eq)]
pub struct Piece {
	pub big: bool,
	pub dark: bool,
	pub round: bool,
	pub flat: bool,
}

/// A game board (array of rows)
#[derive(Default)]
pub struct Board(pub [[Option<Piece>; 4]; 4]);
impl Index<BPos> for Board {
	type Output = Option<Piece>;
	fn index(&self, pos: BPos) -> &Self::Output {
		&self.0[pos.y as usize][pos.x as usize]
	}
}
impl IndexMut<BPos> for Board {
	fn index_mut(&mut self, pos: BPos) -> &mut Self::Output {
		&mut self.0[pos.y as usize][pos.x as usize]
	}
}
impl Board {
	/// create a board with all different stones on it
	pub fn full() -> Board {
		let mut board: Board = Default::default();
		for x in 0..4 {
			for y in 0..4 {
				let big	  = x <= 1;
				let dark  = x % 2 == 0;
				let round = y <= 1;
				let flat  = y % 2 == 0;

				board.0[x][y] = Some(Piece { big, dark, round, flat });
			}
		}
		board
	}
}

/// A position on the board
#[derive(Copy, Clone)]
pub struct BPos { pub x: u16, pub y: u16 }
impl BPos {
	pub fn new(x: u16, y: u16) -> Self {
		Self { x, y }
	}
	pub fn invalid() -> Self {
		// Self { x: std::u16::MAX, y: std::u16::MAX }
		Self { x: 99, y: 99 }
	}
	pub fn _is_invalid(&self) -> bool {
		// self.x == std::u16::MAX && self.y == std::u16::MAX
		self.x == 99 && self.y == 99
	}
}
