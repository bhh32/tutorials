pub struct FloatingPoints {
    f32_min: f32,
    f32_max: f32,
    f64_min: f64,
    f64_max: f64,
}

impl Default for FloatingPoints {
    fn default() -> Self {
        Self {
            f32_min: f32::MIN,
            f32_max: f32::MAX,
            f64_min: f64::MIN,
            f64_max: f64::MAX,
        }
    }
}

impl FloatingPoints {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_f32(&self) -> (f32, f32) {
        (self.f32_min, self.f32_max)
    }

    pub fn get_f64(&self) -> (f64, f64) {
        (self.f64_min, self.f64_max)
    }
}
