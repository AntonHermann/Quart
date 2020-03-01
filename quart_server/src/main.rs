use quart_lib::{Game};
use std::sync::Mutex;
use actix_web::{
	web::{self, Data},
	App,
	// HttpRequest,
	HttpServer,
	// Responder
};

mod handlers;
mod print_board;

pub struct AppState {
	game: Mutex<Game>,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
	let game = Data::new(AppState {
		game: Mutex::new(Game::new())
	});

	HttpServer::new(move || {
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
				// .route("/by/{x}/{name}", web::get().to(handlers::greet))
			)
			.route("/enter", web::get().to(handlers::enter))
			.route("/", web::get().to(handlers::greet))
			.route("/{name}", web::get().to(handlers::greet))
			.wrap(print_board::PrintBoard)
	})
	.bind("127.0.0.1:8000")?
	.run()
	.await
}
