mod macros;
pub mod gfx;
pub mod resources;
use resources::Resources;

mod camera;
use crate::camera::*;

mod level;
use level::*;
mod player;
use player::*;

use std::time::Instant;

struct Game {
    res: Resources,

    camera: Camera,
    level: Level,
    player: Player,

    last_tick: Instant,
}

impl Game {
    pub fn new(level: Level, player_pos: (f32, f32, f32)) -> Game {
        Game {
            res: Resources::new(),

            camera: Camera::new(CameraPosition::S),
            level,
            player: Player::new(player_pos),

            last_tick: Instant::now(),
        }
    }

    pub fn move_camera(&mut self, dir: CameraDirection) {
        self.camera.move_(dir);
        self.player.snap_to(self.camera.next_position(), &self.level);
    }

    pub fn zoom_camera(&mut self, diff: f32) {
        self.camera.zoom += diff;
    }

    pub fn tick(&mut self) {
        // update timing
        let current_tick = Instant::now();
        let delta = current_tick.duration_since(self.last_tick).as_secs_f32();
        self.last_tick = current_tick;

        self.player.tick(delta, &self.camera, &self.level);

        self.camera.tick(delta);
        self.res.set_camera_matrices(self.camera.view_matrix(), self.camera.inverse_z_rotation_matrix());
    }

    pub fn draw(&self) {
        self.res.clear(self.level.bg_color);

        for platform in &self.level.platforms {
            self.res.draw_platform(
                platform.surface_center.into(),
                platform.surface_dim,
                platform.height,
                platform.color);
        }

        self.res.draw_square(self.player.pos.into(), 1.0, (1.0, 1.0, 1.0));
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

    let platform_color = (0.15, 0.38, 0.34);

    let level = Level {
        bg_color: (0.1, 0.1, 0.1),
        platforms: vec![
            Platform::new((0.0, 0.0, -6.0), (8.0, 8.0), 1.0, platform_color),
            Platform::new((0.0, 0.0, 6.0), (4.0, 4.0), 12.0, platform_color),
            Platform::new((-5.0, -9.0, -3.0), (2.0, 2.0), 1.0, platform_color),
            Platform::new((9.0, -5.0, 0.0), (2.0, 2.0), 1.0, platform_color),
            Platform::new((5.0, 9.0, 3.0), (2.0, 2.0), 1.0, platform_color),
            Platform::new((-9.0, 5.0, 6.0), (2.0, 2.0), 1.0, platform_color),
        ],
    };
    let mut game = Game::new(level, (-3.0, -3.0, -6.0));

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
                use glutin::event::VirtualKeyCode::*;

                if let Some(keycode) = input.virtual_keycode {
                    if input.state == ElementState::Pressed {
                        match keycode {
                            A => game.move_camera(CameraDirection::L),
                            D => game.move_camera(CameraDirection::R),

                            O => game.zoom_camera(-0.125),
                            P => game.zoom_camera(0.125),

                            Left => game.player.movement.press_left(),
                            Right => game.player.movement.press_right(),

                            Z => game.player.jump(),

                            _ => (),
                        }
                    } else if input.state == ElementState::Released {
                        match keycode {
                            Left => game.player.movement.release_left(),
                            Right => game.player.movement.release_right(),

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
