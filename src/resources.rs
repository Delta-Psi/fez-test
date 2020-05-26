use gl::types::*;
use cgmath::prelude::*;

use cgmath::{Matrix4, Vector3};

mod vertex_data;
mod shader_sources;

pub static TEST_PNG: &[u8] = include_bytes!("tex/test.png");

use crate::gfx::*;
use crate::c_str;

pub struct Resources {
    platform_faces: BufferObject,
    platform_faces_vao: VertexArrayObject,

    platform_edges: BufferObject,
    platform_edges_vao: VertexArrayObject,

    square_faces: BufferObject,
    square_faces_vao: VertexArrayObject,

    shader_program: ShaderProgram,

    unif_model: GLint,
    unif_view: GLint,
    unif_proj: GLint,
    unif_color: GLint,
    unif_apply_diffuse: GLint,

    test_texture: Texture,

    camera_inverse_rotation: Matrix4<f32>,
}

impl Resources {
    pub fn new() -> Resources {
        // initialize all opengl data
        let vertex_shader = Shader::compile(ShaderType::Vertex, shader_sources::SOLID_VERTEX_SHADER).unwrap();
        let fragment_shader = Shader::compile(ShaderType::Fragment, shader_sources::SOLID_FRAGMENT_SHADER).unwrap();

        let shader_program = ShaderProgram::new();
        shader_program.attach(&vertex_shader);
        shader_program.attach(&fragment_shader);
        shader_program.link().unwrap();
        shader_program.use_();

        let pos_attrib = unsafe {
            gl::GetAttribLocation(shader_program.name(), c_str!("inPosition").as_ptr()) as u32
        };
        let normal_attrib = unsafe {
            gl::GetAttribLocation(shader_program.name(), c_str!("inNormal").as_ptr()) as u32
        };

        use std::mem::size_of_val;

        let platform_faces = BufferObject::new();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, platform_faces.name());
            gl::BufferData(gl::ARRAY_BUFFER, size_of_val(vertex_data::PLATFORM_FACES) as GLsizeiptr, vertex_data::PLATFORM_FACES.as_ptr() as *const _, gl::STATIC_DRAW);
        }

        let platform_faces_vao = VertexArrayObject::new();
        platform_faces_vao.bind();
        unsafe {
            gl::VertexAttribPointer(pos_attrib, 3, gl::FLOAT, gl::FALSE, size_of_val(&vertex_data::PLATFORM_FACES[0]) as GLint, std::ptr::null_mut());
            gl::EnableVertexAttribArray(pos_attrib);

            gl::VertexAttribPointer(normal_attrib, 3, gl::FLOAT, gl::FALSE, size_of_val(&vertex_data::PLATFORM_FACES[0]) as GLint, std::ptr::null_mut::<GLfloat>().offset(3) as *mut _);
            gl::EnableVertexAttribArray(normal_attrib);
        }

        let platform_edges = BufferObject::new();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, platform_edges.name());
            gl::BufferData(gl::ARRAY_BUFFER, size_of_val(vertex_data::PLATFORM_EDGES) as GLsizeiptr, vertex_data::PLATFORM_EDGES.as_ptr() as *const _, gl::STATIC_DRAW);
        }

        let platform_edges_vao = VertexArrayObject::new();
        platform_edges_vao.bind();
        unsafe {
            gl::VertexAttribPointer(pos_attrib, 3, gl::FLOAT, gl::FALSE, size_of_val(&vertex_data::PLATFORM_EDGES[0]) as GLint, std::ptr::null_mut());
            gl::EnableVertexAttribArray(pos_attrib);
        }

        let square_faces = unsafe {
            let vbo = BufferObject::new();

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo.name());
            gl::BufferData(gl::ARRAY_BUFFER, size_of_val(vertex_data::SQUARE_FACES) as GLsizeiptr, vertex_data::SQUARE_FACES.as_ptr() as *const _, gl::STATIC_DRAW);

            vbo
        };

        let square_faces_vao = VertexArrayObject::new();
        square_faces_vao.bind();
        unsafe {
            gl::VertexAttribPointer(pos_attrib, 3, gl::FLOAT, gl::FALSE, size_of_val(&vertex_data::SQUARE_FACES[0]) as GLint, std::ptr::null_mut());
            gl::EnableVertexAttribArray(pos_attrib);
        }

        let unif_model = shader_program.get_uniform_location(c_str!("model"));
        let unif_view = shader_program.get_uniform_location(c_str!("view"));
        let unif_proj = shader_program.get_uniform_location(c_str!("proj"));
        let unif_color = shader_program.get_uniform_location(c_str!("color"));
        let unif_apply_diffuse = shader_program.get_uniform_location(c_str!("apply_diffuse"));

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
        let test_texture = Texture::load_from_png(std::io::Cursor::new(TEST_PNG));
        unsafe {
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        }

        unsafe {
            assert_eq!(gl::GetError(), 0);
        }

        Resources {
            platform_faces,
            platform_faces_vao,

            platform_edges,
            platform_edges_vao,

            square_faces,
            square_faces_vao,

            shader_program,

            unif_model,
            unif_view,
            unif_proj,
            unif_color,
            unif_apply_diffuse,

            test_texture,

            camera_inverse_rotation: Matrix4::identity(),
        }
    }

    pub fn clear(&self, color: (f32, f32, f32)) {
        unsafe {
            gl::ClearColor(color.0, color.1, color.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn set_camera_matrices(&mut self, view: Matrix4<f32>, inverse_rotation: Matrix4<f32>) {
        self.camera_inverse_rotation = inverse_rotation;

        unsafe {
            gl::UniformMatrix4fv(self.unif_view, 1, gl::FALSE, view.as_ptr());
        }
    }

    pub fn draw_platform(&self, surface_center: Vector3<f32>, surface_dim: (f32, f32), height: f32, color: (f32, f32, f32)) {
        let scale = Matrix4::from_nonuniform_scale(surface_dim.0, surface_dim.1, height);
        let translate = Matrix4::from_translation(surface_center - Vector3::new(0.0, 0.0, height/2.0));

        let transform = translate*scale;

        unsafe {
            gl::UniformMatrix4fv(self.unif_model, 1, gl::FALSE, transform.as_ptr());
            gl::Uniform3f(self.unif_color, color.0, color.1, color.2);
            gl::Uniform1i(self.unif_apply_diffuse, 1);

            gl::BindVertexArray(self.platform_faces_vao.name());
            gl::DrawArrays(gl::TRIANGLES, 0, vertex_data::PLATFORM_FACES.len() as GLint);

            
            gl::Uniform3f(self.unif_color, 0.0, 0.0, 0.0);
            gl::Uniform1i(self.unif_apply_diffuse, 1);
            
            gl::BindVertexArray(self.platform_edges_vao.name());
            gl::DrawArrays(gl::LINES, 0, vertex_data::PLATFORM_EDGES.len() as GLint);
        }
    }

    pub fn draw_square(&self, base: Vector3<f32>, side: f32, color: (f32, f32, f32)) {
        let scale = Matrix4::from_scale(side);
        let rotate = Matrix4::from_angle_x(cgmath::Deg(90.0));
        let rotate2 = &self.camera_inverse_rotation;
        let translate = Matrix4::from_translation(base + Vector3::new(0.0, 0.0, side/2.0));

        let transform = translate*rotate2*rotate*scale;

        unsafe {
            gl::UniformMatrix4fv(self.unif_model, 1, gl::FALSE, transform.as_ptr());
            gl::Uniform3f(self.unif_color, color.0, color.1, color.2);
            gl::Uniform1i(self.unif_apply_diffuse, 0);

            gl::BindVertexArray(self.square_faces_vao.name());
            gl::DrawArrays(gl::TRIANGLES, 0, vertex_data::SQUARE_FACES.len() as GLint);
        }
    }
}
