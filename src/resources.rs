use gl::types::*;
use cgmath::prelude::*;

use cgmath::{Matrix4, Vector3};

// cube
pub static CUBE_VERTICES: &[GLfloat] = &[
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

        //outColor = vec4((ambient + diffuse)*vec3(1.0, 0.0, 0.0), 1.0);
        outColor = vec4(((Normal+1.0)/2.0), 1.0);
}
"#;

use crate::gfx::*;
use crate::c_str;

pub struct Resources {
    pub cube_vertices: BufferObject,

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
        let cube_vertices = unsafe {
            let vbo = BufferObject::new();

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo.name());
            gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(CUBE_VERTICES) as _, CUBE_VERTICES.as_ptr() as _, gl::STATIC_DRAW);

            vbo
        };

        let vertex_shader = Shader::compile(ShaderType::Vertex, VERTEX_SHADER).unwrap();
        let fragment_shader = Shader::compile(ShaderType::Fragment, FRAGMENT_SHADER).unwrap();

        let shader_program = ShaderProgram::new();
        shader_program.attach(&vertex_shader);
        shader_program.attach(&fragment_shader);
        shader_program.link().unwrap();
        unsafe {
            gl::UseProgram(shader_program.name());
        }

        let vao = VertexArrayObject::new();
        unsafe {
            gl::BindVertexArray(vao.name());
        }

        unsafe {
            let vao = vao.name();

            let pos_attrib = gl::GetAttribLocation(shader_program.name(), c_str!("position").as_ptr()) as u32;
            gl::VertexAttribPointer(pos_attrib, 3, gl::FLOAT, gl::FALSE, 6*std::mem::size_of::<GLfloat>() as i32, 0 as *mut _);
            gl::EnableVertexAttribArray(pos_attrib);

            let normal_attrib = gl::GetAttribLocation(shader_program.name(), c_str!("normal").as_ptr()) as u32;
            gl::VertexAttribPointer(normal_attrib, 3, gl::FLOAT, gl::FALSE, 6*std::mem::size_of::<GLfloat>() as i32, (3*std::mem::size_of::<GLfloat>()) as *mut _);
            gl::EnableVertexAttribArray(normal_attrib);

            vao
        };

        let unif_model = unsafe {
            gl::GetUniformLocation(shader_program.name(), c_str!("model").as_ptr())
        };
        let unif_view = unsafe {
            gl::GetUniformLocation(shader_program.name(), c_str!("view").as_ptr())
        };
        let unif_proj = unsafe {
            gl::GetUniformLocation(shader_program.name(), c_str!("proj").as_ptr())
        };

        const ASPECT_RATIO: f32 = 640.0 / 480.0;

        // matrix transformations
        use cgmath::prelude::*;
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
            gl::UniformMatrix4fv(unif_proj, 1, gl::FALSE, proj.as_ptr());
        }

        unsafe {
            assert_eq!(gl::GetError(), 0);
        }

        Resources {
            cube_vertices,

            vertex_shader,
            fragment_shader,
            shader_program,
            vao,

            unif_model,
            unif_view,
            unif_proj,
        }
    }

    pub fn set_view_matrix(&self, matrix: &Matrix4<f32>) {
        unsafe {
            //gl::UseProgram etc
            gl::UniformMatrix4fv(self.unif_view, 1, gl::FALSE, matrix.as_ptr());
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn draw_cube(&self, center: Vector3<f32>, side: f32) {
        let scale = Matrix4::from_scale(side);
        let translate = Matrix4::from_translation(-center);
        let model_matrix = translate*scale;

        unsafe {
            // gl::UseProgram etc
            gl::UniformMatrix4fv(self.unif_model, 1, gl::FALSE, model_matrix.as_ptr());
            // gl::BindBuffer etc
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}
