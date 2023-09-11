pub struct Map {
    width: usize,
    height: usize,

    grid: Vec<Vec<u8>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![0; height]; width];
        Self { width, height, grid }
    }
}