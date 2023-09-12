pub struct Map {
    pub width: usize,
    pub height: usize,

    grid: Vec<u8>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![0; height * width];
        Self { width, height, grid }
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.grid[x + y * self.height] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> Result<u8, &'static str> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height { return Err("Out of map bounds") }
        Ok(self.grid[x + y * self.height])
    }
}
