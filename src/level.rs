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
}
