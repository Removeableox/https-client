use std::u8;

pub struct SimpleRng {
    state: u64,
} 

impl SimpleRng {
    pub fn new(state: u64) -> Self {
        Self {state}
    }
    
    /// simple linear congruential generator
    ///
    /// X_{n+1} = (A * X_{n} + C) % M
    ///
    /// Where A is the constant multiplier 
    /// Where X_{n} is the current seed (Self.state)
    /// Where C is the constant offset 
    /// Where M is the upper bound 
    pub fn next(&mut self) -> u8 {
        // constant
        const A: u64 = 1664525;
        const C: u64 = 1013904223;
        const M: u64 = u64::MAX;

        self.state = (self.state.wrapping_mul(A).wrapping_add(C)) % M;
        // return the lower 8 bits
        (self.state & 0xFF) as u8 
    }
}
