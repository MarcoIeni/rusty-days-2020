use crate::cell::Cell;
use crate::graphics::{
	FragmentShader, GeometryShader, Program, VertexArrayObject, VertexBufferObject, VertexShader,
};
use memoffset::offset_of;
use mpsc::{Receiver, RecvError, SendError, Sender, TryRecvError};
use std::mem::size_of;
use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};

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
	shared: Arc<Mutex<Vec<Cell>>>,
	size: usize,
	sync: Sync,
}

impl Renderer {
	pub fn new(shared: Arc<Mutex<Vec<Cell>>>) -> Result<(Self, Sync), String> {
		let initial = shared.lock().unwrap();
		let size = initial.len();
		unsafe {
			let vs = VertexShader::new(&Path::new("shaders/cell.vert"))?;
			let vs = GeometryShader::new(&Path::new("shaders/cell.geom"))?;
			let fs = FragmentShader::new(&Path::new("shaders/cell.frag"))?;
			let program = Program::new(&vs, &fs)?;

			Program::bind(&program);

			let vao = VertexArrayObject::new();
			VertexArrayObject::bind(&vao);

			let vbo = VertexBufferObject::new(initial.len(), Some(&initial));
			VertexBufferObject::bind(&vbo);

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
		}

		let (sync, other) = Sync::channel();
		Ok((Self { shared, sync, size }, other))
	}

	pub fn update(&mut self) {
		self.sync.recv().ok(); // TODO
	}

	pub fn render(&mut self) {}
}
