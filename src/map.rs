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
        return MapData {
            tiles: vec![TileData::new_default_empty(); 0],
            tiles_amount: 1,
            data: vec![0, size_y * size_x],
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
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                self.data[(y * self.size_x + x) as usize] = (rng.gen_range(0..self.tiles_amount)) as u16;
            }
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
    let tile_size_y: u16 = map.get_tiles()[0].size_y;
    let tile_size_x: u16 = map.get_tiles()[0].size_x;
    for y in 0..map.size_y {
        for x in 0..map.size_x {
            let tile = map.get_tiles()[map.data[(y * map.size_x + x) as usize] as usize].clone();
            if tile_size_y != tile.size_y || tile_size_x != tile.size_x {
                logger::log(logger::PREFIX_ERROR, "Drawing map, but sizes of tiles are not equal!");
                std::process::exit(1);
            }
            if y >= start_y && y < start_y + screen_size_y && x >= start_x && x < start_x + screen_size_x {
                screen.draw_sprite(
                    &tile.get_as_1d_vec().0.into_boxed_slice(),
                    tile_size_y as usize,
                    tile_size_x as usize,
                    ((y - start_y) * tile_size_y) as usize,
                    ((x - start_x) * tile_size_x) as usize,
                );
            }
        }
    }
}