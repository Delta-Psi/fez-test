use gl::types::*;

pub struct Texture(GLuint);

impl Texture {
    pub fn new() -> Texture {
        Texture(unsafe {
            let mut name = 0;
            gl::GenTextures(1, &mut name as *mut GLuint);
            name
        })
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
