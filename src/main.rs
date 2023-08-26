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

fn run() {
    const SCALE: usize = 1;
    let mut window = gfx::Screen::new(
        720 / SCALE,
        1280 / SCALE,
        SCALE,
        "BioSphere",
        33,
        gfx::WindowOptionsSettings::new(
            false,
            true,
            true,
            false,
            false,
        ),
        include_bytes!("../assets/UbuntuMono.ttf").to_vec(),
    );
    let size_y = 720 / SCALE;
    let size_x = 1280 / SCALE;

    let mut timer_main = timer::Timer::new();
    while window.is_open() {
        timer_main.start();

        window.draw_rectangle(0, 0, size_y, size_x, 0xFF_000000);
        window.draw_text(
            0,
            0,
            (50.0 / (SCALE as f64 / 1.5)) as usize,
            0xFF_FFFFFF,
            0xFF_AAAAAA,
            "AaBbCcDdEeFfGg!@#$%^&*()_+",
        );

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
