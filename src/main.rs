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

const TILE_SIZE: usize = 8;
const TILE_ONE: [u32; TILE_SIZE * TILE_SIZE] = [
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_FF_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_FF_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_FF_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_FF_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
];
const TILE_TWO: [u32; TILE_SIZE * TILE_SIZE] = [
    0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00,
    0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00,
    0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00,
    0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_FF, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_FF_00_FF, 0xFF_00_FF_00,
    0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_FF_00_FF, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00,
    0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00,
    0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00,
    0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_FF_00_FF, 0xFF_00_FF_00, 0xFF_00_FF_00, 0xFF_00_FF_00,
];
const TILES: [[u32; TILE_SIZE * TILE_SIZE]; 2] = [
    TILE_ONE,
    TILE_TWO,
];
const MAP_SIZE_X: usize = 30;
const MAP_SIZE_Y: usize = 20;
const MAP: [u32; MAP_SIZE_Y * MAP_SIZE_X] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

fn run() {
    let mut window = gfx::Screen::new(
        720,
        1280,
        1,
        "BioSphere",
        33,
    );

    let mut timer_main = timer::Timer::new();
    while window.is_open() {
        timer_main.start();

        window.draw_rectangle(0, 0, 160, 240, 0xFF_181818);
        // window.draw_sprite(&TILE_ONE, TILE_SIZE, TILE_SIZE, 10, 12);
        // window.draw_sprite(&TILE_ONE, TILE_SIZE, TILE_SIZE, 20, 12);
        // window.draw_sprite(&TILE_ONE, TILE_SIZE, TILE_SIZE, 30, 12);
        // window.draw_sprite(&TILE_ONE, TILE_SIZE, TILE_SIZE, 30, 12);
        for y in 0..MAP_SIZE_Y {
            for x in 0..MAP_SIZE_X {
                let tile = MAP[y * MAP_SIZE_X + x];
                window.draw_sprite(&TILES[tile as usize], TILE_SIZE, TILE_SIZE, y * TILE_SIZE, x * TILE_SIZE);
            }
        }
        window.redraw();

        timer_main.stop();
        window.add_to_title(format!("{}\\{}ms",
            timer_main.get_time_between_as_micros(),
            window.max_update_time_as_micros,
        ));
        timer_main.wait(window.max_update_time_as_micros);
    }
}
