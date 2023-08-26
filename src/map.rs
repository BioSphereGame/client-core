use super::tile::*;
use super::logger;
use gfx;
use rand::Rng;

pub struct MapData {
    pub tiles: Vec<TileData>,
    pub tiles_amount: u16,
    pub data: Vec<u16>,
    pub size_y: u16,
    pub size_x: u16,
}
impl MapData {
    pub fn new(size_y: u16, size_x: u16) -> MapData {
        let data = vec![0; (size_y * size_x) as usize];
        return MapData {
            tiles: vec![TileData::new_default_empty(); 1],
            tiles_amount: 1,
            data: data,
            size_y: size_y,
            size_x: size_x,
        }
    }

    pub fn check_data_size(&self) -> bool {
        return self.data.len() == (self.size_y * self.size_x) as usize;
    }

    pub fn set_data_size_y_by_x(&mut self, size_x: u16) {
        if self.data.len() % size_x as usize == 0 {
            self.size_y = (self.data.len() / size_x as usize) as u16;
        } else {
            logger::log(logger::PREFIX_ERROR, "Setting tile size by X, but size of data is not divisible by size of map!");
            std::process::exit(1);
        }
    }

    pub fn set_tiles(&mut self, tiles: Vec<TileData>) {
        self.tiles = tiles;
        self.tiles_amount = self.tiles.len() as u16;
    }

    pub fn fill(&mut self, tile: u16) {
        self.data = vec![tile; (self.size_y * self.size_x) as usize];
    }

    pub fn get_tiles(&self) -> Vec<TileData> {
        return self.tiles.clone();
    }

    pub fn check_tiles_size(&self) -> bool {
        return self.tiles.len() == self.tiles_amount as usize;
    }

    pub fn set_data_from_1d_vec(&mut self, data: Vec<u16>, size_y: u16, size_x: u16) {
        if data.len() == (size_y * size_x) as usize {
            self.data = data;
            self.size_y = size_y;
            self.size_x = size_x;
        } else {
            logger::log(logger::PREFIX_ERROR, "Setting map data, but size of data is not equal to size of map!");
            std::process::exit(1);
        }
    }

    pub fn sed_data_from_2d_vec(&mut self, data: Vec<Vec<u16>>) {
        let mut data_1d: Vec<u16> = vec![0; 0];
        for y in 0..data.len() {
            for x in 0..data[y].len() {
                data_1d.push(data[y][x]);
            }
        }
        let mut last_x_size = 0;
        for y in 0..data.len() {
            if y == 0 {
                last_x_size = data[y].len();
            } else {
                if data[y].len() != last_x_size {
                    logger::log(logger::PREFIX_ERROR, "Setting map data from 2D vector, but sizes of X in different Y are not equal!");
                    std::process::exit(1);
                }
            }
        }
        self.data = data_1d;
        self.size_y = data.len() as u16;
        self.size_x = data[0].len() as u16;
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.data.len() {
                self.data[(i) as usize] = (rng.gen_range(0..self.tiles_amount)) as u16;
        }
    }
}

pub fn draw_map(
    screen: &mut gfx::Screen,
    map: &MapData,
    start_y: u16,
    start_x: u16,
    screen_size_y: u16,
    screen_size_x: u16,
) {
    let tile_size_y: usize = map.get_tiles()[0].size_y as usize;
    let tile_size_x: usize = map.get_tiles()[0].size_x as usize;
    for i in 0..map.data.len() {
        let tile = map.tiles[map.data[i] as usize].clone();
        let map_y = i as u16 / map.size_x;
        let map_x = i as u16 % map.size_x;
        if map_y >= start_y && map_y < start_y + screen_size_y && map_x >= start_x && map_x < start_x + screen_size_x {
            let screen_y = map_y as usize * tile_size_y;
            let screen_x = map_x as usize * tile_size_x;
            screen.draw_sprite(tile.data.as_slice(), tile_size_y, tile_size_x, screen_y, screen_x);
        }
    }
}