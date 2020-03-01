use quart_lib::{BPos};
use actix_web::{
	web::{self, Data},
	HttpRequest,
	Responder,
	dev::RequestHead
};
use super::AppState;

// handler functions
pub async fn greet(req: HttpRequest) -> impl Responder {
	let name = req.match_info().get("name").unwrap_or("World");
	format!("Hello {}!", &name)
}

pub fn board_pos_guard(req: &RequestHead) -> bool {
	let path: &str = req.uri.path();
	let valid_entry_count = path
		.split('/')
		.filter_map(|s| s.parse::<i16>().ok())
		.filter(|val : &i16| val.abs() < 4)
		.count();
	valid_entry_count == 2
}
pub async fn mov_cur_by(data: Data<AppState>, delta: web::Path<(i8,i8)>) -> String {
	let mut game = data.game.lock().unwrap(); // get game's MutexGuard
	game.move_cursor(delta.0.into(), delta.1.into());
	format!("{:?}", game.cursor_pos)
}
pub async fn mov_cur_to(data: Data<AppState>, pos: web::Path<(u8,u8)>) -> String {
	let mut game = data.game.lock().unwrap(); // get game's MutexGuard
	game.set_cursor_pos(BPos::new(pos.0.into(), pos.1.into()));
	format!("{:?}", game.cursor_pos)
}
pub async fn enter(data: Data<AppState>) -> String {
	let mut game = data.game.lock().unwrap(); // get game's MutexGuard
	game.enter();
	if game.check() {
		String::from("Game over!")
	} else {
		String::from("All fine")
	}
}
