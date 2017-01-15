extern crate bear_lib_terminal;
extern crate env_logger;
extern crate log;
extern crate log_panics;
extern crate rand;
extern crate specs;
extern crate termion;
extern crate time;

mod components;
mod map;
mod screens;
mod systems;

use std::thread;
use std::time::Duration;

use bear_lib_terminal::terminal;

use self::screens::Screen;

const MS: u64 = 1_000_000;
const TICK_TIME: u64 = MS * 25;

fn main() {
    env_logger::init().unwrap();
    log_panics::init();

    let mut world = specs::World::new();
    components::register_all(&mut world);
    let mut planner = specs::Planner::<()>::new(world, 2);

    let mut screen = Box::new(screens::TitleScreen);

    terminal::open("jRogue", 80, 24);
    terminal::refresh();

    let mut elapsed = 0;
    let mut last_tick = time::precise_time_ns();
    let mut avg_frame_time = 0.0;
    let mut frames: u64 = 0;

    'main: loop {
        if let Some(terminal::Event::Close) = terminal::read_event() {
            break 'main;
        }

        let now = time::precise_time_ns();
        let old_tick = last_tick;
        elapsed += now - last_tick;
        last_tick = now;

        while elapsed > TICK_TIME {
            elapsed -= TICK_TIME;
            screen.update(&mut planner);
        }

        terminal::clear(None);
        screen.render(&mut planner);
        terminal::refresh();

        thread::sleep(Duration::from_millis((TICK_TIME - elapsed) / MS));

        let frame_time = time::precise_time_ns() - old_tick;
        avg_frame_time = ((frames as f64 * avg_frame_time) + frame_time as f64) / (frames as f64 + 1.0);
        frames += 1;
    }

    println!("Average frame time: {}", avg_frame_time);
}
