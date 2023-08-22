use logger;
use sdl_gui;

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
    sdl_gui::say_hi();
}

fn main() {
    say_hi();
    run();
}

fn run() {
    let mut screen = sdl_gui::Screen::new("BioSphere", 800, 600);
    logger::log(logger::PREFIX_DEBUG, "Starting Screen update cycle");
    'running: loop {
        for key in screen.get_all_pressed_buttons() {
            match key.name().as_str() {
                "Escape" => break 'running,
                _ => {}
            }
        }

        screen.clear();
        screen.draw_filled_polygon(&[
            (10, 20),
            (20, 110),
            (200, 200),
            (110, 10),
        ], &[255, 0, 0, 255]);
        screen.draw();
    }
}
