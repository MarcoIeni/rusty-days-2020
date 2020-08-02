use crate::cell::Cell;
use crate::graphics::{
	FragmentShader, GeometryShader, Program, VertexArrayObject, VertexBufferObject, VertexShader,
};
use glutin::{PossiblyCurrent, WindowedContext};
use memoffset::offset_of;
use mpsc::{Receiver, RecvError, SendError, Sender, TryRecvError};
use std::mem::size_of;
use std::path::Path;
use std::sync::{mpsc, Arc, RwLock};

macro_rules! opengl_error {
	() => {
		let error = gl::GetError();
		if error != 0 {
			panic!(format!("[file={}, line={}] {}", file!(), line!(), error))
			}
	};
}

pub struct Sync(Receiver<()>, Sender<()>);

impl Sync {
	pub fn send(&self) -> Result<(), SendError<()>> {
		self.1.send(())
	}

	pub fn try_recv(&self) -> Result<(), TryRecvError> {
		self.0.try_recv()
	}

	pub fn recv(&self) -> Result<(), RecvError> {
		self.0.recv()
	}

	pub fn channel() -> (Self, Self) {
		let (atx, brx) = mpsc::channel();
		let (btx, arx) = mpsc::channel();
		(Self(arx, atx), Self(brx, btx))
	}
}

pub struct Renderer {
	shared: Arc<RwLock<Vec<Cell>>>,
	capacity: usize,
	length: usize,
	sync: Sync,
	window: WindowedContext<PossiblyCurrent>,
	_opengl: (Program, VertexArrayObject, VertexBufferObject),
}

impl Renderer {
	pub fn new(
		shared: Arc<RwLock<Vec<Cell>>>,
		size: usize,
		sync: Sync,
		window: WindowedContext<PossiblyCurrent>,
	) -> Result<Self, String> {
		if !window.is_current() {
			panic!("The provided window is not the current context!")
		}

		let (program, vao, vbo);
		unsafe {
			// Compile the shaders and the program
			let vs = VertexShader::new(&Path::new("src/shaders/cell.vert"))?;
			let gs = GeometryShader::new(&Path::new("src/shaders/cell.geom"))?;
			let fs = FragmentShader::new(&Path::new("src/shaders/cell.frag"))?;
			program = Program::new(&vs, &gs, &fs)?;
			Program::bind(&program);
			opengl_error!();

			// Create and bind a vertex array object
			vao = VertexArrayObject::new();
			VertexArrayObject::bind(&vao);
			opengl_error!();

			// Create and bind a vertex buffer object
			vbo = VertexBufferObject::new::<Cell>(size, None);
			VertexBufferObject::bind(&vbo);
			opengl_error!();

			// Vertex attributes:
			//
			//  state   position    direction
			// +------+-----+-----+-----+-----+
			// | id:0 |   id:1    |   id:2    |
			// +------+-----+-----+-----+-----+

			VertexArrayObject::u8_attrib_format(0, 1, size_of::<Cell>(), offset_of!(Cell, state));
			VertexArrayObject::f32_attrib_format(
				1,
				2,
				size_of::<Cell>(),
				offset_of!(Cell, position),
			);
			VertexArrayObject::f32_attrib_format(
				2,
				2,
				size_of::<Cell>(),
				offset_of!(Cell, direction),
			);
			opengl_error!();
		}
		Ok(Self {
			shared,
			sync,
			capacity: size,
			length: 0,
			window,
			_opengl: (program, vao, vbo),
		})
	}

	pub fn update(&mut self) {
		// Tell the Game thread to update the shared buffer data
		self.sync.send().expect("Game broke?!");
		// Wait for a response from the Game thread
		if let Ok(()) = self.sync.recv() {
			// Lock the shared memory (readonly)
			if let Ok(vec) = self.shared.read() {
				// If the size has changed, the buffer must be reallocated
				let size = vec.len();
				if self.capacity < size {
					self.capacity = size;
					self.length = size;
					unsafe { VertexBufferObject::resize(size, Some(&vec)) };
				} else {
					self.length = size;
					unsafe { VertexBufferObject::write(0, &vec) }
				}
				// Drop the lock at this point, as it's not gonna need it anymore
				std::mem::drop(vec);
				// Render the scene
				unsafe {
					gl::Clear(gl::COLOR_BUFFER_BIT);
					gl::DrawArrays(gl::POINTS, 0, self.length as i32);
					opengl_error!();
				}
				self.window.swap_buffers().ok(); // TODO
			} else {
				// TODO
			}
		} else {
			// TODO
		}
	}
}
