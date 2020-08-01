mod cell;
mod config;
mod game;
mod graphics;
mod point;
mod renderer;

use game::Game;
use renderer::Renderer;
use std::sync::{Arc, RwLock};

fn main() {
	let cells = config::get().cells.to_vec();
	let shared = Arc::new(RwLock::new(cells));

	let (mut renderer, sender) = Renderer::new(Arc::clone(&shared)).unwrap();
	let mut game = Game::new(shared, sender);

	std::thread::spawn(move || loop {
		renderer.update();
	});
	game.run();
}
