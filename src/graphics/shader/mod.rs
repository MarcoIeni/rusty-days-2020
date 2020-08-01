pub mod fragment;
pub mod program;
pub mod vertex;

pub use fragment::FragmentShader;
pub use program::Program;
pub use vertex::VertexShader;

use std::ffi::CString;

// Creates a CString with the specified length
pub fn new_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}

// Converts a String into a CString
pub fn string_to_cstring(string: &str) -> CString {
    unsafe { CString::from_vec_unchecked(string.as_bytes().to_vec()) }
}
