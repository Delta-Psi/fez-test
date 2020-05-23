use gl::types::*;

pub struct VertexArrayObject(GLuint);

impl VertexArrayObject {
    pub fn new() -> VertexArrayObject {
        let handle = unsafe {
            let mut handle: GLuint = 0;
            gl::GenVertexArrays(1, &mut handle as *mut GLuint);
            handle
        };

        VertexArrayObject(handle)
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.0);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn gl_handle(&self) -> GLuint {
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
