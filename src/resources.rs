use gl::types::*;

// cube
pub static VERTICES: &[GLfloat] = &[
    -0.5, -0.5, -0.5,
    -0.5, -0.5,  0.5,
    -0.5,  0.5, -0.5,
    -0.5,  0.5,  0.5,
     0.5, -0.5, -0.5,
     0.5, -0.5,  0.5,
     0.5,  0.5, -0.5,
     0.5,  0.5,  0.5,
];
pub static EDGES: &[GLuint] = &[
    0, 1,
    0, 2,
    0, 4,
    3, 1,
    3, 2,
    3, 7,
    5, 1,
    5, 4,
    5, 7,
    6, 2,
    6, 4,
    6, 7,
];
pub static FACES: &[GLuint] = &[
    0, 1, 2,
    1, 2, 3,

    0, 2, 4,
    2, 4, 6,

    0, 1, 4,
    1, 4, 5,

    7, 6, 5,
    6, 5, 4,

    7, 5, 3,
    5, 3, 1,

    7, 6, 3,
    6, 3, 2,
];

static VERTEX_SHADER: &str = r#"
#version 150 core

in vec3 position;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

void main()
{
        gl_Position = proj * view * model * vec4(position, 1.0);
}
"#;

static FRAGMENT_SHADER: &str = r#"
#version 150 core

out vec4 outColor;

void main()
{
        outColor = vec4(1.0, 0.0, 0.0, 1.0);
}
"#;

pub struct Resources {
    pub box_vertices: GLuint,
    pub box_edges: GLuint,
    pub box_faces: GLuint,

    pub vertex_shader: GLuint,
    pub fragment_shader: GLuint,
    pub shader_program: GLuint,
    pub vao: GLuint,

    pub unif_model: GLint,
    pub unif_view: GLint,
    pub unif_proj: GLint,
}

impl Resources {
    pub fn new() -> Resources {
        // initialize all opengl data
        use std::ffi::CString;

        let box_vertices = unsafe {
            let mut vbo = 0;

            gl::GenBuffers(1, &mut vbo as *mut _);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(VERTICES) as _, VERTICES.as_ptr() as _, gl::STATIC_DRAW);

            vbo
        };

        let vertex_shader = unsafe {
            let shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(shader, 1, &CString::new(VERTEX_SHADER).unwrap().as_ptr() as *const _, std::ptr::null());
            gl::CompileShader(shader);

            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status as *mut _);
            assert_eq!(status, gl::TRUE as GLint);

            shader
        };

        let fragment_shader = unsafe {
            let shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(shader, 1, &CString::new(FRAGMENT_SHADER).unwrap().as_ptr() as *const _, std::ptr::null());
            gl::CompileShader(shader);

            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status as *mut _);
            assert_eq!(status, gl::TRUE as GLint);

            shader
        };

        let shader_program = unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);

            gl::BindFragDataLocation(program, 0, CString::new("outColor").unwrap().as_ptr());

            gl::LinkProgram(program);
            gl::UseProgram(program);

            program
        };

        let vao = unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao as *mut _);
            gl::BindVertexArray(vao);

            let pos_attrib = gl::GetAttribLocation(shader_program, CString::new("position").unwrap().as_ptr()) as u32;
            gl::VertexAttribPointer(pos_attrib, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(pos_attrib);

            vao
        };

        let box_edges = unsafe {
            let mut ebo = 0;

            gl::GenBuffers(1, &mut ebo as *mut _);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, std::mem::size_of_val(EDGES) as _, EDGES.as_ptr() as _, gl::STATIC_DRAW);

            ebo
        };

        let box_faces = unsafe {
            let mut ebo = 0;

            gl::GenBuffers(1, &mut ebo as *mut _);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, std::mem::size_of_val(FACES) as _, FACES.as_ptr() as _, gl::STATIC_DRAW);

            ebo
        };

        let unif_model = unsafe {
            gl::GetUniformLocation(shader_program, CString::new("model").unwrap().as_ptr())
        };
        let unif_view = unsafe {
            gl::GetUniformLocation(shader_program, CString::new("view").unwrap().as_ptr())
        };
        let unif_proj = unsafe {
            gl::GetUniformLocation(shader_program, CString::new("proj").unwrap().as_ptr())
        };

        const ASPECT_RATIO: f32 = 640.0 / 480.0;

        // matrix transformations
        use cgmath::prelude::*;
        let model = cgmath::Matrix4::identity();
        let view = cgmath::Matrix4::look_at((0.0, 1.0, 0.0).into(), (0.0, 0.0, 0.0).into(), (0.0, 0.0, 1.0).into());
        //let proj = cgmath::perspective(cgmath::Deg(45.0), 640.0/480.0, 1.0, 10.0);
        // orthogonal (w fixed aspect ratio)
        let proj =
            cgmath::Matrix4::from_nonuniform_scale(1.0, ASPECT_RATIO, 1.0)
            *
            cgmath::ortho(
                -1.5, 1.5,
                -1.5, 1.5,
                -10.0, 10.0,
            );
        unsafe {
            gl::UniformMatrix4fv(unif_model, 1, gl::FALSE, model.as_ptr());
            gl::UniformMatrix4fv(unif_view, 1, gl::FALSE, view.as_ptr());
            gl::UniformMatrix4fv(unif_proj, 1, gl::FALSE, proj.as_ptr());
        }

        unsafe {
            assert_eq!(gl::GetError(), 0);
        }

        Resources {
            box_vertices,
            box_edges,
            box_faces,

            vertex_shader,
            fragment_shader,
            shader_program,
            vao,

            unif_model,
            unif_view,
            unif_proj,
        }
    }
}

impl Drop for Resources {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.box_faces as *const _);
            gl::DeleteBuffers(1, &self.box_edges as *const _);

            gl::DeleteVertexArrays(1, &self.vao as *const _);
            gl::DeleteProgram(self.shader_program);
            gl::DeleteShader(self.fragment_shader);
            gl::DeleteShader(self.vertex_shader);

            gl::DeleteBuffers(1, &self.box_vertices as *const _);
        }
    }
}
