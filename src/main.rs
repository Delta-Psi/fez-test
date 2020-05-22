use std::time::Instant;
use gl::types::*;

// cube
static VERTICES: &[GLfloat] = &[
    0.0, 0.5,
    0.5, -0.5,
    -0.5, -0.5];
/*
    -1., -1., -1.,
    -1., -1.,  1.,
    -1.,  1., -1.,
    -1.,  1.,  1.,
     1., -1., -1.,
     1., -1.,  1.,
     1.,  1., -1.,
     1.,  1.,  1.,
];*/
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

in vec2 position;

void main()
{
        gl_Position = vec4(position, 0.0, 1.0);
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
            gl::VertexAttribPointer(pos_attrib, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(pos_attrib);
        }

        unsafe {
            assert_eq!(gl::GetError(), 0);
        }

        Game {
            last_tick: Instant::now(),
        }
    }

    pub fn tick(&mut self) {
        // update timing
        let current_tick = Instant::now();
        let _delta = current_tick.duration_since(self.last_tick);
        self.last_tick = current_tick;

        //println!("{} FPS", 1.0/delta.as_secs_f64());
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawArrays(gl::LINE_LOOP, 0, 3);
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
