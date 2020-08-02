use gl;

pub struct VertexArrayObject {
	id: u32,
}

impl VertexArrayObject {
	pub unsafe fn new() -> VertexArrayObject {
		let mut id = 0;
		gl::GenVertexArrays(1, &mut id);
		VertexArrayObject { id }
	}

	pub unsafe fn f32_attrib_format(location: u32, elements: u32, stride: usize, offset: usize) {
		gl::EnableVertexAttribArray(location);
		gl::VertexAttribPointer(
			location,
			elements as i32,
			gl::FLOAT,
			gl::FALSE,
			stride as i32,
			offset as *const _,
		);
	}

	pub unsafe fn i32_attrib_format(location: u32, elements: u32, stride: usize, offset: usize) {
		gl::EnableVertexAttribArray(location);
		gl::VertexAttribIPointer(
			location,
			elements as i32,
			gl::INT,
			stride as i32,
			offset as *const _,
		);
	}

	pub unsafe fn u32_attrib_format(location: u32, elements: u32, stride: usize, offset: usize) {
		gl::EnableVertexAttribArray(location);
		gl::VertexAttribIPointer(
			location,
			elements as i32,
			gl::UNSIGNED_INT,
			stride as i32,
			offset as *const _,
		);
	}

	pub unsafe fn u8_attrib_format(location: u32, elements: u32, stride: usize, offset: usize) {
		gl::EnableVertexAttribArray(location);
		gl::VertexAttribIPointer(
			location,
			elements as i32,
			gl::UNSIGNED_BYTE,
			stride as i32,
			offset as *const _,
		);
	}

	pub unsafe fn bind(vao: &Self) {
		gl::BindVertexArray(vao.id);
	}

	pub unsafe fn unbind() {
		gl::BindVertexArray(0);
	}
}

impl Drop for VertexArrayObject {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteVertexArrays(1, &mut self.id);
		}
	}
}
