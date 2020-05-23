mod macros;
pub mod gfx;
pub mod resources;
use resources::Resources;

use std::time::Instant;
use cgmath::Matrix4;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum CameraState {
    N, E, S, W,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum CameraDirection {
    L, R,
}

struct Camera {
    state: CameraState,
    direction: Option<CameraDirection>,
    rotate_again: bool,
    rotation_phase: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            state: CameraState::N,
            direction: None,
            rotate_again: false,
            rotation_phase: 0.0,
        }
    }

    pub fn rotate(&mut self, dir: CameraDirection) {
        match self.direction {
            None => self.direction = Some(dir),
            Some(current_dir) => if current_dir == dir {
                self.rotate_again = true;
            },
        }
    }

    fn tick(&mut self, delta: f32) {
        use CameraState::*;

        if let Some(dir) = self.direction {
            self.rotation_phase += match dir {
                CameraDirection::L => -delta / CAMERA_ROTATION_PERIOD,
                CameraDirection::R =>  delta / CAMERA_ROTATION_PERIOD,
            };

            if self.rotation_phase.abs() >= 1.0 {
                self.rotation_phase = 0.0;

                self.state = match dir {
                    CameraDirection::L => match self.state {
                        N => E,
                        E => S,
                        S => W,
                        W => N,
                    },
                    CameraDirection::R => match self.state {
                        N => W,
                        W => S,
                        S => E,
                        E => N,
                    },
                };

                if self.rotate_again {
                    self.rotate_again = false;
                } else {
                    self.direction = None;
                }
            }
        }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let angle = cgmath::Deg(match self.state {
            CameraState::S => 0.0,
            CameraState::E => 90.0,
            CameraState::N => 180.0,
            CameraState::W => 270.0,
        } + self.rotation_phase*90.0);

        Matrix4::from_angle_x(cgmath::Deg(90.0)) * Matrix4::from_angle_z(angle)
    }
}

struct Game {
    res: Resources,

    camera: Camera,

    last_tick: Instant,
}

const CAMERA_ROTATION_PERIOD: f32 = 0.2;

impl Game {
    pub fn new() -> Game {
        Game {
            res: Resources::new(),

            camera: Camera::new(),

            last_tick: Instant::now(),
        }
    }

    pub fn rotate_camera(&mut self, dir: CameraDirection) {
        self.camera.rotate(dir);
    }

    pub fn tick(&mut self) {
        // update timing
        let current_tick = Instant::now();
        let delta = current_tick.duration_since(self.last_tick).as_secs_f32();
        self.last_tick = current_tick;

        self.camera.tick(delta);
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            self.res.set_view_matrix(&self.camera.view_matrix());
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
