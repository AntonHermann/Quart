use quart_lib::{
	Game,
	BPos,
	Piece,
	Board,
};
use crate::AiAgent;
use itertools::{
	// Itertools,
	iproduct
};

/// An AI enemy that plans ahead
pub struct DecisionTreeAgent {
	// base: Board,
}

impl DecisionTreeAgent {
	/// Create a `DecisionTreeAgent`
	pub fn new(_game: &Game) -> Self {
		Self {
			// base: game.board.clone(),
		}
	}
}
impl AiAgent for DecisionTreeAgent {
	fn play(&mut self, game: &Game) -> (BPos, Piece) {
		// moves the agent could do
		let agent_moves: Vec<Move> = get_all_agent_moves(game);
		log::trace!("Agent could put the piece at {} places", agent_moves.len());

		// FIXME: We mixed up pieces here: sometimes, they are the pieces that are placed in a move (Move = place `piece` at `pos`)
		// whereas sometimes we mean just the place (as we can't choose the piece we are given) and the piece refers to the
		// piece we give the player.
		//
		if let Some(winning_move) = find_winning_move(&game.board, &agent_moves) {
			// Place selected piece at `winning_move.pos`,
			// the piece to select afterwards shouldn't be relevant anymore.
			return (winning_move.pos, Piece::default());
		}

		// iterator of all moves that are safe to do without
		// giving the player the oportunity to win
		let mut not_losing_moves = agent_moves.iter().filter_map(|agent_move: &Move| {
			// how the board would look after playing this move
			let new_board: Board = apply_move(game.board.clone(), agent_move);

			// what moves the player could do then
			let new_board_moves: Vec<Move> = get_all_player_moves(&new_board);

			match find_winning_move(&new_board, &new_board_moves) {
				// if there is no winning move for the enemy after we do a move,
				// this move is safe to do
				None => Some(agent_move),
				Some(_) => None,
			}
		});
		// just do the first move that won't give the player the opportunity to win
		if let Some(not_losing_move) = not_losing_moves.next() {
			return (not_losing_move.pos, not_losing_move.piece);
		}

		// no directly winning move, no move that can't lead to a loss
		// => do random move and hope the player doesn't notice he can win
		crate::rand_agent::valid_random_move(&game.board)
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Move {
	pos: BPos,
	piece: Piece,
}
impl Move {
	fn new((x,y):(u16,u16), piece: Piece) -> Self {
		Self {
			pos: BPos::new(x,y),
			piece
		}
	}
}

/// An iterator containing all 16 pieces in the game
fn get_all_pieces() -> impl Iterator<Item=Piece> {
	iproduct!(0..4, 0..4).map(|(a,b)| {
        let big = a <= 1;
        let dark = a % 2 == 0;
        let round = b <= 1;
        let flat = b % 2 == 0;
        Piece {
            big,
            dark,
            round,
            flat,
        }
    })
}

/// All moves the player could do on the given board
fn get_all_player_moves(board: &Board) -> Vec<Move> {
	iproduct!(0..4,0..4)
		.filter(|(x,y)| board[(*x,*y)].is_none())
		.flat_map(|pos| {
			get_all_pieces()
				.filter(|p| !board.contains(*p))
				.map(move |piece| Move::new(pos, piece))
		})
		.collect()
}


/// All moves the agent could do with the given piece
fn get_all_agent_moves(game: &Game) -> Vec<Move> {
	assert!(game.selected_piece.is_some(), "No piece selected");
	iproduct!(0..4,0..4)
		.filter(|(x,y)| game.board[(*x,*y)].is_none())
		.map(|pos| Move::new(pos, game.selected_piece.unwrap()))
		.collect()
}

fn apply_move(mut board: Board, mov: &Move) -> Board {
	assert!(board[mov.pos].is_none());

	board[mov.pos] = Some(mov.piece);

	board
}
fn is_winning_move(board: Board, mov: &Move) -> bool {
	apply_move(board, mov).check().is_some()
}
/// Find a winning move, if any
fn find_winning_move<'m>(board: &Board, moves: &'m [Move]) -> Option<&'m Move> {
	moves.iter().find(|mov: &&Move| is_winning_move(board.clone(), mov))
}
/// Partition moved into (winning moves, not winning moves)
fn _partition_moves<'m>(board: &Board, moves: &'m [Move]) -> (Vec<Move>, Vec<Move>) {
	moves.iter().partition(|mov: &&Move| is_winning_move(board.clone(), mov))
}

#[test]
fn test_get_all_pieces() {
	let board = Board::full();
	assert!(get_all_pieces().all(|p| board.contains(p)));
	assert_eq!(get_all_pieces().count(), 16);
}
#[test]
fn test_apply_move() {
	let empty_board = Board([
		[None, None, None, None],
		[None, None, None, None],
		[None, None, None, None],
		[None, None, None, None],
	]);
	let piece = Piece { big: true, dark: true, round: false, flat: false };
	let board1 = Board([
		[None, None, None, None],
		[None, None, None, Some(piece)],
		[None, None, None, None],
		[None, None, None, None],
	]);
	let mov1 = Move::new((3,1), piece);
	assert_eq!(apply_move(empty_board, &mov1), board1);
}
#[test]
#[should_panic]
fn test_apply_invalid_move() {
	let piece = Piece { big: true, dark: true, round: false, flat: false };
	let board1 = Board([
		[None, None, None, None],
		[None, None, None, Some(piece)],
		[None, None, None, None],
		[None, None, None, None],
	]);
	let mov1 = Move::new((3,1), piece);
	apply_move(board1, &mov1);
}
#[test]
fn test_is_winning_move() {
	let p = |big,dark,round,flat| Some(Piece { big, dark, round, flat });
	let (t,f) = (true, false);
	let board1 = Board([
		[None, None, None, p(t,t,t,t)],
		[None, None, None, p(t,t,t,f)],
		[None, None, None, p(t,t,f,t)],
		[None, None, None, None],
	]);
	let move1 = Move::new((3,3), Piece { big: true, dark: false, round: false, flat: false });
	assert!(is_winning_move(board1, &move1));
}
