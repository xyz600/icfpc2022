struct XorShift {
    state: [u64; 2],
}

impl XorShift {
    fn new(seed: u64) -> Self {
        let mut ret = XorShift { state: [seed, seed * 195 + 1] };
        for _ in 0..128 {
            ret.next();
        }
        ret
    }
    fn next(&mut self) -> u64 {
        let mut s1 = self.state[0];
        let s0 = self.state[1];
        self.state[0] = s0;
        s1 ^= s1 << 23;
        s1 ^= s1 >> 17;
        s1 ^= s0;
        s1 ^= s0 >> 26;
        self.state[1] = s1;

        // avoid overflow for debug build
        if let Some(v) = s0.checked_add(s1) {
            v
        } else {
            s0 - (std::u64::MAX - s1) - 1
        }
    }

    pub fn next_float(&mut self) -> f64 {
        return (self.next() as f64) / (std::u64::MAX as f64);
    }
}

pub struct CachedRandom {
    int_table: Vec<u32>,
    uniform_table: Vec<f64>,
    log_table: Vec<f64>,
    index: usize,
}

impl CachedRandom {
    pub fn new(size: usize, seed: u64) -> CachedRandom {
        let mut ret = CachedRandom {
            int_table: vec![],
            uniform_table: vec![],
            log_table: vec![],
            index: 0,
        };

        let mut rand = XorShift::new(seed);

        for _ in 0..size {
            let val = (rand.next() >> 32) as u32;
            ret.int_table.push(val);

            let fval = (val as f64) / (std::u32::MAX as f64);
            ret.uniform_table.push(fval);

            // add eps to avoid log(0)
            let log_fval = (fval + 1e-20).log(std::f64::consts::E);
            ret.log_table.push(log_fval);
        }

        ret
    }

    pub fn next_int(&mut self) -> u32 {
        let ret = self.int_table[self.index];
        self.update();
        ret
    }

    // FIXME: 高速化
    pub fn next_int64(&mut self) -> u64 {
        let v1 = self.next_int();
        let v2 = self.next_int();
        ((v1 as u64) << 32) + (v2 as u64)
    }

    pub fn next_int_range(&mut self, left: u32, right: u32) -> u32 {
        (((right - left) as u64) * self.next_int() as u64 >> 32) as u32 + left
    }

    pub fn next_float(&mut self) -> f64 {
        let ret = self.uniform_table[self.index];
        self.update();
        ret
    }

    pub fn next_float_range(&mut self, left: f64, right: f64) -> f64 {
        self.next_float() * (right - left) + left
    }

    pub fn next_log_float(&mut self) -> f64 {
        let ret = self.log_table[self.index];
        self.update();
        ret
    }

    fn len(&self) -> usize {
        self.int_table.len()
    }

    fn update(&mut self) {
        self.index += 1;
        if self.index == self.len() {
            self.index = 0;
        }
    }
}
