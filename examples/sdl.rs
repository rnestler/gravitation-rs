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

    let mut camera_speed = Dimension::new();
    let mut camera_pos = Dimension::new();
    'main: loop {
        for event in events.poll_iter() {

            match event {

                Event::MouseMotion{mousestate, xrel, yrel, ..} => {
                    if mousestate.left() {
                        camera_pos.x += xrel as f64;
                        camera_pos.y += yrel as f64;
                    }
                }

                Event::MouseWheel{y, ..} => {
                    camera_pos.z += y as f64 * 10.0;
                }

                Event::Quit {..} => break 'main,

                Event::KeyDown {keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Escape => break 'main,
                        Keycode::Left => {
                            camera_speed.x += 1.0;
                        }
                        Keycode::Right => {
                            camera_speed.x -= 1.0;
                        }

                        Keycode::Up =>  {
                            camera_speed.y += 1.0;
                        }
                        Keycode::Down =>  {
                            camera_speed.y -= 1.0;
                        }
                        Keycode::Space => {
                            tx.send(ThreadCommand::Reset).unwrap();
                        }
                        _ => (),
                    }
                }

                Event::KeyUp {keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Left => {
                            camera_speed.x *= 0.1;
                        }
                        Keycode::Right => {
                            camera_speed.x *= 0.1;
                        }
                        Keycode::Up =>  {
                            camera_speed.y *= 0.1;
                        }
                        Keycode::Down =>  {
                            camera_speed.y *= 0.1;
                        }

                        _ => (),
                    }
                }

                _ => {
                }
            }
        }

        camera_pos.x += camera_speed.x;
        camera_pos.y += camera_speed.y;

        renderer.set_draw_color(pixels::Color::RGB(0, 0, 0));
        renderer.clear();

        let world_copy = world.lock().unwrap().clone();
        for star in &world_copy.stars {
            let pos_z = star.position.z - camera_pos.z;

            let zoom_factor = pos_z * 2.0 / SCREEN_DEEPNESS as f64;

            let mut size = (2.0 - zoom_factor) * world_copy.star_size;
            let mut alpha = 255.0;
            if zoom_factor > 0.0 {
                alpha /= zoom_factor;
            }
            if alpha < 0.0 {
                alpha = 0.0
            } else if alpha > 255.0 {
                alpha = 255.0
            }

            if size < 1.0 {
                size = 1.0;
            } else if size > 65535.0 {
                size = 65535.0;
            }

            if size < world_copy.star_size * 2.0 {
                let pos_x = world_copy.width_2 + (camera_pos.x + star.position.x) / (zoom_factor);
                let pos_y = world_copy.height_2 + (camera_pos.y + star.position.y) / (zoom_factor);

                if size > 3.0 {
                    renderer.filled_circle(pos_x as i16, pos_y as i16, size as i16, (alpha as u32 / 3) << 24 | 0x00FFFFFFu32).unwrap();
                    size /= 1.5;
                }
                if size > 2.0 {
                    renderer.filled_circle(pos_x as i16, pos_y as i16, size as i16, (alpha as u32 / 2) << 24 | 0x00FFFFFFu32).unwrap();
                    size /= 2.0;
                }
                renderer.filled_circle(pos_x as i16, pos_y as i16, size as i16, (alpha as u32) << 24 | 0x00FFFFFFu32).unwrap();
            }
        }

        let visible_counter = world_copy.count_visible();
        let threshold = STAR_COUNT / 2;
        if visible_counter < threshold {
            tx.send(ThreadCommand::Reset).expect("Sending command to worker failed");
        }
        renderer.present();
        thread::sleep(time::Duration::from_millis(1));
    }
    //            renderer.pixel(star.position.x as i16, star.position.y as i16, 0xA0_FF_FF_FFu32).unwrap();
}
