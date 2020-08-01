use super::FragmentShader as FS;
use super::GeometryShader as GS;
use super::VertexShader as VS;

pub struct Program {
	id: u32,
}

impl Program {
	pub unsafe fn new(vs: &VS, gs: &GS, fs: &FS) -> Result<Program, String> {
		let id = gl::CreateProgram();
		gl::AttachShader(id, vs.id());
		gl::AttachShader(id, gs.id());
		gl::AttachShader(id, fs.id());
		gl::LinkProgram(id);
		// Checking program link status
		let mut status = 0;
		gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);

		if status == 0 {
			// Get the legth of the info log
			let mut len = 0;
			gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
			// Allocate the memory to store the log
			let log = super::new_cstring_with_len(len as usize);
			// Retrive the info log
			gl::GetProgramInfoLog(id, len, &mut len, log.as_ptr() as *mut _);
			return if let Ok(string) = log.into_string() {
				Err(format!("Program linking error:\n{}", string))
			} else {
				Err("Program linking error:\n<Can't convert the error log to a String>".into())
			};
		}
		Self::unbind();
		Ok(Program { id })
	}

	pub unsafe fn uniform_location(&self, name: &str) -> Result<i32, String> {
		let uniform = gl::GetUniformLocation(self.id, super::string_to_cstring(name).as_ptr());
		if uniform < 0 {
			Err(format!("'{}' is not a uniform", name))
		} else {
			Ok(uniform)
		}
	}

	pub unsafe fn vertex_attrib_location(&self, name: &str) -> Result<i32, String> {
		let attrib = gl::GetAttribLocation(self.id, super::string_to_cstring(name).as_ptr());
		if attrib < 0 {
			Err(format!("'{}' is not a vertex attribute", name))
		} else {
			Ok(attrib)
		}
	}

	pub unsafe fn bind(prg: &Self) {
		gl::UseProgram(prg.id);
	}

	pub unsafe fn unbind() {
		gl::UseProgram(0);
	}
}
