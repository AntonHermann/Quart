use quart_lib::{Game, Board, BPos, GameState, Piece};
use typed_html::{
	html, text,
	dom::DOMTree,
	elements,
	types::{SpacedSet, Class, Id},
};

pub fn render(game: &Game) -> String {
	// whether the main board is active
	let main_act = game.state != GameState::SelectPiece;
	let status_msg = if game.state == GameState::GameOver {
		html!(
		  <h2 class="msg_game_over">
			{ text!("Game Over, player {} won!", game.player_turn) }
		  </h2>
		)
	} else {
		html!(
		  <h2 class="msg_player_turn">
		  	{ text!("Player {}s turn", game.player_turn) }
		  </h2>
		)
	};
	let cursor = game.cursor_pos;
	let doc: DOMTree<String> = html!(
      <html>
    	<head>
          <title>"Quart"</title>
          // <meta name=Metadata::Author content="Not Sanrio Co., Ltd"/>
          <link rel="stylesheet" href="/s/style.css" />
        </head>
        <body>
          <h1>"Quart"</h1>
          <div id="content">
			{ render_board(&game.main_board, Id::new("main_board"), cursor, main_act) }
			<div id="center_view">
			  <a href="/enter" id="button_submit"><span>"Submit"</span></a>
			  { render_selected_piece(game.selected_piece) }
			</div>
			{ render_board(&game.pieces_board, Id::new("pieces_board"), cursor, !main_act) }
		  </div>
		  <div id="status_msg">
			{ status_msg }
		  </div>
        </body>
      </html>
	);
	doc.to_string()
}

fn render_board(board: &Board, id: Id, cursor: BPos, sel: bool) -> Box<elements::table<String>> {
	let mut class = SpacedSet::new();
	if sel {
		class.add("selected");
	}
	html!(
        <table id=id class=class>
        	<tr class="digits dig_u">
				<th></th> <th>"A"</th> <th>"B"</th> <th>"C"</th> <th>"D"</th>
        	</tr>
        	{ (0..4).map(|x| html!(
        	<tr>
        		<th class="numbers num_l"> { text!("{}",x+1) } </th>
            	{ (0..4).map(|y| render_cell(board, x, y, cursor, sel)) }
        		<th class="numbers num_r"> { text!("{}",x+1) } </th>
            </tr>
        	)) }
        	<tr class="digits dig_d">
				<th></th> <th>"A"</th> <th>"B"</th> <th>"C"</th> <th>"D"</th>
        	</tr>
		</table>
	)
}

fn render_cell(board: &Board, x: u16, y: u16, cursor: BPos, sel: bool) -> Box<elements::td<String>> {
	let pos = BPos::new(x,y);
	let piece = board[pos];
	let id = Id::new(format!("cell_{}_{}", x, y));
	let href = if sel { format!("/move_cursor/to/{}/{}", x, y) } else { "".into() };
	let mut class_list = gen_classlist_for_piece(piece);
	if pos == cursor {
		class_list.add("piece_cursor");
	}
	let content = gen_piece_string(piece);

	html!(
		<td class=class_list>
			<a href=href id=id>
				{ text!("{}", content) }
			</a>
		</td>
	)
}

fn render_selected_piece(piece: Option<Piece>) -> Box<elements::div<String>> {
	let class_list = gen_classlist_for_piece(piece);
	let content = gen_piece_string(piece);
	html!(
	  <div id="selected_piece" class=class_list>
		<span>
		  { text!("{}", content) }
		</span>
	  </div>
	)
}

fn gen_classlist_for_piece(piece: Option<Piece>) -> SpacedSet<Class> {
	let mut classes = SpacedSet::new();
	if let Some(piece) = piece {
		if piece.big   { classes.add("piece_big"); }
		if piece.dark  { classes.add("piece_dark"); }
		if piece.flat  { classes.add("piece_flat"); }
		if piece.round { classes.add("piece_round"); }
	}
	classes
}
/// Creates the visual representation of a field, split into 3 lines
fn gen_piece_string(field: Option<Piece>) -> String {
	if let Some(Piece { big, dark:_, round, flat }) = field {
		let (rl, rr) = if round { ("(", ")") } else { ("|", "|") };
		let f1 = if flat { "---" } else { "\\ /" };
		let f2 = if flat { "   " } else { " O " };

		// (\ /) (---) |---| |\ /|
		// ( O ) (   ) |   | | O | (\ /) (---) |---| |\ /|
		// (   ) (   ) |   | |   | ( O ) (   ) |   | | O |
		//  BRf   BRF   BrF   Brf   bRf   bRF   brF   brf

		let roof  = format!("{}{}{}", rl, f1, rr);
		let base1 = format!("{}{}{}", rl, f2, rr);
		let base2 = format!("{}   {}", rl, rr);
		let empty = format!("     ");

		if big {
			roof + "\n" + &base1 + "\n" + &base2
		} else {
			empty + "\n" + &roof + "\n" + &base1
		}
	} else {
		"\n\n".to_owned()
	}
}

