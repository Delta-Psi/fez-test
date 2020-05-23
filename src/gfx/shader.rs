use gl::types::*;

#[derive(Clone, Copy, Debug)]
pub enum ShaderType {
    Vertex,
    Geometry,
    Fragment,
}

pub struct Shader(GLuint);

impl Shader {
    pub fn compile(type_: ShaderType, source: &str) -> Result<Shader, String> {
        // creates the shader object
        let name = unsafe {
            gl::CreateShader(match type_ {
                ShaderType::Vertex => gl::VERTEX_SHADER,
                ShaderType::Geometry => gl::GEOMETRY_SHADER,
                ShaderType::Fragment => gl::FRAGMENT_SHADER
            })
        };
        assert!(name != 0, "could not create shader object");

        // attaches the source
        let source_ptr = source.as_ptr() as *const GLchar;
        let source_length = source.len() as GLint;
        unsafe {
            gl::ShaderSource(name, 1, &source_ptr as *const *const GLchar, &source_length as *const GLint);
        }

        // compiles the shader
        let status = unsafe {
            gl::CompileShader(name);

            // check if it's been compiled correctly
            let mut status: GLint = 0;
            gl::GetShaderiv(name, gl::COMPILE_STATUS, &mut status as *mut GLint);
            status
        };

        if status == gl::FALSE as GLint {
            // read error log
            let len = unsafe {
                let mut len: GLint = 0;
                gl::GetShaderiv(name, gl::INFO_LOG_LENGTH, &mut len as *mut GLint);
                len as usize
            };

            let mut buf = vec![0u8; len];
            unsafe {
                gl::GetShaderInfoLog(name, len as GLsizei, 0 as *mut _, buf.as_mut_ptr() as *mut GLchar);
            }

            Err(String::from_utf8_lossy(&buf).to_string())
        } else {
            Ok(Shader(name))
        }
    }

    pub fn name(&self) -> GLuint {
        self.0
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.0);
        }
    }
}

pub struct ShaderProgram(GLuint);

impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        let name = unsafe {
            gl::CreateProgram()
        };
        assert!(name != 0, "could not create shader program object");

        ShaderProgram(name)
    }

    pub fn attach(&self, shader: &Shader) {
        unsafe {
            gl::AttachShader(self.0, shader.name());
        }
    }

    pub fn link(&self) -> Result<(), String> {
        unsafe {
            gl::LinkProgram(self.0);
        }

        let state = unsafe {
            let mut state: GLint = 0;
            gl::GetProgramiv(self.0, gl::LINK_STATUS, &mut state as *mut GLint);
            state
        };

        if state == gl::FALSE as GLint {
            let len = unsafe {
                let mut len: GLint = 0;
                gl::GetProgramiv(self.0, gl::INFO_LOG_LENGTH, &mut len as *mut GLint);
                len as usize
            };

            let mut buf = vec![0u8; len];
            unsafe {
                gl::GetProgramInfoLog(self.0, len as GLsizei, 0 as *mut _, buf.as_mut_ptr() as *mut GLchar);
            }

            Err(String::from_utf8_lossy(&buf).to_string())
        } else {
            Ok(())
        }
    }

    pub fn name(&self) -> GLuint {
        self.0
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.0)
        }
    }
}

