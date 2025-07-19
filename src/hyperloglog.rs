use std::cmp::max;

use num_traits::pow;

use crate::hyperloglog::hashing::{Hashing, PolRolHF};

mod hashing;
pub struct HyperLogLog {
    best:u8,
    hashing_function: PolRolHF,
}

impl HyperLogLog {
    pub fn new (function: String) -> HyperLogLog {
        HyperLogLog {hashing_function: PolRolHF::new(31, 9 + pow(10, 9)), best: 0}
    }
    fn get_luckiness(&self, mut number: u128)-> u8{

        let mut result: u8 = 0;
        while number % 2 == 0 {
            result += 1;
            number /= 2;
        }
        return result;
    }
    pub fn receive(&mut self, word: String){
        let hashed_value = self.hashing_function.hash(word);
        let luckiness = self.get_luckiness(hashed_value);
        if luckiness > self.best {
            self.best = luckiness;
        }
        // println!("Luckiness: {}", luckiness);
    }

    pub fn get_single_words(&self)-> u128{
        return pow(2, self.best as usize + 1);
    }
}
