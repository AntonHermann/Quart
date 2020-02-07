use std::{
	ops::{Index, IndexMut},
	fmt,
};

/// A position on the board
#[derive(PartialEq, Eq, Clone, Copy)]
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
impl fmt::Debug for BPos {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "BPos{{x:{},y:{}}}", self.x, self.y)
		} else {
			write!(f, "BP({},{})", self.x, self.y)
		}
	}
}

/// One field on a game board
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Piece {
	pub big: bool,
	pub dark: bool,
	pub round: bool,
	pub flat: bool,
}
impl fmt::Debug for Piece {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let Piece { big, dark, round, flat } = self;
		if f.alternate() {
			write!(f, "Piece{{big:{},dark:{},round:{},flat:{}}}",big,dark,round,flat)
		} else {
			let b_str = if *big { "B" } else { "b" };
			let d_str = if *big { "D" } else { "d" };
			let r_str = if *big { "R" } else { "r" };
			let f_str = if *big { "F" } else { "f" };
			write!(f, "P{{{}{}{}{}}}",b_str,d_str,r_str,f_str)
		}
	}
}

/// A game board (array of rows)
#[derive(Debug, Default)]
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

	/// Check for Game Over condition
	/// (at least 1 property has to be equal on all 4 fields of a row, column or diagonal)
	pub fn check(&self) -> bool {
		// check all rows ...
		let mut rows: Vec<Vec<BPos>> = Vec::new();
		for y in 0..4 {
			rows.push((0..4).map(|x| BPos::new(x,y)).collect::<Vec<_>>());
		}
		log::trace!("rows: {:?}", rows);

		// ... all columns ...
		let mut cols: Vec<Vec<BPos>> = Vec::new();
		for x in 0..4 {
			cols.push((0..4).map(|y| BPos::new(x,y)).collect::<Vec<_>>());
		}
		log::trace!("cols: {:?}", cols);

		// ... and the diagonals
		let diags: Vec<Vec<BPos>> = vec![
			(0..4).map(|j| BPos::new(j,  j)).collect::<Vec<_>>(),
			(0..4).map(|j| BPos::new(j,3-j)).collect::<Vec<_>>(),
		];
		log::trace!("diag: {:?}", diags);

		// combine the 3 iterators
		let all = rows.into_iter().chain(cols.into_iter()).chain(diags.into_iter());

		// So far, we have an iterator of vectors of positions, now we turn
		// it into an iterator of vectors of fields (Option<Piece>)
		all.map(|v: Vec<BPos>| v.into_iter().map(|pos| self[pos].clone()).collect())
		   .inspect(|v: &Vec<Option<Piece>>| { log::trace!("{:?}", v) })
		   .any(|v: Vec<Option<Piece>>| check_fields(&v))
	}
}

/// Check the first 4 fields whether they satisfy the conditions
/// (at least 1 property has to be equal on all 4)
fn check_fields(fields: &[Option<Piece>]) -> bool {
	// Insted of a Vector of Options, Option allows us to collect into a
	// Option of Vec instead, being None if one of the elements was None (short-circuit)
	let first4: Option<Vec<Piece>> = fields.iter().take(4).cloned().collect();
	if let Some(pieces) = first4 {
		let mut equal = (true,true,true,true);
		let first = &pieces[0];
		for p in &pieces[1..] {
			equal.0 &= first.big   == p.big;
			equal.1 &= first.dark  == p.dark;
			equal.2 &= first.round == p.round;
			equal.3 &= first.flat  == p.flat;
		}
		equal.0 || equal.1 || equal.2 || equal.3
	} else {
		// at least one of the cells was empty
		false
	}
}

#[allow(non_snake_case)]
#[test]
fn test_check_fields() {
	let p_bdrf = _new_piece(0,0,0,0);
	let p_BdRf = _new_piece(1,0,1,0);
	let p_bDrf = _new_piece(0,1,0,0);
	let p_BDrf = _new_piece(1,1,0,0);
	let p_BDrF = _new_piece(1,1,0,1);

	assert!(check_fields (&[p_bdrf,p_bdrf,p_bdrf,p_bdrf]), "all equal");
	assert!(check_fields (&[p_bdrf,p_BdRf,p_bDrf,p_BDrf]), "all not flat");
	assert!(!check_fields(&[p_bdrf,p_bdrf,p_bdrf,None]  ), "1 none");
	assert!(!check_fields(&[p_bdrf,p_BdRf,p_bDrf,p_BDrF]), "no equals");
}

fn _new_piece(big:u8,dark:u8,round:u8,flat:u8) -> Option<Piece> {
	Some(Piece {
		big:   big 	 != 0,
		dark:  dark  != 0,
		round: round != 0,
		flat:  flat  != 0,
	})
}
