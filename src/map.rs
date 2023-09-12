use std::fs::File;
use std::io::Read;

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

    pub fn load(filename: &str) -> Self {
        let mut file = File::open(filename).unwrap();

        let mut width = [0; 4];
        file.read_exact(&mut width).unwrap();
        let width = u32::from_le_bytes(width) as usize;

        let mut height = [0; 4];
        file.read_exact(&mut height).unwrap();
        let height = u32::from_le_bytes(height) as usize;

        let mut grid = vec![0; height * width];
        file.read_exact(&mut grid).unwrap();

        Self { width, height, grid }
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.grid[x + y * self.height] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> Result<u8, &'static str> {
        if x >= self.width || y >= self.height { return Err("Out of map bounds") }
        Ok(self.grid[x + y * self.height])
    }
}
