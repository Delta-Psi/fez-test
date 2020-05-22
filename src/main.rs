use std::time::Instant;

struct Game {
    last_tick: Instant,
}

impl Game {
    pub fn new() -> Game {
        Game {
            last_tick: Instant::now(),
        }
    }

    pub fn tick(&mut self) {
        // update timing
        let current_tick = Instant::now();
        let delta = current_tick.duration_since(self.last_tick);
        self.last_tick = current_tick;

        println!("{} FPS", 1.0/delta.as_secs_f64());
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let context = glutin::ContextBuilder::new()
        .build_windowed(glutin::window::WindowBuilder::new()
            .with_title("fez test")
            .with_inner_size(glutin::dpi::LogicalSize::new(640.0, 480.0)), &event_loop)
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

            Event::RedrawRequested(_) => {
                // etc
                game.tick();
                game.draw();

                context.swap_buffers().unwrap();
            },

            _ => (),
        };
    });
}
