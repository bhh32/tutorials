pub struct Integers {
    i8_min: i8,
    i8_max: i8,
    i16_min: i16,
    i16_max: i16,
    i32_min: i32,
    i32_max: i32,
    i64_min: i64,
    i64_max: i64,
    i128_min: i128,
    i128_max: i128,
}

impl Default for Integers {
    fn default() -> Self {
        Self {
            i8_min: i8::MIN,
            i8_max: i8::MAX,
            i16_min: i16::MIN,
            i16_max: i16::MAX,
            i32_min: i32::MIN,
            i32_max: i32::MAX,
            i64_min: i64::MIN,
            i64_max: i64::MAX,
            i128_min: i128::MIN,
            i128_max: i128::MAX,
        }
    }
}

impl Integers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_i8(&self) -> (i8, i8) {
        (self.i8_min, self.i8_max)
    }

    pub fn get_i16(&self) -> (i16, i16) {
        (self.i16_min, self.i16_max)
    }

    pub fn get_i32(&self) -> (i32, i32) {
        (self.i32_min, self.i32_max)
    }

    pub fn get_i64(&self) -> (i64, i64) {
        (self.i64_min, self.i64_max)
    }

    pub fn get_i128(&self) -> (i128, i128) {
        (self.i128_min, self.i128_max)
    }
}
