// #![recursion_limit="256"]
#![recursion_limit="512"]

use quart_lib::{Game};
use std::sync::Mutex;
use actix_web::{
	web::{self, Data},
	App,
	HttpServer,
};
use listenfd::ListenFd;

mod handlers;
mod render;

pub const BASE_PATH: &str = "";

pub struct AppState {
	game: Mutex<Game>,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
	let game = Data::new(AppState {
		game: Mutex::new(Game::new())
	});

	// for live reloading
	let mut listenfd = ListenFd::from_env();
	let mut server = HttpServer::new(move || {
		App::new()
			.app_data(game.clone())
			.service(
				web::scope("/move_cursor")
				.service(
					web::resource("/by/{dx}/{dy}")
					.guard(handlers::board_pos_guard)
					.route(web::get().to(handlers::mov_cur_by))
				)
				.service(
					web::resource("/to/{x}/{y}")
					.guard(handlers::board_pos_guard)
					.route(web::get().to(handlers::mov_cur_to))
				)
			)
			.route("/enter", web::get().to(handlers::enter))
			// .route("/", web::get().to(handlers::greet))
			.route("/s/{filename:.*}", web::get().to(handlers::file))
	});

	// if systemfd is running, we reuse the already opened fd
	server = if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
		server.listen(listener)?
	} else {
		server.bind("127.0.0.1:8000")?
	};

	server.run().await
}
