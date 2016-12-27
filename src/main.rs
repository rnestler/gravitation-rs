extern crate sdl2;
extern crate rand;

use std::sync::{Arc, Mutex};
use std::{thread, time};

use rand::Rng;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;

use sdl2::gfx::primitives::DrawRenderer;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;

#[derive(Clone)]
struct Dimension {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone)]
struct Star {
    pub position: Dimension,
    pub speed: Dimension,
}

impl Star {
    pub fn new(x: f64, y: f64) -> Star {
        Star {
            position: Dimension {
                x: x,
                y: y,
            },
            speed: Dimension {
                x: 0.0,
                y: 0.0,
            }
        }
    }
}

#[derive(Clone)]
struct World {
    pub stars: Vec<Star>,
}

impl World {
    pub fn new() -> World {
        let mut rng = rand::thread_rng();
        let mut stars = vec![];
        for _ in 0..300 {
            let (x, y) = rng.gen::<(f64, f64)>();
            stars.push(Star::new(SCREEN_WIDTH as f64 * x, SCREEN_HEIGHT as f64 * y));
        }
        World {
            stars: stars
        }
    }

    pub fn update(&mut self) {
        for i in 0..self.stars.len() {
            for j in i+1..self.stars.len() {
                let dis_x = self.stars[i].position.x - self.stars[j].position.x;
                let dis_y = self.stars[i].position.y - self.stars[j].position.y;
                let dis_2 = dis_x * dis_x + dis_y * dis_y;

                if dis_2 > 8.0 {
                    let dis = dis_2.sqrt();
                    let dis_3 = dis_2 * dis * 1000.0;
                    let speed_x = dis_x / dis_3;
                    let speed_y = dis_y / dis_3;

                    self.stars[i].speed.x -= speed_x;
                    self.stars[i].speed.y -= speed_y;
                    self.stars[j].speed.x += speed_x;
                    self.stars[j].speed.y += speed_y;
                } else {
                    let speed_x = (self.stars[i].speed.x + self.stars[j].speed.x) * 0.5;
                    let speed_y = (self.stars[i].speed.y + self.stars[j].speed.y) * 0.5;
                    
                    self.stars[i].speed.x = speed_x;
                    self.stars[i].speed.y = speed_y;
                    self.stars[j].speed.x = speed_x;
                    self.stars[j].speed.y = speed_y;
                }
            }
        }

        for i in 0..self.stars.len() {
            self.stars[i].position.x += self.stars[i].speed.x;
            self.stars[i].position.y += self.stars[i].speed.y;
        }
    }
}

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window("rust-sdl2_gfx: draw line & FPSManager", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .fullscreen()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_draw_color(pixels::Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();

    let mut events = sdl_context.event_pump().unwrap();

    let world = Arc::new(Mutex::new(World::new()));

    let update_world = world.clone();
    thread::spawn(move|| {
        loop {
            let mut world_copy = update_world.lock().unwrap().clone();
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
        {
            let world_lock = world.lock().unwrap();
            for star in &world_lock.stars {
                renderer.pixel(star.position.x as i16, star.position.y as i16, 0xFFFFFFFFu32).unwrap();
            }
        }
        renderer.present();
        thread::sleep(time::Duration::from_millis(15));

    }
}
