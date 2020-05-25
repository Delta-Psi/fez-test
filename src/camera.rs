use cgmath::Matrix4;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CameraPosition {
    N, E, S, W,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CameraDirection {
    L, R,
}

const CAMERA_ROTATION_PERIOD: f32 = 0.5;

pub struct Camera {
    position: CameraPosition,
    direction: Option<CameraDirection>,
    next_direction: Option<CameraDirection>,
    movement_phase: f32,
    pub zoom: f32, // log2
}

impl Camera {
    pub fn new(position: CameraPosition) -> Camera {
        Camera {
            position,
            direction: None,
            next_direction: None,
            movement_phase: 0.0,
            zoom: 0.0,
        }
    }

    pub fn move_(&mut self, dir: CameraDirection) {
        match self.direction {
            None => self.direction = Some(dir),
            Some(_) => self.next_direction = Some(dir),
        }
    }

    pub fn tick(&mut self, delta: f32) {
        use CameraPosition::*;

        if let Some(dir) = self.direction {
            self.movement_phase += match dir {
                CameraDirection::L => -delta / CAMERA_ROTATION_PERIOD,
                CameraDirection::R =>  delta / CAMERA_ROTATION_PERIOD,
            };

            if self.movement_phase.abs() >= 1.0 {
                self.movement_phase = 0.0;

                self.position = match dir {
                    CameraDirection::L => match self.position {
                        N => E,
                        E => S,
                        S => W,
                        W => N,
                    },
                    CameraDirection::R => match self.position {
                        N => W,
                        W => S,
                        S => E,
                        E => N,
                    },
                };

                self.direction = self.next_direction;
                self.next_direction = None;
            }
        }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let angle = cgmath::Deg(match self.position {
            CameraPosition::S => 0.0,
            CameraPosition::E => 90.0,
            CameraPosition::N => 180.0,
            CameraPosition::W => 270.0,
        } + self.movement_phase*90.0);
        let zoom = self.zoom.exp2();

        let rotate_z = Matrix4::from_angle_z(-angle);
        let rotate_x = Matrix4::from_angle_x(cgmath::Deg(-90.0));
        let zoom = Matrix4::from_scale(zoom);

        zoom * rotate_x * rotate_z
    }
}
