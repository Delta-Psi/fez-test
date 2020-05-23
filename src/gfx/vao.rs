use gl::types::*;

pub struct VertexArrayObject(GLuint);

impl VertexArrayObject {
    pub fn new() -> VertexArrayObject {
        VertexArrayObject(unsafe {
            let mut name = 0;
            gl::GenVertexArrays(1, &mut name as *mut GLuint);
            name
        })
    }

    pub fn name(&self) -> GLuint {
        self.0
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.0 as *const GLuint);
        }
    }
}
