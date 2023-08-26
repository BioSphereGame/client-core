use super::logger;

#[derive(Clone)]
pub struct TileData {
    pub data: Vec<u32>,
    pub size_y: u16,
    pub size_x: u16,
}
impl TileData {
    pub fn new(size_y: u16, size_x: u16) -> TileData {
        return TileData {
            data: vec![0xFF_000000, (size_y * size_x) as u32],
            size_y: size_y,
            size_x: size_x,
        }
    }

    pub fn new_default_empty() -> TileData {
        return TileData {
            data: vec![0xFF_000000, 0],
            size_y: 0,
            size_x: 0,
        }
    }

    pub fn check_size(&self) -> bool {
        return self.data.len() == (self.size_y * self.size_x) as usize;
    }

    pub fn set_size_y_by_x(&mut self, size_x: u16) {
        if self.data.len() % size_x as usize == 0 {
            self.size_y = (self.data.len() / size_x as usize) as u16;
        } else {
            logger::log(logger::PREFIX_ERROR, "Setting tile size by X, but size of data is not divisible by size of tile!");
            std::process::exit(1);
        }
    }

    pub fn set_from_vec(&mut self, data: Vec<u32>, size_y: u16, size_x: u16) {
        if data.len() == (size_y * size_x) as usize {
            self.data = data;
            self.size_y = size_y;
            self.size_x = size_x;
        } else {
            logger::log(logger::PREFIX_ERROR, "Setting tile data, but size of data is not equal to size of tile!");
            std::process::exit(1);
        }
    }

    pub fn set_from_png(&mut self, _data: Vec<u32>, _size_y: u16, _size_x: u16) {
        logger::log(logger::PREFIX_ERROR, "Setting tile data from PNG is not implemented yet!");
        std::process::exit(1);
    }

    pub fn get_as_1d_vec(&self) -> (Vec<u32>, u16) {
        return (self.data.clone(), self.size_x);
    }
    
    pub fn get_as_2d_vec(&self) -> Vec<Vec<u32>> {
        let mut data: Vec<Vec<u32>> = vec![vec![0; self.size_x as usize]; self.size_y as usize];
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                data[y as usize][x as usize] = self.data[(y * self.size_x + x) as usize];
            }
        }
        return data;
    }
}

pub fn tile_from_1d_data(data: Vec<u32>, size_y: u16, size_x: u16) -> TileData {
    let mut tile = TileData::new(size_y, size_x);
    tile.set_from_vec(data, size_y, size_x);
    return tile;
}

pub fn tile_from_2d_data(data: Vec<Vec<u32>>) -> TileData {
    let mut tile = TileData::new(data.len() as u16, data[0].len() as u16);
    tile.set_from_vec(
        data.iter().flatten().cloned().collect(),
        data.len() as u16,
        data[0].len() as u16,
    );
    return tile;
}
