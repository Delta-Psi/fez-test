pub struct Level {
    pub bg_color: (f32, f32, f32),
    pub platforms: Vec<Platform>,
}

pub struct Platform {
    pub surface_center: (f32, f32, f32),
    pub surface_dim: (f32, f32),
    pub height: f32,
    pub color: (f32, f32, f32),
}

impl Platform {
    pub fn new(surface_center: (f32, f32, f32), surface_dim: (f32, f32), height: f32, color: (f32, f32, f32)) -> Self {
        Self {
            surface_center,
            surface_dim,
            height,
            color,
        }
    }

    pub fn intersection(&self, x: f32, y: f32, z_lower: f32, z_upper: f32) -> Option<f32> {
        if (x - self.surface_center.0).abs() > 0.5*self.surface_dim.0 {
            None
        } else if (y - self.surface_center.1).abs() > 0.5*self.surface_dim.1 {
            None
        } else if z_lower <= self.surface_center.2 && self.surface_center.2 <= z_upper {
            Some(self.surface_center.2)
        } else {
            None
        }
    }
}
