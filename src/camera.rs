use cgmath::Matrix4;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Perspective {
    N, E, S, W,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CameraState {
    Stopped,
    MovingL,
    MovingR,
}

const CAMERA_MOVEMENT_PERIOD: f32 = 0.5;

pub struct Camera {
    perspective: Perspective,
    pub zoom: f32, // log2

    state: CameraState,
    phase: f32,
}

impl Camera {
    pub fn new(perspective: Perspective) -> Camera {
        Camera {
            perspective,
            zoom: 0.0,

            state: CameraState::Stopped,
            phase: 0.0,
        }
    }

    pub fn move_left(&mut self) {
        use Perspective::*;

        if let CameraState::Stopped = self.state {
            self.perspective = match self.perspective {
                S => W,
                W => N,
                N => E,
                E => S,
            };
            self.state = CameraState::MovingL;
            self.phase = -1.0;
        }
    }
    pub fn move_right(&mut self) {
        use Perspective::*;

        if let CameraState::Stopped = self.state {
            self.perspective = match self.perspective {
                S => E,
                W => S,
                N => W,
                E => N,
            };
            self.state = CameraState::MovingR;
            self.phase = 1.0;
        }
    }

    pub fn perspective(&self) -> Perspective {
        self.perspective
    }

    pub fn tick(&mut self, delta: f32) {
        match self.state {
            CameraState::Stopped => (),

            CameraState::MovingL => {
                self.phase += delta / CAMERA_MOVEMENT_PERIOD;

                if self.phase >= 0.0 {
                    self.state = CameraState::Stopped;
                    self.phase = 0.0;
                }
            },

            CameraState::MovingR => {
                self.phase -= delta / CAMERA_MOVEMENT_PERIOD;

                if self.phase <= 0.0 {
                    self.state = CameraState::Stopped;
                    self.phase = 0.0;
                }
            },
        }
    }

    fn angle(&self) -> cgmath::Deg<f32> {
        cgmath::Deg(match self.perspective {
            Perspective::S => 0.0,
            Perspective::W => 90.0,
            Perspective::N => 180.0,
            Perspective::E => 270.0,
        } + self.phase*90.0)
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let rotate_z = Matrix4::from_angle_z(self.angle());
        let rotate_x = Matrix4::from_angle_x(cgmath::Deg(-90.0));
        let zoom = Matrix4::from_scale(self.zoom.exp2());

        zoom * rotate_x * rotate_z
    }

    pub fn inverse_z_rotation_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_angle_z(-self.angle())
    }
}
