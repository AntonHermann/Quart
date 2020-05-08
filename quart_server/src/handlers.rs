use quart_lib::{BPos};
use std::path::PathBuf;
use actix_web::{
	web::{self, Data},
	Result,
	HttpRequest,
	HttpResponse,
	// Responder,
	dev::RequestHead,
};
use actix_files::NamedFile;
use super::AppState;
use crate::render::render;

// HANDLER FUNCTIONS
pub async fn file(req: HttpRequest) -> Result<NamedFile> {
	let path: PathBuf = req.match_info().query("filename").parse()?;
	let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	let resources_dir = manifest_dir.join("www");
	if !resources_dir.exists() {
		std::fs::create_dir(&resources_dir)?;
	}
	let file_path = resources_dir.join(path);
	Ok(NamedFile::open(file_path)?)
}

pub async fn mov_cur_by(data: Data<AppState>, delta: web::Path<(i8,i8)>) -> HttpResponse {
	let mut game = data.game.lock().unwrap(); // get game's MutexGuard

	game.move_cursor(delta.0.into(), delta.1.into());

	let s = render(&game);
	HttpResponse::Ok().content_type("text/html").body(s)
}
pub async fn mov_cur_to(data: Data<AppState>, pos: web::Path<(u8,u8)>) -> HttpResponse {
	let mut game = data.game.lock().unwrap(); // get game's MutexGuard
	game.set_cursor_pos(BPos::new(pos.0.into(), pos.1.into()));

	let s = render(&game);
	HttpResponse::Ok().content_type("text/html").body(s)
}
pub async fn enter(data: Data<AppState>) -> HttpResponse {
	let mut game = data.game.lock().unwrap(); // get game's MutexGuard
	game.enter();
	game.check();

	let s = render(&game);
	HttpResponse::Ok().content_type("text/html").body(s)
}

// GUARD FUNCTION
pub fn board_pos_guard(req: &RequestHead) -> bool {
	let path: &str = req.uri.path();
	let valid_entry_count = path
		.split('/')
		.filter_map(|s| s.parse::<i16>().ok())
		.filter(|val : &i16| val.abs() < 4)
		.count();
	valid_entry_count == 2
}
