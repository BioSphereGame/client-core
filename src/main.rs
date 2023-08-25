pub mod timer;
use logger;
use gfx;
use std::thread;

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
const TEST_TILE: [u32; TILE_SIZE * TILE_SIZE] = [
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_FF_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_FF_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_FF_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_FF_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
];

fn run() {
    let mut window = gfx::Screen::new(
        160,
        240,
        5,
        "BioSphere",
        30,
    );

    let mut timer_main = timer::Timer::new();
    while window.is_open() {
        timer_main.start();

        window.draw_rectangle(0, 0, 160, 240, 0xFF_181818);
        window.draw_sprite(&TEST_TILE, TILE_SIZE, TILE_SIZE, 10, 12);
        window.redraw();

        timer_main.stop();
        window.add_to_title(format!(" - {}\\{}ms",
            timer_main.get_time_between_as_micros(),
            window.max_update_time_as_micros,
        ));
        timer_main.wait(window.max_update_time_as_micros);
    }
}
