use gl::types::*;
use std::io::Read;

pub struct Texture(GLuint);

impl Texture {
    pub fn new() -> Texture {
        Texture(unsafe {
            let mut name = 0;
            gl::GenTextures(1, &mut name as *mut GLuint);
            name
        })
    }

    // THIS WILL UNWRAP RESULTS!!!
    // will also leave the texture bound
    pub fn load_from_png<R: Read>(input: R) -> Texture {
        use png::{ColorType, BitDepth};

        let decoder = png::Decoder::new(input);
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();
        
        assert_eq!(info.color_type, ColorType::RGBA, "unimplemented png color type");
        assert_eq!(info.bit_depth, BitDepth::Eight, "unimplemented png bit depth");

        let texture = Self::new();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture.name());
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as GLint,
                info.width as GLint, info.height as GLint, 0,
                gl::RGBA, gl::UNSIGNED_BYTE, buf.as_ptr() as *const _);
        }

        texture
    }

    pub fn name(&self) -> GLuint {
        self.0
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.0 as *const GLuint);
        }
    }
}
