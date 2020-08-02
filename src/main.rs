mod cell;
mod config;
mod game;
mod graphics;
mod point;
mod renderer;

use game::Game;
use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use renderer::{Renderer, Sync};
use std::sync::{Arc, RwLock};

fn main() {
	// Create the event loop
	let el = EventLoop::new();
	// Create the window builder
	let wb = WindowBuilder::new()
		.with_title("Rusty Days") // Set the title of the window
		.with_inner_size(LogicalSize::new(500.0, 500.0)); // Set the size of the window

	// Create the window context from the winow builder and the event loop
	let wc = ContextBuilder::new().build_windowed(wb, &el).unwrap();
	// Set the window context as the current context
	let window = unsafe { wc.make_current().unwrap() };
	// Load the opengl functions
	gl::load_with(|symbol| window.context().get_proc_address(symbol) as *const _);

	let cells = config::get().cells.to_vec();
	let initial_size = cells.len();
	let shared = Arc::new(RwLock::new(cells));

	let (sync_game, sync_renderer) = Sync::channel();
	let mut game = Game::new(Arc::clone(&shared), sync_game);

	// To pass the window context to the other thread, first it must be made not current in this
	// one, that it will be made current in the new thread
	let window_not_current = unsafe { window.make_not_current().unwrap() };
	std::thread::spawn(move || {
		// Create the renderer
		let mut renderer = Renderer::new(shared, initial_size, sync_renderer, unsafe {
			window_not_current.make_current().unwrap()
		})
		.unwrap();
		loop {
			renderer.update();
		}
	});
	// Run the event loop
	el.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Poll;
		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
				_ => {}
			},
			_ => {}
		}
		game.update();
	});
}
