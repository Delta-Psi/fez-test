use gl::types::*;
use cgmath::prelude::*;

use cgmath::{Matrix4, Vector3};

static VERTEX_SHADER: &str = r#"
#version 150 core

in vec3 position;
in vec3 normal;
in vec2 tex_coord;

out vec3 Normal;
out vec3 FragPos;
out vec2 TexCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

void main()
{
        gl_Position = proj * view * model * vec4(position, 1.0);
        Normal = mat3(transpose(inverse(model))) * normal;
        FragPos = vec3(model * vec4(position, 1.0));
        TexCoord = tex_coord;
}
"#;

static FRAGMENT_SHADER: &str = r#"
#version 150 core

in vec3 Normal;
in vec3 FragPos;
in vec2 TexCoord;

out vec4 outColor;

uniform sampler2D tex;

void main()
{
        vec3 lightPos = vec3(1.5, 1.5, 1.0);
        vec3 norm = normalize(Normal);
        vec3 lightDir = normalize(lightPos - FragPos);
        float diffuse = 0.8*max(dot(norm, lightDir), 0.0);
        float ambient = 0.2;

        //outColor = vec4((ambient + diffuse)*vec3(1.0, 0.0, 0.0), 1.0);
        //outColor = vec4(((Normal+1.0)/2.0), 1.0);
        //outColor = vec4(norm, 1.0);
        outColor = texture(tex, TexCoord * (1.0, -1.0));
}
"#;

// cube
pub static CUBE_VERTICES: &[GLfloat] = &[
    -0.5, -0.5, -0.5,   0.0,  0.0, -1.0,   0.0,  0.0,
     0.5, -0.5, -0.5,   0.0,  0.0, -1.0,   0.0,  0.0,
     0.5,  0.5, -0.5,   0.0,  0.0, -1.0,   0.0,  0.0,
     0.5,  0.5, -0.5,   0.0,  0.0, -1.0,   0.0,  0.0,
    -0.5,  0.5, -0.5,   0.0,  0.0, -1.0,   0.0,  0.0,
    -0.5, -0.5, -0.5,   0.0,  0.0, -1.0,   0.0,  0.0,

    -0.5, -0.5,  0.5,   0.0,  0.0,  1.0,   0.0,  0.0,
     0.5, -0.5,  0.5,   0.0,  0.0,  1.0,   0.0,  0.0,
     0.5,  0.5,  0.5,   0.0,  0.0,  1.0,   0.0,  0.0,
     0.5,  0.5,  0.5,   0.0,  0.0,  1.0,   0.0,  0.0,
    -0.5,  0.5,  0.5,   0.0,  0.0,  1.0,   0.0,  0.0,
    -0.5, -0.5,  0.5,   0.0,  0.0,  1.0,   0.0,  0.0,

    -0.5,  0.5,  0.5,  -1.0,  0.0,  0.0,   1.0,  1.0,
    -0.5,  0.5, -0.5,  -1.0,  0.0,  0.0,   1.0,  0.0,
    -0.5, -0.5, -0.5,  -1.0,  0.0,  0.0,   0.0,  0.0,
    -0.5, -0.5, -0.5,  -1.0,  0.0,  0.0,   0.0,  0.0,
    -0.5, -0.5,  0.5,  -1.0,  0.0,  0.0,   0.0,  1.0,
    -0.5,  0.5,  0.5,  -1.0,  0.0,  0.0,   1.0,  1.0,

     0.5,  0.5,  0.5,   1.0,  0.0,  0.0,   0.0,  1.0,
     0.5,  0.5, -0.5,   1.0,  0.0,  0.0,   0.0,  0.0,
     0.5, -0.5, -0.5,   1.0,  0.0,  0.0,   1.0,  0.0,
     0.5, -0.5, -0.5,   1.0,  0.0,  0.0,   1.0,  0.0,
     0.5, -0.5,  0.5,   1.0,  0.0,  0.0,   1.0,  1.0,
     0.5,  0.5,  0.5,   1.0,  0.0,  0.0,   0.0,  1.0,

    -0.5, -0.5, -0.5,   0.0, -1.0,  0.0,   1.0,  0.0,
     0.5, -0.5, -0.5,   0.0, -1.0,  0.0,   0.0,  0.0,
     0.5, -0.5,  0.5,   0.0, -1.0,  0.0,   0.0,  1.0,
     0.5, -0.5,  0.5,   0.0, -1.0,  0.0,   0.0,  1.0,
    -0.5, -0.5,  0.5,   0.0, -1.0,  0.0,   1.0,  1.0,
    -0.5, -0.5, -0.5,   0.0, -1.0,  0.0,   1.0,  0.0,

    -0.5,  0.5, -0.5,   0.0,  1.0,  0.0,   0.0,  0.0,
     0.5,  0.5, -0.5,   0.0,  1.0,  0.0,   1.0,  0.0,
     0.5,  0.5,  0.5,   0.0,  1.0,  0.0,   1.0,  1.0,
     0.5,  0.5,  0.5,   0.0,  1.0,  0.0,   1.0,  1.0,
    -0.5,  0.5,  0.5,   0.0,  1.0,  0.0,   0.0,  1.0,
    -0.5,  0.5, -0.5,   0.0,  1.0,  0.0,   0.0,  0.0,
];

