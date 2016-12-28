extern crate core;

mod random;

use core::u32;


#[derive(Clone)]
pub struct Dimension {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone)]
pub struct Star {
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
pub struct World {
    pub width: u32,
    pub height: u32,
    pub stars: Vec<Star>,
}

impl World {
    pub fn new(width: u32, height: u32, star_count: usize,
               prng_init: Option<(u32, u32, u32, u32)>) -> World {
        let mut stars = vec![];
        let mut rng = match prng_init {
            Some(init) => random::Xorshift128::init(init),
            None => random::Xorshift128::new(),
        };
        for _ in 0..star_count {
            let x = rng.next() as f64 / u32::MAX as f64;
            let y = rng.next() as f64 / u32::MAX as f64;
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
                let mut star_i = self.stars[i].clone();
                let mut star_j = self.stars[j].clone();

                let dis_x = star_i.position.x - star_j.position.x;
                let dis_y = star_i.position.y - star_j.position.y;
                let dis_2 = dis_x * dis_x + dis_y * dis_y;

                if dis_2 > 3.0 {
                    let dis = dis_2.sqrt();
                    let dis_3 = dis_2 * dis * 1000.0;
                    let speed_x = dis_x / dis_3;
                    let speed_y = dis_y / dis_3;

                    star_i.speed.x -= speed_x;
                    star_i.speed.y -= speed_y;
                    star_j.speed.x += speed_x;
                    star_j.speed.y += speed_y;
                }
                else if dis_2 < 2.5 {
                    let dis = dis_2.sqrt();
                    let dis_3 = dis * 500.0;
                    let speed_x = dis_x / dis_3;
                    let speed_y = dis_y / dis_3;

                    star_i.speed.x += speed_x;
                    star_i.speed.y += speed_y;
                    star_j.speed.x -= speed_x;
                    star_j.speed.y -= speed_y;
                }
                else {
                    /*
                    let speed_x = (star_i.speed.x + star_j.speed.x) * 0.5 * 0.001;
                    let speed_y = (star_i.speed.y + star_j.speed.y) * 0.5 * 0.001;

                    star_i.speed.x = star_i.speed.x * 0.999 + speed_x;
                    star_i.speed.y = star_i.speed.y * 0.999 + speed_y;
                    star_j.speed.x = star_j.speed.x * 0.999 + speed_x;
                    star_j.speed.y = star_j.speed.y * 0.999 + speed_y;
                    */

                    let speed_x = (star_i.speed.x + star_j.speed.x) * 0.5;
                    let speed_y = (star_i.speed.y + star_j.speed.y) * 0.5;
                    star_i.speed.x = speed_x;
                    star_i.speed.y = speed_y;
                    star_j.speed.x = speed_x;
                    star_j.speed.y = speed_y;
                }

                self.stars[i] = star_i;
                self.stars[j] = star_j;
            }
        }

        for star in &mut self.stars {
            star.position.x += star.speed.x;
            star.position.y += star.speed.y;
        }
    }

    pub fn count_visible(&self) -> usize {
        self.stars.iter().filter(|star| {
            star.position.x >= 0f64 && star.position.x <= self.width as f64 &&
                star.position.y >= 0f64 && star.position.y <= self.height as f64
        }).count()
    }
}

