extern crate sdl2;
extern crate gravitation;
extern crate rand;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::{thread, time};

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;
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

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window("Gravity simulation", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .fullscreen()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let mut events = sdl_context.event_pump().unwrap();
    let world = Arc::new(Mutex::new(make_world()));

    // Initialize renderer
    renderer.set_draw_color(pixels::Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();

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

    'main: loop {
        for event in events.poll_iter() {

            match event {

                Event::Quit {..} => break 'main,

                Event::KeyDown {keycode: Some(keycode), ..} => {
                    if keycode == Keycode::Escape {
                        break 'main
                    }
                }

                _ => {
                }
            }
        }

        renderer.set_draw_color(pixels::Color::RGB(0, 0, 0));
        renderer.clear();

        let world_copy = world.lock().unwrap().clone();
        for star in &world_copy.stars {
            let mut color = 255.0 - (star.position.z * 255.0 / SCREEN_DEEPNESS as f64);
            if color < 0.0 {
                color = 0.0;
            } else if color > 255.0 {
                color = 255.0;
            }

            let mut size = (2.0 - (star.position.z * 2.0 / SCREEN_DEEPNESS as f64)) * world_copy.star_size;
            if size < 0.0 {
                size = 0.0;
            } else if size > 65535.0 {
                size = 65535.0;
            }

            renderer.filled_circle(star.position.x as i16, star.position.y as i16, size as i16, (color as u32) << 24 | 0x00FFFFFFu32).unwrap();
        }

        let visible_counter = world_copy.count_visible();
        let threshold = STAR_COUNT / 2;
        if visible_counter < threshold {
            tx.send(ThreadCommand::Reset).expect("Sending command to worker failed");
        }
        renderer.present();
        thread::sleep(time::Duration::from_millis(15));
    }
    //            renderer.pixel(star.position.x as i16, star.position.y as i16, 0xA0_FF_FF_FFu32).unwrap();
}
