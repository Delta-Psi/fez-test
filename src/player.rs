pub struct Player {
    pub pos: (f32, f32, f32),
}

impl Player {
    pub fn new(pos: (f32, f32, f32)) -> Self {
        Self {
            pos,
        }
    }
}
