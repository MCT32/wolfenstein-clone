pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn to_u32(&self) -> u32 {
        self.blue as u32 | (self.green as u32) << 8 | (self.red as u32) << 16
    }
}