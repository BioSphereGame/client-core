use logger;
use sdl_gui;

use std::thread;
use std::time::{Duration, Instant};

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
    let mut screen = sdl_gui::Screen::new("BioSphere", 1280, 720);

    logger::log(logger::PREFIX_DEBUG, "Starting Screen update cycle");
    let targetfps = 30;
    let targetframetime = Duration::from_millis(1000 / targetfps);
    'running: loop {
        let start = Instant::now();
        for key in screen.get_all_pressed_buttons() {
            match key.name().as_str() {
                "Escape" => break 'running,
                _ => {}
            }
        }
        screen.clear();
        
        let sizex: i32 = 255;
        let sizey: i32 = 255;
        let mut sprite: Vec<(u8, u8, u8, bool)> = vec!();
        for y in 0..sizex {
            for x in 0..sizex {
                sprite.push((y as u8, x as u8, 0, true));
            }
        }
        screen.render_texture_from_color_vec(&sprite, sizex, sizey, 1);

        // screen.draw_filled_polygon(&[
        //     (10, 20),
        //     (20, 110),
        //     (200, 200),
        //     (110, 10),
        // ], &[255, 0, 0, 255]);
        
        screen.draw();
        let elapsed = start.elapsed();
        if elapsed < targetframetime {
            thread::sleep(targetframetime - elapsed);
        } else {
            logger::log(logger::PREFIX_WARN, format!(
                "Frame took too long to render: {}ms from {}ms max",
                elapsed.as_millis(),
                targetframetime.as_millis(),
            ).as_str());
        }
    }
}
