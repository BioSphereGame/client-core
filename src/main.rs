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
        60,
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

    let mut couter = 0;
    let mut plus_button = gfx::ui::button::TextRendererButton::new(
        50,
        175,
        50,
        140,

        "Plus".to_string(),
        35,
        0xFF_181818,
        font.clone(),

        0xFF_E7E7E7,
        0xFF_FFFFFF,
        0xFF_A0A0A0,

        0xFF_C0C0C0,
        3,
        2,
    );
    plus_button.render();

    let mut minus_button = gfx::ui::button::TextRendererButton::new(
        50,
        0,
        50,
        175,

        "Minus".to_string(),
        35,
        0xFF_181818,
        font.clone(),

        0xFF_E7E7E7,
        0xFF_FFFFFF,
        0xFF_A0A0A0,

        0xFF_C0C0C0,
        3,
        2,
    );
    let mut text = gfx::ui::text::Text::new(
        0,
        0,
        50,
        1,
        1,
        0xFF_FFFFFF,
        font.clone(),
        "Counter: ".to_string(),
        size_y,
        size_x,
    );
    text.render();

    minus_button.render();
    let mut timer_main = timer::Timer::new();
    log_debug!("Starting main update cycle...");
    while window.is_open() {
        timer_main.start();

        plus_button.update(&mut window);
        if plus_button.pressed {couter += 1; text.text = format!("Counter: {}", couter); text.render()}
        minus_button.update(&mut window);
        if minus_button.pressed {couter -= 1; text.text = format!("Counter: {}", couter); text.render()}

        window.draw_rectangle(0, 0, size_y, size_x, 0xFF_181818);
        plus_button.draw(&mut window);
        minus_button.draw(&mut window);
        text.draw(&mut window);
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
