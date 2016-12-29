extern crate piston_window;
extern crate gravitation;
extern crate rand;
extern crate time;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;

use piston_window::{WindowSettings, PistonWindow, clear, ellipse};
use rand::Rng;

use gravitation::*;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;
const SCREEN_DEEPNESS: u32 = 1000;
const STAR_COUNT: usize = 300;

const REVERSE_GRAVITY: f64 = 50.0;
const STAR_SIZE: f64 = 20.0;

enum ThreadCommand {
    Reset
}

fn make_world() -> World {
    let mut rng = rand::thread_rng();
    let prng_init: (u32, u32, u32, u32) = rng.gen();
    World::new(SCREEN_WIDTH, SCREEN_HEIGHT, SCREEN_DEEPNESS, STAR_COUNT, Some(prng_init), REVERSE_GRAVITY, STAR_SIZE)
}

struct Stats {
    pub cycles_per_s: f64,
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Gravitation", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .exit_on_esc(true).fullscreen(true).build().unwrap();

    let stats = Arc::new(Mutex::new(Stats{cycles_per_s: 0.0}));
    let world = Arc::new(Mutex::new(make_world()));
    let (tx, rx) = channel();
    let update_world = world.clone();
    let stats_clone = stats.clone();
    thread::spawn(move|| {
        loop {
            let start = time::precise_time_s();
            for _ in 0..1000 {
                let mut world_copy = match rx.try_recv() {
                    Ok(ThreadCommand::Reset) => make_world(),
                    _ => update_world.lock().unwrap().clone(),
                };
                world_copy.update();
                *update_world.lock().unwrap() = world_copy;
            }
            let stop = time::precise_time_s();
            stats_clone.lock().unwrap().cycles_per_s = 1000.0 / (stop - start);
        }
    });

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.0; 4], g);
            let world_copy = world.lock().unwrap().clone();
            let mut mean_color = 0.0;
            for star in &world_copy.stars {
                //let color = ((star.speed.x * star.speed.x * star.speed.y * star.speed.y).sqrt() * 1000.0) as f32;
                //let red = (star.speed.x * 10.0) as f32;
                //let green = (star.speed.y * 10.0) as f32;
                let color = 2.0 - (star.position.z * 2.0 / SCREEN_DEEPNESS as f64) as f32;
                ellipse([1.0, 1.0, 1.0, color],
                        [star.position.x, star.position.y, world_copy.star_size * color as f64, world_copy.star_size * color as f64],
                        c.transform, g);
                mean_color += color;
            }
            println!("{}", mean_color / world_copy.stars.len() as f32);

            let visible_counter = world_copy.count_visible();
            let cycles_per_s = stats.lock().unwrap().cycles_per_s;
            println!("Cycles/s: {}", cycles_per_s);
            let threshold = STAR_COUNT / 2;
            if visible_counter < threshold {
                tx.send(ThreadCommand::Reset).expect("Sending command to worker failed");
            }
        });
    }
}
