struct SimpleRng {
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
    /// Where A 
    pub fn next(&mut self) -> u8 {
        // constant
        const A: u64 = 1664525;
         
    }
}
