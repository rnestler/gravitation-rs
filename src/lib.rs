extern crate core;

mod random;

use core::u32;


#[derive(Clone)]
pub struct Dimension {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone)]
pub struct Star {
    pub position: Dimension,
    pub speed: Dimension,
}

impl Star {
    pub fn new(x: f64, y: f64, z: f64) -> Star {
        Star {
            position: Dimension {
                x: x,
                y: y,
                z: z,
            },
            speed: Dimension {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
    }
}

#[derive(Clone)]
pub struct World {
    pub width: u32,
    pub height: u32,
    pub deepness: u32,
    pub reverse_gravity: f64,
    pub star_size: f64,
    pub stars: Vec<Star>,
}

impl World {
    pub fn new(width: u32, height: u32, deepness: u32, star_count: usize,
               prng_init: Option<(u32, u32, u32, u32)>,
               reverse_gravity: f64, star_size: f64) -> World {
        let mut stars = vec![];
        let mut rng = match prng_init {
            Some(init) => random::Xorshift128::init(init),
            None => random::Xorshift128::new(),
        };
        for _ in 0..star_count {
            let x = rng.next() as f64 / u32::MAX as f64;
            let y = rng.next() as f64 / u32::MAX as f64;
            let z = rng.next() as f64 / u32::MAX as f64;
            let mut star = Star::new(width as f64 * x, height as f64 * y, deepness as f64 * z);

            //star.speed.x = (star.position.y - (height as f64) / 2.0) / (height as f64 * 50.0);
            //star.speed.y = -(star.position.x - (width as f64) / 2.0) / (width as f64 * 50.0);
            stars.push(star);
        }
        World {
            width: width,
            height: height,
            deepness: deepness,
            stars: stars,
            reverse_gravity: reverse_gravity,
            star_size: star_size,
        }
    }

    pub fn update(&mut self) {
        let limit_touching = self.star_size * self.star_size;
        for i in 0..self.stars.len() {
            for j in i+1..self.stars.len() {
                let mut star_i = self.stars[i].clone();
                let mut star_j = self.stars[j].clone();

                let dis_x = star_i.position.x - star_j.position.x;
                let dis_y = star_i.position.y - star_j.position.y;
                let dis_z = star_i.position.z - star_j.position.z;
                let dis_2 = dis_x * dis_x + dis_y * dis_y + dis_z * dis_z;

                if dis_2 > limit_touching {
                    let dis = dis_2.sqrt();
                    let dis_3 = dis_2 * dis * self.reverse_gravity;
                    let speed_x = dis_x / dis_3;
                    let speed_y = dis_y / dis_3;
                    let speed_z = dis_z / dis_3;

                    star_i.speed.x -= speed_x;
                    star_i.speed.y -= speed_y;
                    star_i.speed.z -= speed_z;
                    star_j.speed.x += speed_x;
                    star_j.speed.y += speed_y;
                    star_j.speed.z += speed_z;
                }
                else if dis_2 < limit_touching / 2.0 {
                    let dis = dis_2.sqrt();
                    let dis_3 = dis_2 * dis * self.reverse_gravity;// * 2.0;
                    let speed_x = dis_x / dis_3;
                    let speed_y = dis_y / dis_3;
                    let speed_z = dis_z / dis_3;

                    star_i.speed.x += speed_x;
                    star_i.speed.y += speed_y;
                    star_i.speed.z += speed_z;
                    star_j.speed.x -= speed_x;
                    star_j.speed.y -= speed_y;
                    star_i.speed.z -= speed_z;
                }
                else {
                    let filter = 0.999;
                    let speed_x = (star_i.speed.x + star_j.speed.x) * 0.5 * (1.0 - filter);
                    let speed_y = (star_i.speed.y + star_j.speed.y) * 0.5 * (1.0 - filter);
                    let speed_z = (star_i.speed.z + star_j.speed.z) * 0.5 * (1.0 - filter);

                    star_i.speed.x = star_i.speed.x * filter + speed_x;
                    star_i.speed.y = star_i.speed.y * filter + speed_y;
                    star_i.speed.z = star_i.speed.z * filter + speed_z;

                    star_j.speed.x = star_j.speed.x * filter + speed_x;
                    star_j.speed.y = star_j.speed.y * filter + speed_y;
                    star_j.speed.z = star_j.speed.z * filter + speed_z;

                    /*
                    let speed_x = (star_i.speed.x + star_j.speed.x) * 0.5;
                    let speed_y = (star_i.speed.y + star_j.speed.y) * 0.5;
                    let speed_z = (star_i.speed.z + star_j.speed.z) * 0.5;
                    star_i.speed.x = speed_x;
                    star_i.speed.y = speed_y;
                    star_i.speed.z = speed_z;
                    star_j.speed.x = speed_x;
                    star_j.speed.y = speed_y;
                    star_j.speed.z = speed_z;
                    */
                }

                self.stars[i] = star_i;
                self.stars[j] = star_j;
            }
        }

        for star in &mut self.stars {
            star.position.x += star.speed.x;
            star.position.y += star.speed.y;
            star.position.z += star.speed.z;
        }
    }

    pub fn count_visible(&self) -> usize {
        self.stars.iter().filter(|star| {
            star.position.x >= 0f64 && star.position.x <= self.width as f64 &&
                star.position.y >= 0f64 && star.position.y <= self.height as f64
        }).count()
    }
}

