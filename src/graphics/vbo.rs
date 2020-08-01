use gl;
use std::mem::size_of;
use std::ptr;

pub struct VertexBufferObject {
    id: u32,
}

impl VertexBufferObject {
    pub unsafe fn new<T>(size: usize, data: Option<&[T]>) -> VertexBufferObject {
        let mut id = 0;
        gl::GenBuffers(1, &mut id);
        gl::BindBuffer(gl::ARRAY_BUFFER, id);
        gl::BufferStorage(
            gl::ARRAY_BUFFER,
            (size * size_of::<T>()) as isize,
            if let Some(v) = data {
                v.as_ptr() as *const _
            } else {
                ptr::null()
            },
            gl::MAP_WRITE_BIT | gl::DYNAMIC_STORAGE_BIT,
        );
        Self::unbind();
        VertexBufferObject { id }
    }

    pub unsafe fn bind(vbo: &Self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo.id);
    }

    pub unsafe fn write<T>(offset: usize, data: &[T]) {
        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            (offset * size_of::<T>()) as isize,
            (data.len() * size_of::<T>()) as isize,
            data.as_ptr() as *const _,
        );
    }

    pub unsafe fn unbind() {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
}

impl Drop for VertexBufferObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}
