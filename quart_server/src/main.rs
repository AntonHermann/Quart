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

pub const SERVER_ADDRESS: &str = "127.0.0.1:8000";

pub struct AppState {
	game: Mutex<Game>,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    flexi_logger::Logger::with_env_or_str("info, quart::gui=debug")
        .log_to_file()
        .directory     (concat!(env!("CARGO_MANIFEST_DIR"), "/logs"))
        .create_symlink(concat!(env!("CARGO_MANIFEST_DIR"), "/log.log"))
        .format(flexi_logger::default_format)
        .start()
        .unwrap();

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
			.route("/", web::get().to(handlers::show))
			.route("/s/{filename:.*}", web::get().to(handlers::file))
	});

	log::debug!("Created game and server");

	// if systemfd is running, we reuse the already opened fd
	server = if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
		log::info!("Attached to existing server instance");
		if let Ok(local_addr) = listener.local_addr() {
			eprintln!("Attached to server instance bound to {}", local_addr);
		} else {
			eprintln!("Attached to server instance with unknown address");
		}
		server.listen(listener)?
	} else {
		log::info!("Binding to address {}", SERVER_ADDRESS);
		eprintln!("Binding to address http://{}/", SERVER_ADDRESS);
		server.bind(SERVER_ADDRESS)?
	};



	server.run().await
}
