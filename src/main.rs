mod macros;
pub mod gfx;
pub mod resources;
use resources::Resources;

use std::time::Instant;
use cgmath::Matrix4;

#[derive(Copy, Clone, Debug)]
enum CameraState {
    N, E, S, W,
}

#[derive(Copy, Clone, Debug)]
enum CameraDirection {
    L, R,
}

struct Game {
    res: Resources,

    camera_state: CameraState,
    camera_direction: Option<CameraDirection>,
    camera_rotation_phase: f32, // either [0.0, 1.0) or (-1.0, 0.0]

    last_tick: Instant,
}

const CAMERA_ROTATION_PERIOD: f32 = 0.2;

impl Game {
    pub fn new() -> Game {
        Game {
            res: Resources::new(),

            camera_state: CameraState::N,
            camera_direction: None,
            camera_rotation_phase: 0.0,

            last_tick: Instant::now(),
        }
    }

    pub fn rotate_camera(&mut self, dir: CameraDirection) {
        if let None = self.camera_direction {
            self.camera_direction = Some(dir)
        }
    }

    pub fn tick(&mut self) {
        // update timing
        let current_tick = Instant::now();
        let delta = current_tick.duration_since(self.last_tick).as_secs_f32();
        self.last_tick = current_tick;

        self.update_camera(delta);
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            self.res.set_view_matrix(&self.view_matrix());
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }

    fn update_camera(&mut self, delta: f32) {
        use CameraState::*;

        if let Some(dir) = self.camera_direction {
            self.camera_rotation_phase += match dir {
                CameraDirection::L => -delta / CAMERA_ROTATION_PERIOD,
                CameraDirection::R =>  delta / CAMERA_ROTATION_PERIOD,
            };

            if self.camera_rotation_phase.abs() >= 1.0 {
                self.camera_rotation_phase = 0.0;
                self.camera_direction = None;

                self.camera_state = match dir {
                    CameraDirection::L => match self.camera_state {
                        N => E,
                        E => S,
                        S => W,
                        W => N,
                    },
                    CameraDirection::R => match self.camera_state {
                        N => W,
                        W => S,
                        S => E,
                        E => N,
                    },
                };
            }
        }
    }

    fn view_matrix(&self) -> Matrix4<f32> {
        let angle = cgmath::Deg(match self.camera_state {
            CameraState::S => 0.0,
            CameraState::E => 90.0,
            CameraState::N => 180.0,
            CameraState::W => 270.0,
        } + self.camera_rotation_phase*90.0);

        Matrix4::from_angle_x(cgmath::Deg(90.0)) * Matrix4::from_angle_z(angle)
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
                            A => game.rotate_camera(CameraDirection::L),
                            D => game.rotate_camera(CameraDirection::R),

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
