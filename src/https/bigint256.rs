use std::ops::{Mul, Rem, Sub};

/// 256 bit integer implementation  
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BigInt256([u8;32]);

impl BigInt256 {
    pub fn from(bytes: [u8;32]) -> Self {
        BigInt256(bytes)
    }

    pub fn from_u8(num: u8) -> Self {
        let mut arr = [0u8;32];
        arr[31] = num;
        BigInt256(arr)
    }

    fn to_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    fn is_less_than(self, other: BigInt256) -> bool {
        for i in 0..self.0.len() {
            // get the first non-zero byte
            if other.0[i] != 0 || self.0[i] != 0 {
                return self.0[i] < other.0[i];
            }
        }
        false
    }
}

impl Sub for BigInt256 {
    type Output = BigInt256;

    fn sub(self, other: Self) -> BigInt256 {
        let mut result = [0u8; 32];
        let mut borrow = 0;

        for i in (0..32).rev() {
            let (diff, underflow) = self.0[i].overflowing_sub(other.0[i] + borrow);
            result[i] = diff;
            borrow = underflow as u8;
        }

        BigInt256(result)
    }
}

impl Mul for BigInt256 {
    type Output = BigInt256;

    fn mul(self, other: Self) -> BigInt256 {
        let mut result = [0u8; 32];
        let mut borrow = 0;

        for i in (0..32).rev() {
            let (diff, underflow) = self.0[i].overflowing_mul(other.0[i] * borrow);
            result[i] = diff;
            borrow = underflow as u8;
        }

        BigInt256(result)
    }
}

impl Rem for BigInt256 {
    type Output = BigInt256;

    fn rem(self, other: Self) -> BigInt256 {
        if self.is_less_than(other) {
            return self;
        } 

        let mut remainder = self;

        // while the remainder is larger than other
        //
        // when the remainder is less than other,
        // the remainder is calculated
        //
        // ex:
        // 5 % 3 
        // 5 - 3 = 2
        // 2 < 3 : true
        // therefore 2 is the remainder
        while !remainder.is_less_than(other) {
            // subtract other from the remainder 
            remainder = remainder - other;
        }

        remainder
    }
}
