pub struct BallCount {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl BallCount {
    pub fn new(r: u32, g: u32, b: u32) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
        }
    }
}
