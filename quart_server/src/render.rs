use quart_lib::{Board, BPos, GameState, Piece};
use crate::ui_state::UiState;
use itertools::join;

pub fn render(ui_state: &UiState) -> String {
	// whether the main board is active
	let main_act = ui_state.game.state != GameState::SelectPiece;
	let status_msg = if ui_state.game.state == GameState::GameOver {
		format!("<h2 class='msg_game_over'>Game Over, player {} won!</h2>", ui_state.game.player_turn)
	} else {
		format!("<h2 class='msg_player_turn'>Player {}s turn</h2>", ui_state.game.player_turn)
	};
	let cursor = ui_state.cursor_pos;
	let board1 = render_board(&ui_state.game.board, "main_board", cursor, main_act);
	let selected_piece = render_selected_piece(ui_state.selected_piece);
	let board2 = render_board(&ui_state.pieces_board, "pieces_board", cursor, !main_act);
	format!(r#"
      <html>
    	<head>
          <title>Quart</title>
          <link rel="stylesheet" href="/s/style.css" />
        </head>
        <body>
          <h1>Quart</h1>
          <div id="content">
          	{board1}
			<div id="center_view">
			  <a href="/enter" id="button_submit"><span>"Submit"</span></a>
			  {selected_piece}
			</div>
			{board2}
		  </div>
		  <div id="status_msg">
			{status_msg}
		  </div>
        </body>
      </html>"#, status_msg=status_msg, board1=board1, selected_piece=selected_piece, board2=board2)
}

fn render_board(board: &Board, id: &'static str, cursor: BPos, sel: bool) -> String {
	let class = if sel { "selected" } else { "" };

	let mut rows = String::new();
	for x in 0..4 {
		let mut cells = String::new();
		for y in 0..4 {
        	cells.push_str(&render_cell(board, x, y, cursor, sel));
		}
		let row = format!("<tr><th class='numbers num_l'>{num}</th>{cells}<th class='numbers num_r'>{num}</th></tr>", num=x+1, cells=cells);
		rows.push_str(&row);
	}
	format!(r#"
        <table id={id} class={class}>
        	<tr class="digits dig_u">
				<th></th> <th>A</th> <th>B</th> <th>C</th> <th>D</th>
        	</tr>
			{rows}
        	<tr class="digits dig_d">
				<th></th> <th>A</th> <th>B</th> <th>C</th> <th>D</th>
        	</tr>
		</table>"#,
		id=id, class=class, rows=rows)
}

fn render_cell(board: &Board, x: u16, y: u16, cursor: BPos, sel: bool) -> String {
	let pos = BPos::new(x,y);
	let piece = board[pos];
	let id = format!("cell_{}_{}", x, y);
	let href = if sel { format!("/move_cursor/to/{}/{}", x, y) } else { "".into() };
	let mut class_list = gen_classlist_for_piece(piece);
	if pos == cursor {
		class_list.push("piece_cursor".into());
	}
	let content = gen_piece_string(piece);

	format!("
		<td class='{class_list}'>
			<a href='{href}' id='{id}'>{content}</a>
		</td>",
		class_list=join(class_list, ", "), href=href, id=id, content=content)
}

fn render_selected_piece(piece: Option<Piece>) -> String {
	let class_list = gen_classlist_for_piece(piece);
	let content = gen_piece_string(piece);
	format!(r#"
	  <div id="selected_piece" class="{class_list}">
		<span>
		  {content}
		</span>
	  </div>"#,
	  class_list=join(class_list, ", "), content=content)
}

fn gen_classlist_for_piece(piece: Option<Piece>) -> Vec<&'static str> {
	let mut classes = Vec::new();
	if let Some(piece) = piece {
		if piece.big   { classes.push("piece_big"); }
		if piece.dark  { classes.push("piece_dark"); }
		if piece.flat  { classes.push("piece_flat"); }
		if piece.round { classes.push("piece_round"); }
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

