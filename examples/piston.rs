extern crate piston_window;
extern crate gravitation;
extern crate rand;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::{thread, time};

use piston_window::*;
use rand::Rng;

use gravitation::*;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;
const STAR_COUNT: u32 = 300;

enum ThreadCommand {
    Reset
}

fn make_world() -> World {
    let mut rng = rand::thread_rng();
    let prng_init: (u32, u32, u32, u32) = rng.gen();
    World::new(SCREEN_WIDTH, SCREEN_HEIGHT, STAR_COUNT, Some(prng_init))
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Gravitation", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .exit_on_esc(true).fullscreen(true).build().unwrap();

    let world = Arc::new(Mutex::new(make_world()));
    let (tx, rx) = channel();
    let update_world = world.clone();
    thread::spawn(move|| {
        loop {
            let mut world_copy = match rx.try_recv() {
                Ok(ThreadCommand::Reset) => make_world(),
                _ => update_world.lock().unwrap().clone(),
            };
            world_copy.update();
            *update_world.lock().unwrap() = world_copy;
        }
    });

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.0; 4], g);
            let mut visible_counter = 0; // Number of visible stars
            {
                let world_lock = world.lock().unwrap();
                for star in &world_lock.stars {
                    if star.position.x >= 0f64 && star.position.x <= SCREEN_WIDTH as f64 &&
                        star.position.y >= 0f64 && star.position.y <= SCREEN_HEIGHT as f64 {
                            visible_counter += 1;
                        }
                    let size = 5.0;
                    ellipse([1.0, 1.0, 1.0, 1.0],
                            [star.position.x, star.position.y, size, size],
                            c.transform, g);
                }
            }
            let threshold = STAR_COUNT / 2;
            if visible_counter < threshold {
                tx.send(ThreadCommand::Reset).expect("Sending command to worker failed");
            }
        });
    }
}
