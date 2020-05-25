use gl::types::*;

#[derive(Debug)]
pub struct BufferObject(GLuint);

impl BufferObject {
    pub fn new() -> BufferObject {
        BufferObject(unsafe {
            let mut name = 0;
            gl::GenBuffers(1, &mut name as *mut GLuint);
            name
        })
    }

    pub fn name(&self) -> GLuint {
        self.0
    }
}

impl Drop for BufferObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.0 as *const GLuint);
        }
    }
}
