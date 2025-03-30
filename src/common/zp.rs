use num_traits::{identities::Zero, One};
use std::{
    cmp::max,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign},
    sync::Mutex,
};

use lazy_static::lazy_static;

#[derive(Clone, Copy, Debug)]
pub struct ZpNumber {
    pub val: u32,
    pub p: u32,
}

lazy_static! {
    static ref BODIES: Mutex<HashMap<u32, Vec<u32>>> = Mutex::new(HashMap::new());
}

const DEFAULT_P: u32 = 2;
impl ZpNumber {
    pub fn new(val: u32, p: u32) -> ZpNumber {
        if !BODIES.lock().unwrap().contains_key(&p) {
            ZpNumber::init_body(p);
        }

        ZpNumber { val: val % p, p }
    }

    fn init_body(p: u32) {
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

        BODIES.lock().unwrap().insert(p, inv);
    }

    pub fn inv(&self) -> ZpNumber {
        let val = BODIES.lock().unwrap()[&self.p][self.val as usize];
        ZpNumber::new(val, self.p)
    }

    pub fn zero() -> ZpNumber {
        ZpNumber::new(0, DEFAULT_P)
    }

    pub fn one() -> ZpNumber {
        ZpNumber::new(1, DEFAULT_P)
    }
}

impl Add for ZpNumber {
    type Output = Self;

    fn add(self, other: ZpNumber) -> ZpNumber {
        let p = max(self.p, other.p);
        ZpNumber::new((self.val + other.val) % p, p)
    }
}

impl AddAssign for ZpNumber {
    fn add_assign(&mut self, other: ZpNumber) {
        *self = *self + other;
    }
}

impl Sub for ZpNumber {
    type Output = Self;

    fn sub(self, other: ZpNumber) -> ZpNumber {
        let p = max(self.p, other.p);
        ZpNumber::new((p + self.val - other.val) % p, p)
    }
}

impl SubAssign for ZpNumber {
    fn sub_assign(&mut self, other: ZpNumber) {
        *self = *self - other;
    }
}

impl Mul for ZpNumber {
    type Output = Self;

    fn mul(self, other: ZpNumber) -> ZpNumber {
        let p = max(self.p, other.p);
        ZpNumber::new((self.val * other.val) % p, p)
    }
}

impl MulAssign for ZpNumber {
    fn mul_assign(&mut self, other: ZpNumber) {
        *self = *self * other;
    }
}

impl Div for ZpNumber {
    type Output = Self;

    fn div(self, other: ZpNumber) -> ZpNumber {
        if other.val == 0 {
            panic!("Division by zero");
        }

        self * other.inv()
    }
}

impl Neg for ZpNumber {
    type Output = Self;

    fn neg(self) -> ZpNumber {
        ZpNumber::new(self.p - self.val, self.p)
    }
}

impl Zero for ZpNumber {
    fn zero() -> Self {
        ZpNumber::new(0, DEFAULT_P)
    }

    fn is_zero(&self) -> bool {
        self.val == 0
    }
}

impl One for ZpNumber {
    fn one() -> Self {
        ZpNumber::new(1, DEFAULT_P)
    }

    fn set_one(&mut self) {
        self.val = 1;
    }

    fn is_one(&self) -> bool {
        self.val == 1
    }
}

impl PartialEq for ZpNumber {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Display for ZpNumber {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}
