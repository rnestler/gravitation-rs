extern crate sdl2;
extern crate rand;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::{thread, time};

use rand::Rng;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;

use sdl2::gfx::primitives::DrawRenderer;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;
const STAR_COUNT: u32 = 100;

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
    pub width: u32,
    pub height: u32,
    pub stars: Vec<Star>,
}

impl World {
    pub fn new(width: u32, height: u32) -> World {
        let mut rng = rand::thread_rng();
        let mut stars = vec![];
        for _ in 0..STAR_COUNT {
            let (x, y) = rng.gen::<(f64, f64)>();
            let mut star = Star::new(width as f64 * x, height as f64 * y);

            star.speed.x = (star.position.y - (height as f64) / 2.0) / (height as f64 * 50.0);
            star.speed.y = -(star.position.x - (width as f64) / 2.0) / (width as f64 * 50.0);
            stars.push(star);
        }
        World {
            width: width,
            height: height,
            stars: stars,
        }
    }

    pub fn update(&mut self) {
        for i in 0..self.stars.len() {
            for j in i+1..self.stars.len() {
                let dis_x = self.stars[i].position.x - self.stars[j].position.x;
                let dis_y = self.stars[i].position.y - self.stars[j].position.y;
                let dis_2 = dis_x * dis_x + dis_y * dis_y;

                if dis_2 > 3.0 {
                    let dis = dis_2.sqrt();
                    let dis_3 = dis_2 * dis * 1000.0;
                    let speed_x = dis_x / dis_3;
                    let speed_y = dis_y / dis_3;

                    self.stars[i].speed.x -= speed_x;
                    self.stars[i].speed.y -= speed_y;
                    self.stars[j].speed.x += speed_x;
                    self.stars[j].speed.y += speed_y;
                }
                else if dis_2 < 2.5 {
                    let dis = dis_2.sqrt();
                    let dis_3 = dis * 500.0;
                    let speed_x = dis_x / dis_3;
                    let speed_y = dis_y / dis_3;

                    self.stars[i].speed.x += speed_x;
                    self.stars[i].speed.y += speed_y;
                    self.stars[j].speed.x -= speed_x;
                    self.stars[j].speed.y -= speed_y;
                }
                else {
                    /*
                    let speed_x = (self.stars[i].speed.x + self.stars[j].speed.x) * 0.5 * 0.001;
                    let speed_y = (self.stars[i].speed.y + self.stars[j].speed.y) * 0.5 * 0.001;

                    self.stars[i].speed.x = self.stars[i].speed.x * 0.999 + speed_x;
                    self.stars[i].speed.y = self.stars[i].speed.y * 0.999 + speed_y;
                    self.stars[j].speed.x = self.stars[j].speed.x * 0.999 + speed_x;
                    self.stars[j].speed.y = self.stars[j].speed.y * 0.999 + speed_y;
                    */

                    let speed_x = (self.stars[i].speed.x + self.stars[j].speed.x) * 0.5;
                    let speed_y = (self.stars[i].speed.y + self.stars[j].speed.y) * 0.5;
                    self.stars[i].speed.x = speed_x;
                    self.stars[i].speed.y = speed_y;
                    self.stars[j].speed.x = speed_x;
                    self.stars[j].speed.y = speed_y;
                }
            }
        }

        for star in &mut self.stars {
            star.position.x += star.speed.x;
            star.position.y += star.speed.y;
        }
    }
}

fn initialize(renderer: &mut Renderer, world: &Mutex<World>) {
    renderer.set_draw_color(pixels::Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();

    let mut world_lock = world.lock().unwrap();
    *world_lock = World::new(SCREEN_WIDTH, SCREEN_HEIGHT);
}

enum ThreadCommand {
    Reset
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
    let world = Arc::new(Mutex::new(World::new(SCREEN_WIDTH, SCREEN_HEIGHT)));


    initialize(&mut renderer, &world);
    let (tx, rx) = channel();
    let update_world = world.clone();
    thread::spawn(move|| {
        loop {
            let mut world_copy = match rx.try_recv() {
                Ok(ThreadCommand::Reset) => World::new(SCREEN_WIDTH, SCREEN_HEIGHT),
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
        let mut visible_counter = 0; // Number of visible stars

        // lock context
        {
            let world_lock = world.lock().unwrap();
            for star in &world_lock.stars {
                if star.position.x >= 0f64 && star.position.x <= SCREEN_WIDTH as f64 &&
                    star.position.y >= 0f64 && star.position.y <= SCREEN_HEIGHT as f64 {
                        visible_counter += 1;
                }
                renderer.pixel(star.position.x as i16, star.position.y as i16, 0xFFFFFFFFu32).unwrap();
            }
        }
        let threshold = STAR_COUNT / 2;
        println!("Stars visible: {}/{}, Threshold: {}", visible_counter, STAR_COUNT, threshold);
        if visible_counter < threshold {
            println!("Reset world!");
            tx.send(ThreadCommand::Reset).expect("Sending command to worker failed");
        }
        renderer.present();
        thread::sleep(time::Duration::from_millis(15));
    }
}
