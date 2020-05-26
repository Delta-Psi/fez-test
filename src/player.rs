use bitflags::bitflags;

bitflags! {
    pub struct Movement: u8 {
        const PRESSING_LEFT =  0b0001;
        const PRESSING_RIGHT = 0b0010;
        const MOVING_LEFT =    0b0100;
        const MOVING_RIGHT =   0b1000;
    }
}

impl Movement {
    pub fn moving_left(&self) -> bool {
        self.contains(Self::MOVING_LEFT)
    }
    pub fn moving_right(&self) -> bool {
        self.contains(Self::MOVING_RIGHT)
    }

    pub fn press_left(&mut self) {
        self.insert(Self::PRESSING_LEFT);
        self.insert(Self::MOVING_LEFT);
        self.remove(Self::MOVING_RIGHT);
    }
    pub fn release_left(&mut self) {
        self.remove(Self::PRESSING_LEFT);
        self.remove(Self::MOVING_LEFT);

        if self.contains(Self::PRESSING_RIGHT) {
            self.insert(Self::MOVING_RIGHT);
        }
    }

    pub fn press_right(&mut self) {
        self.insert(Self::PRESSING_RIGHT);
        self.insert(Self::MOVING_RIGHT);
        self.remove(Self::MOVING_LEFT);
    }
    pub fn release_right(&mut self) {
        self.remove(Self::PRESSING_RIGHT);
        self.remove(Self::MOVING_RIGHT);

        if self.contains(Self::PRESSING_LEFT) {
            self.insert(Self::MOVING_LEFT);
        }
    }
}

pub const GRAVITY: f32 = 24.0;

pub const MOVE_VEL: f32 = 6.0;
pub const JUMP_VEL: f32 = 14.0;

pub struct Player {
    pub pos: (f32, f32, f32),
    pub z_vel: f32,
    pub on_ground: bool,
    pub movement: Movement,
}

use super::{Camera, Level};

impl Player {
    pub fn new(pos: (f32, f32, f32)) -> Self {
        Self {
            pos,
            z_vel: 0.0,
            on_ground: false,
            movement: Movement::empty(),
        }
    }

    pub fn jump(&mut self) {
        if self.on_ground {
            self.z_vel = JUMP_VEL;
            self.on_ground = false;
        }
    }

    pub fn tick(&mut self, delta: f32, camera: &Camera, level: &Level) {
        if camera.direction().is_some() {
            return;
        }

        use super::CameraPosition::*;

        let mut new_pos = self.pos;
        if self.movement.moving_left() {
            match camera.position() {
                S => new_pos.0 -= MOVE_VEL*delta,
                N => new_pos.0 += MOVE_VEL*delta,

                E => new_pos.1 -= MOVE_VEL*delta,
                W => new_pos.1 += MOVE_VEL*delta,
            };
        } else if self.movement.moving_right() {
            match camera.position() {
                S => new_pos.0 += MOVE_VEL*delta,
                N => new_pos.0 -= MOVE_VEL*delta,

                E => new_pos.1 += MOVE_VEL*delta,
                W => new_pos.1 -= MOVE_VEL*delta,
            };
        }
        
        // TODO: check against x/y collision
        self.pos = new_pos;

        let mut new_z = self.pos.2 + delta * (self.z_vel - delta*0.5*GRAVITY);
        let mut new_z_vel = self.z_vel - delta*GRAVITY;

        // check against z collision when falling
        if new_z_vel < 0.0 {
            let z_lower = self.pos.2.min(new_z);
            let z_upper = self.pos.2.max(new_z);
            for platform in &level.platforms {
                if let Some(z) = platform.intersection(self.pos.0, self.pos.1, z_lower, z_upper) {
                    new_z = z;
                    new_z_vel = 0.0;
                    self.on_ground = true;
                    break;
                }
            }
        }

        self.pos.2 = new_z;
        self.z_vel = new_z_vel;
    }
}
