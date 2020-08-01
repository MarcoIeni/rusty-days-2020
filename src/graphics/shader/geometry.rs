use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ptr;

pub struct GeometryShader {
	id: u32,
	source: String,
}

impl GeometryShader {
	pub fn id(&self) -> u32 {
		self.id
	}

	pub unsafe fn new(path: &Path) -> Result<GeometryShader, String> {
		if let Ok(mut file) = File::open(path) {
			let mut source = String::new();
			// Open the file
			if let Ok(_) = file.read_to_string(&mut source) {
				// Create a new geometry shader
				let id = gl::CreateShader(gl::GEOMETRY_SHADER);
				// Attach the source code to it
				gl::ShaderSource(
					id,
					1,
					&super::string_to_cstring(source.as_str()).as_ptr(),
					ptr::null(),
				);
				gl::CompileShader(id); // Compile it

				// Checking shader compile status
				let mut status = 0;
				gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut status);
				if status == 0 {
					// Get the legth of the info log
					let mut len = 0;
					gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
					// Allocate the memory to store the log
					let log = super::new_cstring_with_len(len as usize);
					// Retrive the info log
					gl::GetShaderInfoLog(id, len, &mut len, log.as_ptr() as *mut _);
					if let Ok(string) = log.into_string() {
						Err(format!("Geometry shader compile error:\n{}", string))
					} else {
						Err("Geometry shader compile error:\n<Can't convert the error log to a String>".to_string())
					}
				} else {
					Ok(GeometryShader { id, source })
				}
			} else {
				return Err("Cannot read the vertex shader file!".to_string());
			}
		} else {
			return Err("Geometry shader not found!".to_string());
		}
	}
}

impl std::fmt::Debug for GeometryShader {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Geometry shader source code:\n{}", self.source)
	}
}

impl Drop for GeometryShader {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteShader(self.id);
		}
	}
}
