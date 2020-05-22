use std::time::Instant;
use gl::types::*;

// cube
static VERTICES: &[GLfloat] = &[
    -0.5, -0.5, -0.5,
    -0.5, -0.5,  0.5,
    -0.5,  0.5, -0.5,
    -0.5,  0.5,  0.5,
     0.5, -0.5, -0.5,
     0.5, -0.5,  0.5,
     0.5,  0.5, -0.5,
     0.5,  0.5,  0.5,
];
static EDGES: &[GLuint] = &[
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

struct Game {
    last_tick: Instant,
    timer: f32,

    uni_model: GLint,
}

impl Game {
    pub fn new() -> Game {
        // initialize all opengl data
        use std::ffi::CString;

        let _vertices = unsafe {
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

        unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao as *mut _);
            gl::BindVertexArray(vao);

            let pos_attrib = gl::GetAttribLocation(shader_program, CString::new("position").unwrap().as_ptr()) as u32;
            gl::VertexAttribPointer(pos_attrib, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(pos_attrib);
        }

        let _elements = unsafe {
            let mut ebo = 0;

            gl::GenBuffers(1, &mut ebo as *mut _);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, std::mem::size_of_val(EDGES) as _, EDGES.as_ptr() as _, gl::STATIC_DRAW);

            ebo
        };

        let uni_model = unsafe {
            gl::GetUniformLocation(shader_program, CString::new("model").unwrap().as_ptr())
        };
        let uni_view = unsafe {
            gl::GetUniformLocation(shader_program, CString::new("view").unwrap().as_ptr())
        };
        let uni_proj = unsafe {
            gl::GetUniformLocation(shader_program, CString::new("proj").unwrap().as_ptr())
        };

        // matrix transformations
        use cgmath::prelude::*;
        //let model = cgmath::Matrix4::identity();
        let model: cgmath::Matrix4<f32> = cgmath::Quaternion::from_angle_z(cgmath::Deg(10.5)).into();
        println!("{:?}", model);
        let view = cgmath::Matrix4::look_at((1.5, 1.5, 1.5).into(), (0.0, 0.0, 0.0).into(), (0.0, 0.0, 1.0).into());
        let proj = cgmath::perspective(cgmath::Deg(45.0), 640.0/480.0, 1.0, 10.0);
        unsafe {
            gl::UniformMatrix4fv(uni_model, 1, gl::FALSE, model.as_ptr());
            gl::UniformMatrix4fv(uni_view, 1, gl::FALSE, view.as_ptr());
            gl::UniformMatrix4fv(uni_proj, 1, gl::FALSE, proj.as_ptr());
        }

        unsafe {
            assert_eq!(gl::GetError(), 0);
        }

        Game {
            last_tick: Instant::now(),
            timer: 0.0,

            uni_model,
        }
    }

    pub fn tick(&mut self) {
        // update timing
        let current_tick = Instant::now();
        let delta = current_tick.duration_since(self.last_tick);
        self.last_tick = current_tick;
        self.timer += delta.as_secs_f32();

        const SPEED: f32 = 260.0; // degrees per second
        let angle = cgmath::Deg(self.timer * SPEED);

        use cgmath::prelude::*;
        let model: cgmath::Matrix4<_> = cgmath::Quaternion::from_angle_z(angle).into();
        unsafe {
            gl::UniformMatrix4fv(self.uni_model, 1, gl::FALSE, model.as_ptr());
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawElements(gl::LINES, EDGES.len() as GLint, gl::UNSIGNED_INT, 0 as *const _);
        }
    }
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let context = glutin::ContextBuilder::new()
        .build_windowed(glutin::window::WindowBuilder::new()
            .with_title("fez test")
            .with_inner_size(glutin::dpi::LogicalSize::new(640.0, 480.0))
            .with_resizable(false)
            , &event_loop)
        .unwrap();

    let context = unsafe {
        context.make_current().unwrap()
    };
    
    // initialize opengl
    gl::load_with(|s| context.get_proc_address(s) as *const _);

    let mut game = Game::new();

    event_loop.run(move |event, _, control_flow| {
        use glutin::event_loop::ControlFlow;
        // continously runs the event loop
        *control_flow = ControlFlow::Poll;

        use glutin::event::{Event, WindowEvent};
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                context.resize(size);
            },

            Event::MainEventsCleared => {
                game.tick();

                context.window().request_redraw();
            },

            Event::RedrawRequested(_) => {
                // etc
                game.draw();

                context.swap_buffers().unwrap();
            },

            _ => (),
        };
    });
}
