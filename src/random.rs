pub struct Xorshift128 {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl Xorshift128 {
    pub fn new() -> Self {
        Xorshift128 {
            x: 123456789,
            y: 362436069,
            z: 521288629,
            w: 88675123,
        }
    }

    pub fn init(init: (u32, u32, u32, u32)) -> Self {
        Xorshift128 {
            x: init.0,
            y: init.1,
            z: init.2,
            w: init.3,
        }
    }

    pub fn next(&mut self) -> u32 {
          let t: u32 = self.x ^ (self.x << 11);
          self.x = self.y;
          self.y = self.z;
          self.z = self.w;
          self.w ^= (self.w >> 19) ^ t ^ (t >> 8);
          self.w
    }
}
