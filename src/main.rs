mod macros;
pub mod gfx;
pub mod resources;
use resources::Resources;

mod camera;
use crate::camera::*;

use std::time::Instant;

struct Game {
    res: Resources,

    camera: Camera,

    last_tick: Instant,
}

impl Game {
    pub fn new() -> Game {
        Game {
            res: Resources::new(),

            camera: Camera::new(),

            last_tick: Instant::now(),
        }
    }

    pub fn move_camera(&mut self, dir: CameraDirection) {
        self.camera.move_(dir);
    }

    pub fn tick(&mut self) {
        // update timing
        let current_tick = Instant::now();
        let delta = current_tick.duration_since(self.last_tick).as_secs_f32();
        self.last_tick = current_tick;

        self.camera.tick(delta);
    }

    pub fn draw(&self) {
        self.res.clear();

        self.res.set_view_matrix(&self.camera.view_matrix());
        self.res.draw_cube((0.25, 0.25, 0.25).into(), 0.5);
        self.res.draw_cube((-0.25, -0.25, -0.25).into(), 0.5);
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

            Event::WindowEvent { event: WindowEvent::KeyboardInput {input, ..}, .. } => {
                use glutin::event::ElementState;

                if input.state == ElementState::Pressed {
                    if let Some(keycode) = input.virtual_keycode {
                        use glutin::event::VirtualKeyCode::*;

                        match keycode {
                            A => game.move_camera(CameraDirection::L),
                            D => game.move_camera(CameraDirection::R),

                            _ => (),
                        }
                    }
                }
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
