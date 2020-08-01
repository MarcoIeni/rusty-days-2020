use gl::{self, types as gl_t};
use std::ptr;

pub struct Texture {
    id: u32,
    width: u32,
    height: u32,
    px_size: (f32, f32),
}

impl Texture {
    pub unsafe fn new(&self, width: u32, height: u32, data: &[u8]) -> Self {
        let mut id: gl_t::GLuint = 0;
        // Genereate a new texture
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA as u32,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const _,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        Self::unbind();
        Self {
            id,
            width,
            height,
            px_size: (1.0 / width as f32, 1.0 / height as f32),
        }
    }

    pub unsafe fn resize(&mut self, width: u32, height: u32) {
        if self.width != width || self.height != height {
            Self::bind(self);

            self.width = width;
            self.height = height;
            self.px_size = (1.0 / width as f32, 1.0 / height as f32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            );
            Self::unbind();
        }
    }

    pub unsafe fn bind(tex: &Self) {
        gl::BindTexture(gl::TEXTURE_2D, tex.id);
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixel_size(&self) -> (f32, f32) {
        self.px_size
    }

    pub unsafe fn set_active_unit(unit: u32) {
        gl::ActiveTexture(gl::TEXTURE0 + unit);
    }

    pub unsafe fn unbind() {
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }
}
