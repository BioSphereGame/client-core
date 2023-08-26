pub mod timer;
pub mod map;
pub mod tile;
use logger;
use gfx;

fn say_hi() {
    logger::log(
        logger::PREFIX_DEBUG,
        format!("Booting {}Core v{}{} up...",
            logger::COLOR_BOLD_GREEN,
            env!("CARGO_PKG_VERSION"),
            logger::COLOR_RESET,
        ).as_str()
    );
    logger::say_hi();
    gfx::say_hi();
}

fn main() {
    say_hi();
    run();
}

const SCALE: usize = 4;
const TILE_SIZE: u16 = 8;
const TILE_ONE_DATA: [u32; TILE_SIZE as usize * TILE_SIZE as usize] = [
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_FF_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_FF_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_FF_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_FF_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
];
const TILE_TWO_DATA: [u32; TILE_SIZE as usize * TILE_SIZE as usize] = [
    0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00,
    0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00,
    0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00,
    0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_FF, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_FF_00_FF, 0xFF_00_FF_00,
    0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_FF_00_FF, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00,
    0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00,
    0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00,
    0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_FF_00_FF, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00,
];

fn run() {
    let mut window = gfx::Screen::new(
        720 / SCALE,
        1280 / SCALE,
        SCALE,
        "BioSphere",
        33,
    );

    let tiles = vec![
        tile::tile_from_1d_data(TILE_ONE_DATA.to_vec(), TILE_SIZE, TILE_SIZE),
        tile::tile_from_1d_data(TILE_TWO_DATA.to_vec(), TILE_SIZE, TILE_SIZE),
    ];

    let mut map = map::MapData::new(
        (720.0 / SCALE as f64 / TILE_SIZE as f64).ceil() as u16,
        (1280.0 / SCALE as f64 / TILE_SIZE as f64).ceil() as u16,
    );
    map.set_tiles(tiles);
    map.randomize();

    let mut timer_main = timer::Timer::new();
    while window.is_open() {
        timer_main.start();

        map::draw_map(&mut window, &map, 0, 0, 720 / 4, 1280 / 4);
        window.redraw();

        timer_main.stop();
        let max_update_time: f32 = window.max_update_time_as_micros as f32 / 1000.0;
        let work_time: f32 = timer_main.get_time_between_as_float_millis() as f32;
        window.add_to_title(format!("{} \\ {}ms ({}%)",
            timer_main.get_time_between_as_float_millis(),
            max_update_time,
            (work_time * 100.0 / max_update_time) as u8,
        ));
        timer_main.wait(window.max_update_time_as_micros);
    }
}
