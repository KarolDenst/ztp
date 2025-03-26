pub struct Zp {
    p: u32,
    inv: Vec<u32>,
}

impl Zp {
    pub fn new(p: u32) -> Zp {
        let mut inv = vec![0; p as usize];

        inv[1] = 1;

        for i in 2..p {
            let mut result = 1;
            let mut base = i;
            let mut exponent = p - 2;

            while exponent > 0 {
                if exponent % 2 == 1 {
                    result = (result as u64 * base as u64 % p as u64) as u32;
                }
                base = (base as u64 * base as u64 % p as u64) as u32;
                exponent /= 2;
            }

            inv[i as usize] = result;
        }

        Zp { p, inv }
    }

    pub fn add(&self, a: u32, b: u32) -> u32 {
        (a + b) % self.p
    }

    pub fn sub(&self, a: u32, b: u32) -> u32 {
        (a + self.p - b) % self.p
    }

    pub fn mul(&self, a: u32, b: u32) -> u32 {
        (a * b) % self.p
    }

    pub fn div(&self, a: u32, b: u32) -> u32 {
        self.mul(a, self.inv[b as usize])
    }

    pub fn inv(&self, a: u32) -> u32 {
        self.inv[a as usize]
    }

    pub fn fix(&self, a: &mut u32) {
        *a %= self.p;
    }

    pub fn neg(&self, a: u32) -> u32 {
        self.sub(0, a)
    }
}
