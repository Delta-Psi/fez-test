use std::time::Instant;

mod macros;
pub mod gfx;
pub mod resources;
use resources::Resources;

struct Game {
    res: Resources,

    last_tick: Instant,
    timer: f32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            res: Resources::new(),

            last_tick: Instant::now(),
            timer: 0.0,
        }
    }

    pub fn tick(&mut self) {
        // update timing
        let current_tick = Instant::now();
        let delta = current_tick.duration_since(self.last_tick);
        self.last_tick = current_tick;
        self.timer += delta.as_secs_f32();

        const SPEED: f32 = 90.0; // degrees per second
        let angle = cgmath::Deg(self.timer * SPEED);

        use cgmath::prelude::*;
        let model: cgmath::Matrix4<_> = cgmath::Quaternion::from_angle_z(angle).into();
        unsafe {
            gl::UniformMatrix4fv(self.res.unif_model, 1, gl::FALSE, model.as_ptr());
        }
    }

    pub fn draw(&self) {

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::DrawArrays(gl::TRIANGLES, 0, 36);
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
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

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