pub static TEST_PNG: &[u8] = include_bytes!("tex/test.png");

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

    pub test_texture: Texture,
}

impl Resources {
    pub fn new() -> Resources {
        // initialize all opengl data
        let vertex_shader = Shader::compile(ShaderType::Vertex, VERTEX_SHADER).unwrap();
        let fragment_shader = Shader::compile(ShaderType::Fragment, FRAGMENT_SHADER).unwrap();

        let shader_program = ShaderProgram::new();
        shader_program.attach(&vertex_shader);
        shader_program.attach(&fragment_shader);
        shader_program.link().unwrap();
        unsafe {
            gl::UseProgram(shader_program.name());
        }

        let cube_vertices = unsafe {
            let vbo = BufferObject::new();

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo.name());
            gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(CUBE_VERTICES) as _, CUBE_VERTICES.as_ptr() as _, gl::STATIC_DRAW);

            vbo
        };

        let vao = VertexArrayObject::new();
        unsafe {
            use std::mem::size_of;
            let vao = vao.name();

            gl::BindVertexArray(vao);

            let pos_attrib = gl::GetAttribLocation(shader_program.name(), c_str!("position").as_ptr()) as u32;
            gl::VertexAttribPointer(pos_attrib, 3, gl::FLOAT, gl::FALSE, 8*size_of::<GLfloat>() as i32, 0 as *mut _);
            gl::EnableVertexAttribArray(pos_attrib);

            /*let normal_attrib = gl::GetAttribLocation(shader_program.name(), c_str!("normal").as_ptr()) as u32;
            gl::VertexAttribPointer(normal_attrib, 3, gl::FLOAT, gl::FALSE, 8*size_of::<GLfloat>() as i32, (3*size_of::<GLfloat>()) as *mut _);
            gl::EnableVertexAttribArray(normal_attrib);*/

            let tex_coord_attrib = gl::GetAttribLocation(shader_program.name(), c_str!("tex_coord").as_ptr()) as u32;
            gl::VertexAttribPointer(tex_coord_attrib, 2, gl::FLOAT, gl::FALSE, 8*size_of::<GLfloat>() as i32, (6*size_of::<GLfloat>()) as *mut _);
            gl::EnableVertexAttribArray(tex_coord_attrib);

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
        //let proj = cgmath::perspective(cgmath::Deg(45.0), ASPECT_RATIO, 1.0, 10.0);
        // orthogonal (w fixed aspect ratio)
        let proj =
            cgmath::Matrix4::from_nonuniform_scale(1.0/ASPECT_RATIO, 1.0, 1.0)
            *
            cgmath::ortho(
                -8.0, 8.0,
                -8.0, 8.0,
                -100.0, 100.0,
            );
        unsafe {
            gl::UniformMatrix4fv(unif_proj, 1, gl::FALSE, proj.as_ptr());
        }

        // decode test texture
        use image::ImageDecoder;
        let test_decoder = image::png::PngDecoder::new(std::io::Cursor::new(TEST_PNG)).unwrap();
        let (test_w, test_h) = test_decoder.dimensions();
        let test_image = image::DynamicImage::from_decoder(test_decoder).unwrap();
        let test_data = test_image.to_rgb().into_raw();

        let test_texture = Texture::new();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, test_texture.name());
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as GLint, test_w as GLint, test_h as GLint, 0, gl::RGB, gl::UNSIGNED_BYTE, test_data.as_ptr() as *const _);
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
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

            test_texture,
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

    pub fn draw_platform(&self, surface_center: Vector3<f32>, surface_dim: (f32, f32), height: f32) {
        let scale = Matrix4::from_nonuniform_scale(surface_dim.0, surface_dim.1, height);
        let translate = Matrix4::from_translation(surface_center - Vector3::new(0.0, 0.0, height/2.0));

        let transform = translate*scale;

        unsafe {
            // gl::UseProgram etc
            gl::UniformMatrix4fv(self.unif_model, 1, gl::FALSE, transform.as_ptr());
            // gl::BindBuffer etc
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}
