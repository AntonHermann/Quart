use quart_lib::{Game, Board, BPos, GameState, Piece};
use typed_html::{
	html, text,
	dom::DOMTree,
	elements,
	types::{SpacedSet, Class},
};

pub fn render(game: &Game) -> String {
	let board = match game.state {
		GameState::SelectPiece => &game.pieces_board,
		_ => &game.main_board,
	};
	let doc: DOMTree<String> = html!(
	    <html>
	        <head>
	            <title>"Quart"</title>
	            // <meta name=Metadata::Author content="Not Sanrio Co., Ltd"/>
	            <style> { text!("{}", CSS) } </style>
	        </head>
	        <body>
	            <h1>"Quart"</h1>
	            <table>
	            	<tr>
						<th></th> <th>"A"</th>  <th>"B"</th>  <th>"C"</th>  <th>"D"</th>
	            	</tr>
	            	{ (0..4).map(|x| html!(
		            	<tr>
		            		<th> { text!("{}",x+1) } </th>
			            	{ (0..4).map(|y| render_cell(board, x, y)) }
			            </tr>
	            	)) }
				</table>
	        </body>
	    </html>
	);
	doc.to_string()
}

fn render_cell(board: &Board, x: u16, y: u16) -> Box<elements::td<String>> {
	let pos = BPos::new(x,y);
	let td = if let Some(piece) = board[pos] {
		let class_list = gen_classlist_for_piece(piece);
		html!(<td class=class_list> { text!("{:?}", piece) } </td>)
	} else {
		html!(<td></td>)
	};
	// td.to_string()
	td
}

fn gen_classlist_for_piece(piece: Piece) -> SpacedSet<Class> {
	let mut classes = SpacedSet::new();
	if piece.big   { classes.add("piece_big"); }
	if piece.dark  { classes.add("piece_dark"); }
	if piece.flat  { classes.add("piece_flat"); }
	if piece.round { classes.add("piece_round"); }
	classes
}

const CSS: &str = r##"
	td.piece_big   {
		font-size: 1.4em;
	}
	td.piece_dark  {
		font-weight: bold;
	}
	td.piece_flat  {
		// font-size: 1.4em;
	}
	td.piece_round {
		border: 2px solid; 
		border-radius: 50%;
	}
"##;
