pub mod timer;
pub mod graphics;
use logger;

macro_rules! log_debug {
    ($s:expr) => {
        logger::log(logger::PREFIX_DEBUG, $s);
    };
}

fn say_hi() {
    log_debug!(format!("Booting {}Core v{}{} up...", logger::COLOR_BOLD_GREEN, env!("CARGO_PKG_VERSION"), logger::COLOR_RESET).as_str());
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
    );
    let size_y = 720 / SCALE;
    let size_x = 1280 / SCALE;

    let font = gfx::ui::font::FontData::new(include_bytes!("../assets/SMB1.ttf").to_vec());
    let mut test_text = gfx::ui::text::RendererText::new(
        0, 0,
        24 / SCALE as u16, 1, 1,
        0xFF_000000,
        font.clone(), "Hello, world!".to_string(),
        size_y, size_x,
    );
    test_text.render();
    let mut test_text_two = gfx::ui::text::RendererText::new(
        2, 0,
        24 / SCALE as u16, 1, 1,
        0xFF_FF00FF,
        font.clone(), "Hello, world 2 !".to_string(),
        size_y, size_x,
    );
    test_text_two.render();

    let mut timer_main = timer::Timer::new();
    log_debug!("Starting main update cycle...");
    while window.is_open() {
        timer_main.start();

        window.draw_rectangle(0, 0, size_y, size_x, 0xFF_FFFFFF);
        test_text.draw(&mut window);
        test_text_two.draw(&mut window);
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
