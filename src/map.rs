use super::tile::*;
use super::logger;
use gfx;

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
}

pub fn clip_map(map: &MapData, tiles_y: i16, tiles_x: i16, pos_y: i16, pos_x: i16) -> MapData {
    let tiles_up = tiles_y / 2;
    let tiles_down = tiles_y - tiles_up;
    let tiles_left = tiles_x / 2;
    let tiles_right = tiles_x - tiles_left;

    let mut clipped_y_size = tiles_up + tiles_down;
    let mut clipped_x_size = tiles_left + tiles_right;

    if pos_y - tiles_up < 0 {
        clipped_y_size += pos_y - tiles_up;
    }
    if pos_y + tiles_down >= map.size_y as i16 {
        clipped_y_size += pos_y + tiles_down - map.size_y as i16;
    }
    if pos_x - tiles_left < 0 {
        clipped_x_size += pos_x - tiles_left;
    }
    if pos_x + tiles_right >= map.size_x as i16 {
        clipped_x_size += pos_x + tiles_right - map.size_x as i16;
    }

    let clipped_up = if pos_y - tiles_up < 0 { 0 } else { pos_y - tiles_up };
    let clipped_down = if pos_y + tiles_down >= map.size_y as i16 { map.size_y as i16 } else { pos_y + tiles_down };
    let clipped_left = if pos_x - tiles_left < 0 { 0 } else { pos_x - tiles_left };
    let clipped_right = if pos_x + tiles_right >= map.size_x as i16 { map.size_x as i16 } else { pos_x + tiles_right };

    let mut new_map = MapData::new(tiles_y as u16, tiles_x as u16);
    let mut new_data: Vec<u16> = vec![0; 0];
    for y in pos_y - clipped_up..pos_y + clipped_down {
        for x in pos_x - clipped_left..pos_x + clipped_right {
            new_data.push(map.data[(y * map.size_x as i16 + x) as usize]);
        }
    }
    new_map.set_data_from_1d_vec(new_data, clipped_y_size as u16, clipped_x_size as u16);
    return new_map;
}

pub fn draw_map(
    screen: &mut gfx::Screen,
    map: &MapData,
    screen_pos_y: u16,
    screen_pos_x: u16,
) {

}