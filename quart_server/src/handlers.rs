use quart_lib::BPos;
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
	log::info!("Serving file {}", file_path.display());
	Ok(NamedFile::open(file_path)?)
}

pub async fn mov_cur_by(app_state: Data<AppState>, delta: web::Path<(i8,i8)>) -> HttpResponse {
	log::info!("Requested: Move Cursor By {:?}", delta);
	let mut ui_state = app_state.ui_state.lock().unwrap();

	ui_state.move_cursor(delta.0.into(), delta.1.into());

	let s = render(&ui_state);
	HttpResponse::Ok().content_type("text/html").body(s)
}

pub async fn mov_cur_to(app_state: Data<AppState>, pos: web::Path<(u8,u8)>) -> HttpResponse {
	log::info!("Requested: Move Cursor To {:?}", pos);
	let mut ui_state = app_state.ui_state.lock().unwrap();
	
	ui_state.set_cursor_pos(BPos::new(pos.0.into(), pos.1.into()));

	let s = render(&ui_state);
	HttpResponse::Ok().content_type("text/html").body(s)
}

pub async fn enter(app_state: Data<AppState>) -> HttpResponse {
	log::info!("Requested: Enter");
	let mut ui_state = app_state.ui_state.lock().unwrap();
	ui_state.enter();
	ui_state.game.check();

	let s = render(&ui_state);
	HttpResponse::Ok().content_type("text/html").body(s)
}

pub async fn show(app_state: Data<AppState>) -> HttpResponse {
	log::info!("Requested: Show");
	let ui_state = app_state.ui_state.lock().unwrap();
	let s = render(&ui_state);
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
