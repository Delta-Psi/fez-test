use gl::types::*;

// cube
pub static VERTICES: &[GLfloat] = &[
    -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
    0.5, -0.5, -0.5,  0.0,  0.0, -1.0, 
    0.5,  0.5, -0.5,  0.0,  0.0, -1.0, 
    0.5,  0.5, -0.5,  0.0,  0.0, -1.0, 
    -0.5,  0.5, -0.5,  0.0,  0.0, -1.0, 
    -0.5, -0.5, -0.5,  0.0,  0.0, -1.0, 

    -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,
    0.5, -0.5,  0.5,  0.0,  0.0, 1.0,
    0.5,  0.5,  0.5,  0.0,  0.0, 1.0,
    0.5,  0.5,  0.5,  0.0,  0.0, 1.0,
    -0.5,  0.5,  0.5,  0.0,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,

    -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,
    -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,
    -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
    -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
    -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,
    -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,

    0.5,  0.5,  0.5,  1.0,  0.0,  0.0,
    0.5,  0.5, -0.5,  1.0,  0.0,  0.0,
    0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
    0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
    0.5, -0.5,  0.5,  1.0,  0.0,  0.0,
    0.5,  0.5,  0.5,  1.0,  0.0,  0.0,

    -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
    0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
    0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
    0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
    -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
    -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,

    -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
    0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
    0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
    0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
    -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
    -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
];

static VERTEX_SHADER: &str = r#"
#version 150 core

in vec3 position;
in vec3 normal;

out vec3 Normal;
out vec3 FragPos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

void main()
{
        gl_Position = proj * view * model * vec4(position, 1.0);
        Normal = mat3(transpose(inverse(model))) * normal;
        FragPos = vec3(model * vec4(position, 1.0));
}
"#;

static FRAGMENT_SHADER: &str = r#"
#version 150 core

in vec3 Normal;
in vec3 FragPos;

out vec4 outColor;

void main()
{
        vec3 lightPos = vec3(1.5, 1.5, 1.0);
        vec3 norm = normalize(Normal);
        vec3 lightDir = normalize(lightPos - FragPos);
        float diffuse = 0.8*max(dot(norm, lightDir), 0.0);
        float ambient = 0.2;

        outColor = vec4((ambient + diffuse)*vec3(1.0, 0.0, 0.0), 1.0);
}
"#;

use crate::gfx::{Shader, ShaderType, ShaderProgram, VertexArrayObject};

pub struct Resources {
    pub box_vertices: GLuint,

    pub vertex_shader: Shader,
    pub fragment_shader: Shader,
    pub shader_program: ShaderProgram,
    pub vao: VertexArrayObject,

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

        let vertex_shader = Shader::compile(ShaderType::Vertex, VERTEX_SHADER).unwrap();
        let fragment_shader = Shader::compile(ShaderType::Fragment, FRAGMENT_SHADER).unwrap();

        let shader_program = ShaderProgram::new();
        shader_program.attach(&vertex_shader);
        shader_program.attach(&fragment_shader);
        shader_program.link().unwrap();
        shader_program.use_();

        let vao = VertexArrayObject::new();
        vao.bind();

        unsafe {
            let vao = vao.gl_handle();

            let pos_attrib = gl::GetAttribLocation(shader_program.gl_handle(), CString::new("position").unwrap().as_ptr()) as u32;
            gl::VertexAttribPointer(pos_attrib, 3, gl::FLOAT, gl::FALSE, 6*std::mem::size_of::<GLfloat>() as i32, 0 as *mut _);
            gl::EnableVertexAttribArray(pos_attrib);

            let normal_attrib = gl::GetAttribLocation(shader_program.gl_handle(), CString::new("normal").unwrap().as_ptr()) as u32;
            gl::VertexAttribPointer(normal_attrib, 3, gl::FLOAT, gl::FALSE, 6*std::mem::size_of::<GLfloat>() as i32, (3*std::mem::size_of::<GLfloat>()) as *mut _);
            gl::EnableVertexAttribArray(normal_attrib);

            vao
        };

        let unif_model = unsafe {
            gl::GetUniformLocation(shader_program.gl_handle(), CString::new("model").unwrap().as_ptr())
        };
        let unif_view = unsafe {
            gl::GetUniformLocation(shader_program.gl_handle(), CString::new("view").unwrap().as_ptr())
        };
        let unif_proj = unsafe {
            gl::GetUniformLocation(shader_program.gl_handle(), CString::new("proj").unwrap().as_ptr())
        };

        const ASPECT_RATIO: f32 = 640.0 / 480.0;

        // matrix transformations
        use cgmath::prelude::*;
        let model = cgmath::Matrix4::identity();
        //let view = cgmath::Matrix4::look_at((0.0, 1.0, 0.0).into(), (0.0, 0.0, 0.0).into(), (0.0, 0.0, 1.0).into());
        let view = cgmath::Matrix4::look_at((1.5, 1.5, 1.5).into(), (0.0, 0.0, 0.0).into(), (0.0, 0.0, 1.0).into());
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
            gl::DeleteBuffers(1, &self.box_vertices as *const _);
        }
    }
}
